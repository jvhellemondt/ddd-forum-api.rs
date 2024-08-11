use chrono::Local;
use ulid::Ulid;

use crate::modules::users::{repository::UsersRepository, use_cases::create_user::view};
use crate::modules::users::errors::UsersErrors::{EmailAlreadyInUse, UsernameAlreadyTaken};
use crate::modules::users::use_cases::create_user::errors::CreateUserErrors::{self, CommonError, UsersError};
use crate::shared::common::errors::CommonErrors::UnexpectedServerError;
use crate::shared::infrastructure::database as db;
use crate::shared::infrastructure::database::repository::Repository;

#[derive(Debug)]
pub struct UserModel {
    pub id: Option<i64>,
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub fn create(
    payload: view::UserCreateRequestBody,
) -> Result<i64, CreateUserErrors> {
    let now = Local::now();
    let user = UserModel {
        id: None,
        email: payload.email.to_string(),
        username: payload.username.to_string(),
        first_name: payload.first_name.to_string(),
        last_name: payload.last_name.to_string(),
        password: Option::from(Ulid::new().to_string()),
        created_at: now.to_rfc3339(),
        updated_at: now.to_rfc3339(),
    };

    let repository = UsersRepository::new(db::connection::get_connection());

    if let Ok(Some(_)) = repository.get_by("email", &user.email) {
        return Err(UsersError(EmailAlreadyInUse));
    }

    if let Ok(Some(_)) = repository.get_by("username", &user.username) {
        return Err(UsersError(UsernameAlreadyTaken));
    }

    repository.create(&user).map_err(|_e| CommonError(UnexpectedServerError))
}
