use uuid::{Uuid};


#[allow(dead_code)]
#[derive(Clone)]
pub struct Item {
    name: String,
    uuid: Uuid,
}

#[allow(dead_code)]
impl Item {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            uuid: Uuid::new_v4()
        }
    }

    pub fn name(&self) -> String { self.name.clone() }

    pub fn uuid(&self) -> Uuid {
        self.uuid.clone()
    }
}