use crate::{
    adapters::http::{ChallengeRequest, RegisterRequest, VerifyRequest},
    models::user_info::UserInfo,
};

pub trait APIPort {
    async fn register(&mut self, req: RegisterRequest) -> Result<UserInfo, String>;
    async fn create_challenge(&mut self, req: ChallengeRequest) -> Result<UserInfo, String>;
    async fn verify_challenge(&mut self, req: VerifyRequest) -> Result<UserInfo, String>;
}
