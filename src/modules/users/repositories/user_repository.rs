use async_trait::async_trait;
use crate::modules::users::domain::user_entity::UserEntity;
use crate::modules::users::errors::{UsersModuleErrors};

#[async_trait]
pub trait UserRepository {
    async fn create(&self, user: &UserEntity) -> Result<i32, UsersModuleErrors>;
    async fn find_by_id(&self, id: &i32) -> Result<Option<UserEntity>, UsersModuleErrors>;
    async fn find_by_username(&self, username: &str) -> Result<Option<UserEntity>, UsersModuleErrors>;
    async fn find_by_email(&self, email: &str) -> Result<Option<UserEntity>, UsersModuleErrors>;
    async fn update(&self, user: &UserEntity) -> Result<(), UsersModuleErrors>;
}
