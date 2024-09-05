use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{Error as SqlxError, PgPool};

use crate::modules::users::domain::user::User;
use crate::modules::users::errors::{UsersDomainErrors, UsersModuleErrors};
use crate::modules::users::repositories::user_repository::{InsertUserModel, UserRepository};
use crate::shared::common::errors::CommonErrors::DatabaseError;
use crate::shared::infrastructure::database::connection::get_db_pool;

pub struct PostgresUserRepository {
    pub pool: Arc<PgPool>,
}

impl PostgresUserRepository {
    pub fn new() -> Self {
        let pool = get_db_pool().clone();
        PostgresUserRepository { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: &InsertUserModel) -> Result<i32, UsersModuleErrors> {
        let result = sqlx::query!(
            "INSERT INTO users (email, username, first_name, last_name, password, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id",
            user.email,
            user.username,
            user.first_name,
            user.last_name,
            user.password,
            user.created_at,
            user.updated_at
        ).fetch_one(&*self.pool).await;

        match result {
            Ok(record) => Ok(record.id),
            _ => Err(UsersModuleErrors::CommonError(DatabaseError(None))),
        }
    }

    async fn find_by_id(&self, id: &i32) -> Result<Option<User>, UsersModuleErrors> {
        let result = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE id = $1",
            id
        ).fetch_optional(&*self.pool).await;

        match result {
            Ok(user) => Ok(user),
            Err(SqlxError::RowNotFound) => Err(UsersModuleErrors::DomainError(UsersDomainErrors::UserNotFound)),
            _ => Err(UsersModuleErrors::CommonError(DatabaseError(None))),
        }
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, UsersModuleErrors> {
        let result = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE username = $1",
            username
        ).fetch_optional(&*self.pool).await;

        match result {
            Ok(user) => Ok(user),
            Err(SqlxError::RowNotFound) => Err(UsersModuleErrors::DomainError(UsersDomainErrors::UserNotFound)),
            _ => Err(UsersModuleErrors::CommonError(DatabaseError(None))),
        }
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UsersModuleErrors> {
        let result = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE email = $1",
            email
        ).fetch_optional(&*self.pool).await;

        match result {
            Ok(user) => Ok(user),
            Err(SqlxError::RowNotFound) => Err(UsersModuleErrors::DomainError(UsersDomainErrors::UserNotFound)),
            _ => Err(UsersModuleErrors::CommonError(DatabaseError(None))),
        }
    }

    async fn update(&self, user: &User) -> Result<(), UsersModuleErrors> {
        let result = sqlx::query!(
            "UPDATE users SET email = $1, username = $2, first_name = $3, last_name = $4, password = $5, updated_at = $6 WHERE id = $7",
            user.email,
            user.username,
            user.first_name,
            user.last_name,
            user.password,
            user.updated_at,
            user.id
        ).execute(&*self.pool).await;

        match result {
            Ok(_) => Ok(()),
            Err(SqlxError::RowNotFound) => Err(UsersModuleErrors::DomainError(UsersDomainErrors::UserNotFound)),
            _ => Err(UsersModuleErrors::CommonError(DatabaseError(None))),
        }
    }
}
