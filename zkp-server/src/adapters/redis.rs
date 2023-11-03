use std::borrow::BorrowMut;

use redis::Commands;

use crate::{models::user_info::UserInfo, ports::db_port::DBPort};

#[derive(Clone)]
pub struct RedisAdapter {
    client: redis::Client,
}

impl RedisAdapter {
    pub async fn connect(uri: String) -> Self {
        let client = redis::Client::open(uri).expect("Invalid connection URL");
        RedisAdapter { client }
    }
}

impl DBPort<UserInfo> for RedisAdapter {
    async fn get_by_id(&mut self, id: String) -> Result<UserInfo, String> {
        return match self.borrow_mut().client.get(id) {
            Ok(value) => {
                let mut user = UserInfo::default();
                user.username = value;
                return Ok(user);
            }
            Err(err) => Err(err.to_string()),
        };
    }

    async fn create(&mut self, entity: UserInfo) -> Result<UserInfo, String> {
        let value: Result<String, redis::RedisError> = self.borrow_mut().client.set_ex(
            entity.clone().auth_id,
            entity.clone().username,
            60 * 60 * 24,
        );
        if value.is_err() {
            return Err(value.err().unwrap().to_string());
        } else {
            return Ok(entity);
        }
    }

    async fn update(&mut self, _: UserInfo) -> Result<UserInfo, String> {
        todo!()
    }
}
