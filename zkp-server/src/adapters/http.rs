use std::str::FromStr;

use chaum_pedersen_zkp::{models::ZKP, server::ZKPServer};
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{models::user_info::UserInfo, ports::api_port::APIPort};

use super::{mongo::MongoAdapter, redis::RedisAdapter};

#[derive(Clone)]
pub struct ApiAdapter {
    pub core: ZKP,
    pub mongo: MongoAdapter,
    pub redis: RedisAdapter,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct RegisterRequest {
    pub username: String,
    pub y1: String,
    pub y2: String,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct ChallengeRequest {
    pub username: String,
    pub r1: String,
    pub r2: String,
}

#[derive(Serialize, Deserialize, Validate, Debug, Clone)]
pub struct VerifyRequest {
    pub username: String,
    pub auth_id: String,
    pub s: String,
}

impl APIPort for ApiAdapter {
    async fn register(&mut self, req: RegisterRequest) -> Result<UserInfo, String> {
        let mut user_info = UserInfo {
            username: req.username.to_string(),
            c: "".to_string(),
            r1: "".to_string(),
            r2: "".to_string(),
            s: "".to_string(),
            session_id: "".to_string(),
            auth_id: "".to_string(),
            y1: req.y1.to_string(),
            y2: req.y2.to_string(),
        };

        return match user_info.get_user(&mut self.mongo.clone()).await {
            Ok(_) => {
                if user_info.y1 == "" {
                    return user_info.create_user(&mut self.mongo.clone()).await;
                } else {
                    Err("User already exists".to_string())
                }
            }
            Err(err) => Err(err),
        };
    }

    async fn create_challenge(&mut self, req: ChallengeRequest) -> Result<UserInfo, String> {
        let mut user_info = UserInfo {
            username: req.username,
            c: "".to_string(),
            r1: "".to_string(),
            r2: "".to_string(),
            s: "".to_string(),
            auth_id: "".to_string(),
            session_id: "".to_string(),
            y1: "".to_string(),
            y2: "".to_string(),
        };

        match user_info.get_user(&mut self.mongo.clone()).await {
            Ok(_) => {
                let (_, _, _, q) = ZKP::generate_constants();
                user_info.c = ZKP::generate_rand(&q).to_string();
                user_info.r1 = req.r1;
                user_info.r2 = req.r2;
                user_info.auth_id = ZKP::generate_rand_string(12);
                if let Err(err) = user_info.update(&mut self.mongo.clone()).await {
                    return Err(err);
                };

                if let Err(err) = user_info.save_auth_id(&mut self.redis.clone()).await {
                    return Err(err);
                };

                Ok(user_info)
            }
            Err(err) => Err(err),
        }
    }

    async fn verify_challenge(&mut self, req: VerifyRequest) -> Result<UserInfo, String> {
        let (alpha, beta, p, q) = ZKP::generate_constants();
        let zkp = ZKP { alpha, beta, p, q };

        let mut user_info = UserInfo::default();
        user_info.username = req.username;
        let solve = req.s;
        match user_info.get_user(&mut self.mongo.clone()).await {
            Ok(_) => match user_info.update(&mut self.mongo.clone()).await {
                Ok(user) => {
                    println!("update 2 {:?}", user);
                    if zkp.verify(
                        &BigInt::from_str(user.r1.as_str()).unwrap(),
                        &BigInt::from_str(user.r2.as_str()).unwrap(),
                        &BigInt::from_str(user.y1.as_str()).unwrap(),
                        &BigInt::from_str(user.y2.as_str()).unwrap(),
                        &BigInt::from_str(user.c.as_str()).unwrap(),
                        &BigInt::from_str(solve.as_str()).unwrap(),
                    ) {
                        user_info.session_id = ZKP::generate_rand_string(12);
                    } else {
                        return Err("Verifiction Failed".to_string());
                    }

                    Ok(user_info)
                }
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        }
    }
}
