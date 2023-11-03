pub trait DBPort<UserInfo> {
    async fn get_by_id(&mut self, id: String) -> Result<UserInfo, String>;
    async fn create(&mut self, entity: UserInfo) -> Result<UserInfo, String>;
    async fn update(&mut self, entity: UserInfo) -> Result<UserInfo, String>;
}
