use serde::{Deserialize, Serialize};

use crate::{
    adapters::{mongo::MongoAdapter, redis::RedisAdapter},
    ports::db_port::DBPort,
};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub username: String,
    pub session_id: String,
    pub auth_id: String,
    pub y1: String,
    pub y2: String,
    pub r1: String,
    pub r2: String,
    pub c: String,
    pub s: String,
}

impl UserInfo {
    pub async fn create_user(&self, client: &mut MongoAdapter) -> Result<Self, String> {
        client.create(self.clone()).await
    }

    pub async fn update(&self, client: &mut MongoAdapter) -> Result<Self, String> {
        client.update(self.clone()).await
    }

    pub async fn get_user(&mut self, client: &mut MongoAdapter) -> Result<(), String> {
        match client.get_by_id(self.username.clone()).await {
            Ok(res) => {
                *self = res.clone();
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub async fn save_auth_id(&self, client: &mut RedisAdapter) -> Result<Self, String> {
        client.create(self.clone()).await
    }
}
