use uuid::Uuid;


#[allow(dead_code)]
#[derive(Debug)]
pub struct Customer<'a> {
    nickname: &'a str,
    uuid: Uuid,
}

#[allow(dead_code)]
impl<'a> Customer<'a> {
    pub fn new(nickname: &'a str) -> Self {
        Self {
            nickname,
            uuid: Uuid::new_v4(),
        }
    }

    pub fn nickname(&self) -> &str {
        self.nickname
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
}
