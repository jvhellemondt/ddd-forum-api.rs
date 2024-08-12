use chrono::Local;
use ulid::Ulid;

use crate::modules::users::domain::user::UserModel;
use crate::modules::users::errors::UsersDomainErrors::{EmailAlreadyInUse, UsernameAlreadyTaken};
use crate::modules::users::errors::UsersErrors::{self, DomainError};
use crate::modules::users::repository::UsersRepository;
use crate::modules::users::use_cases::create_user::controller;
use crate::shared::infrastructure::database as db;
use crate::shared::infrastructure::database::repository::Repository;

pub fn execute(
    payload: controller::UserCreateRequestBody,
) -> Result<i64, UsersErrors> {
    let now = Local::now();
    let user = UserModel {
        id: None,
        email: payload.email.to_string().to_lowercase(),
        username: payload.username.to_string(),
        first_name: payload.first_name.to_string(),
        last_name: payload.last_name.to_string(),
        password: Option::from(Ulid::new().to_string()),
        created_at: now.to_rfc3339(),
        updated_at: now.to_rfc3339(),
    };

    let repository = UsersRepository::new(db::connection::get_connection());

    if let Ok(Some(_)) = repository.get_by("email", &user.email) {
        return Err(DomainError(EmailAlreadyInUse));
    }

    if let Ok(Some(_)) = repository.get_by("username", &user.username) {
        return Err(DomainError(UsernameAlreadyTaken));
    }

    repository.create(&user)
}
