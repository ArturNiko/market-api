use uuid::Uuid;


#[allow(dead_code)]
#[derive(Debug)]
pub struct Customer {
    nickname: String,
    uuid: Uuid,
}

#[allow(dead_code)]
impl Customer {
    pub fn new(nickname: String) -> Self {
        Self {
            nickname,
            uuid: Uuid::new_v4(),
        }
    }

    pub fn nickname(&self) -> String { self.nickname.clone() }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
}