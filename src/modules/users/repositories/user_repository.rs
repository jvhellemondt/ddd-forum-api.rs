use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::modules::users::domain::user::User;
use crate::modules::users::errors::{UsersModuleErrors};

#[derive(Debug, Clone, Serialize)]
pub struct InsertUserModel {
    pub id: Option<i32>,
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[async_trait]
pub trait UserRepository {
    async fn create(&self, user: &InsertUserModel) -> Result<i32, UsersModuleErrors>;
    async fn find_by_id(&self, id: &i32) -> Result<Option<User>, UsersModuleErrors>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, UsersModuleErrors>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UsersModuleErrors>;
    async fn update(&self, user: &User) -> Result<(), UsersModuleErrors>;
}
