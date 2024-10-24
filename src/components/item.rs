use uuid::{Uuid};


#[allow(dead_code)]
#[derive(Clone)]
pub struct Item<'a> {
    name: &'a str,
    uuid: Uuid,
}

#[allow(dead_code)]
impl<'a> Item<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            uuid: Uuid::new_v4()
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid.clone()
    }
}