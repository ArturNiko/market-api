use dotenv::dotenv;
use std::env;
use tokio::task::JoinHandle;
use tokio_postgres::{Client, Error, NoTls, Row, Statement };
use tokio_postgres::types::ToSql;

#[allow(dead_code)]
pub struct DB {
    client: Client,
    handle: JoinHandle<()>,
}

#[allow(dead_code)]
pub struct QueryBuilder<'a> {
    connection: &'a DB,
    table: String,
    columns: Option<Vec<String>>,
    values: Option<Vec<Box<dyn ToSql + Sync>>>,
    conditions: Option<Vec<String>>,
    logical_operator: Option<String>,
    limit: Option<usize>,
}

#[allow(dead_code)]
impl DB {
    pub async fn new() -> Result<Self, Error> {
        dotenv().ok();

        let postgres_user = env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
        let postgres_password =
            env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
        let postgres_db = env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");

        let connection_string = format!(
            "host=localhost user={} password={} dbname={} port=5432",
            postgres_user, postgres_password, postgres_db
        );

        let (client, connection) = tokio_postgres::connect(&connection_string, NoTls).await?;

        let handle: JoinHandle<()> = tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });


        Ok(Self {
            client,
            handle,
        })
    }

    /// Starts a new query builder
    pub fn table(&self, table: &str) -> QueryBuilder {
        QueryBuilder {
            connection: self,
            table: table.to_string(),
            columns: None,
            values: None,
            conditions: None,
            logical_operator: None,
            limit: None,
        }
    }
}

#[allow(dead_code)]
impl<'a> QueryBuilder<'a> {
    pub fn columns(mut self, columns: Vec<&str>) -> Self {
        self.columns = Some(columns.into_iter().map(|col| col.to_string()).collect());
        self
    }
    pub fn values(mut self, values: Vec<Box<dyn ToSql + Sync>>) -> Self {
        self.values = Some(values);
        self
    }

    pub fn where_condition(mut self, condition: &str) -> Self {
        if self.conditions.is_none() {
            self.conditions = Some(Vec::new());
        }
        self.conditions.as_mut().unwrap().push(condition.to_string());
        self
    }

    pub fn and_condition(mut self, condition: &str) -> Self {
        self.logical_operator = Some("AND".to_string());
        self = self.where_condition(condition); // Add the condition to the list
        self
    }

    pub fn or_condition(mut self, condition: &str) -> Self {
        self.logical_operator = Some("OR".to_string());
        self = self.where_condition(condition); // Add the condition to the list
        self
    }

    fn build_conditions(&self) -> Option<String> {
        if let Some(conds) = &self.conditions {
            if conds.is_empty() {
                return None;
            }
            let operator = self.logical_operator.as_deref().unwrap_or("AND");
            Some(conds.join(&format!(" {} ", operator)))
        } else {
            None
        }
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub async fn insert(self) -> Result<Row, Error> {
        let columns: String = self.columns.as_ref().unwrap().join(", ");
        let values: &Vec<Box<dyn ToSql + Sync>> = self.values.as_ref().unwrap();

        let params: Vec<&(dyn ToSql + Sync)> = values.iter().map(|val| val.as_ref() as _).collect();

        let query = format!(
            "INSERT INTO {} ({}) VALUES ({}) RETURNING *",
            self.table, columns, self.build_placeholders(Command::Insert)
        );

        let statement: Statement = self.connection.client.prepare(&query).await?;
        let row: Row = self.connection.client.query_one(&statement, &params).await?;

        Ok(row)
    }

    pub async fn update(self) -> Result<u64, Error> {
        let values: &Vec<Box<dyn ToSql + Sync>> = self.values.as_ref().unwrap();

        let params: Vec<&(dyn ToSql + Sync)> = values.iter().map(|val| val.as_ref() as _).collect();

        let mut query = format!("UPDATE {} SET {}", self.table, self.build_placeholders(Command::Update));

        if let Some(condition) = self.build_conditions() {
            query.push_str(&format!(" WHERE {}", condition));
        }

        let statement = self.connection.client.prepare(&query).await?;
        let rows_affected = self.connection.client.execute(&statement, &params).await?;

        Ok(rows_affected)
    }

    // implement placeholders
    pub async fn select(self) -> Result<Vec<Row>, Error> {
        let columns: String = self
            .columns
            .as_ref()
            .map(|cols| cols.join(", "))
            .unwrap_or_else(|| "*".to_string());

        let mut query = format!("SELECT {} FROM {}", columns, self.table);

        if let Some(condition) = self.build_conditions() {
            query.push_str(&format!(" WHERE {}", condition));
        }

        if let Some(limit) = self.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        let statement: Statement = self.connection.client.prepare(&query).await?;
        let rows: Vec<Row> = self.connection.client.query(&statement, &[]).await?;

        Ok(rows)
    }

    // implement placeholders
    pub async fn delete(mut self) -> Result<u64, Error> {
        let mut query = format!("DELETE FROM {}", self.table);

        if let Some(condition) = self.build_conditions() {
            query.push_str(&format!(" WHERE {}", condition));
        }

        let statement = self.connection.client.prepare(&query).await?;
        let rows_affected = self.connection.client.execute(&statement, &[]).await?;

        Ok(rows_affected)
    }

    fn build_placeholders(&self, command: Command) -> String {
        match command {
            Command::Insert => {
                if let Some(values) = &self.values {
                        (1..=self.values.as_ref().unwrap().len())
                        .map(|i| format!("${}", i))
                        .collect::<Vec<String>>()
                        .join(", ")
                } else {
                    String::new()
                }
            }
            Command::Update => {
                if let Some(columns) = &self.columns {
                    self
                        .columns
                        .as_ref()
                        .unwrap()
                        .iter()
                        .enumerate()
                        .map(|(i, col)| format!("{} = ${}", col, i + 1))
                        .collect::<Vec<String>>()
                        .join(", ")
                } else {
                    String::new()
                }
            }
            _ => String::new(),
        }
    }
}


#[allow(dead_code)]
enum Command {
    Insert,
    Update,
    Select,
    Delete,
}