use dotenv::dotenv;
use std::env;
use std::io::ErrorKind::InvalidInput;
use tokio_postgres::{Client, Error, NoTls, Row};

pub struct Connection {
    client: Client, // Private client
}

pub struct QueryBuilder<'a> {
    connection: &'a Connection,
    table: String,
    columns: Option<Vec<String>>, // Columns for insertion
    values: Option<Vec<String>>,  // Values for insertion
    condition: Option<String>,
    limit: Option<usize>,
    updates: Option<Vec<(String, String)>>,
}

impl Connection {
    /// Creates a new database connection
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

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        Ok(Self { client })
    }

    /// Starts a new query builder
    pub fn query(&self, table: &str) -> QueryBuilder {
        QueryBuilder {
            connection: self,
            table: table.to_string(),
            columns: None,
            values: None,
            condition: None,
            limit: None,
            updates: None,
        }
    }
}

impl<'a> QueryBuilder<'a> {
    pub fn values(mut self, columns: Vec<&str>, values: Vec<&str>) -> Self {
        self.columns = Some(columns.into_iter().map(|col| col.to_string()).collect());
        self.values = Some(values.into_iter().map(|val| val.to_string()).collect());
        self
    }

    pub fn where_condition(mut self, condition: &str) -> Self {
        self.condition = Some(condition.to_string());
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub async fn select(self) -> Result<Vec<Row>, Error> {
        let columns = self
            .columns
            .map(|cols| cols.join(", "))
            .unwrap_or_else(|| "*".to_string());

        let mut query = format!("SELECT {} FROM {}", columns, self.table);

        if let Some(condition) = self.condition {
            query.push_str(&format!(" WHERE {}", condition));
        }

        if let Some(limit) = self.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        let statement = self.connection.client.prepare(&query).await?;
        let new_rows = self.connection.client.query(&statement, &[]).await?;

        Ok(new_rows)
    }

    pub async fn insert(self) -> Result<u64, Error> {
        if self.columns.is_none() || self.values.is_none() {
            return Err(Error::from(
                std::io::Error::new(InvalidInput, "Columns or values missing for INSERT query").into()
            ));
        }

        let columns = self.columns.unwrap().join(", ");
        let placeholders: Vec<String> = (1..=self.values.as_ref().unwrap().len())
            .map(|i| format!("${}", i))
            .collect();
        let placeholders = placeholders.join(", ");
        let values = self.values.unwrap();

        let query = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            self.table, columns, placeholders
        );

        let statement = self.connection.client.prepare(&query).await?;
        let params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> =
            values.iter().map(|val| val as _).collect();
        let rows_affected = self.connection.client.execute(&statement, &params).await?;

        Ok(rows_affected)
    }

    pub async fn delete(self) -> Result<u64, Error> {
        let mut query = format!("DELETE FROM {}", self.table);

        if let Some(condition) = self.condition {
            query.push_str(&format!(" WHERE {}", condition));
        }

        let statement = self.connection.client.prepare(&query).await?;
        let rows_affected = self.connection.client.execute(&statement, &[]).await?;

        Ok(rows_affected)
    }

    pub async fn update(self) -> Result<u64, Error> {
        if self.updates.is_none() {
            return Err(Error::from(
                std::io::Error::new(InvalidInput, "No updates specified for UPDATE query").into(),
            ));
        }

        let updates = self
            .updates
            .unwrap()
            .into_iter()
            .map(|(col, value)| format!("{} = '{}'", col, value))
            .collect::<Vec<_>>()
            .join(", ");

        let mut query = format!("UPDATE {} SET {}", self.table, updates);

        if let Some(condition) = self.condition {
            query.push_str(&format!(" WHERE {}", condition));
        }

        let statement = self.connection.client.prepare(&query).await?;
        let rows_affected = self.connection.client.execute(&statement, &[]).await?;

        Ok(rows_affected)
    }
}
