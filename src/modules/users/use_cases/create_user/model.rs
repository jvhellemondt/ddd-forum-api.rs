use chrono::Utc;
use ulid::Ulid;

use crate::modules::users::domain::user_entity::UserEntity;
use crate::modules::users::errors::{UsersDomainErrors, UsersModuleErrors};
use crate::modules::users::repositories::implementations::postgres_user_repository::PostgresUserRepository;
use crate::modules::users::repositories::user_repository::UserRepository;
use crate::modules::users::use_cases::create_user::controller;

pub async fn execute(
    payload: controller::UserCreateRequestBody,
) -> Result<i32, UsersModuleErrors> {
    let user = UserEntity {
        id: None,
        email: payload.email.to_lowercase(),
        username: payload.username.to_string(),
        first_name: payload.first_name.to_string(),
        last_name: payload.last_name.to_string(),
        password: Some(Ulid::new().to_string()),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let repository = PostgresUserRepository::new();

    if let Ok(Some(_)) = repository.find_by_email(&user.email).await {
        return Err(UsersModuleErrors::from(UsersDomainErrors::EmailAlreadyInUse));
    }

    if let Ok(Some(_)) = repository.find_by_username(&user.username).await {
        return Err(UsersModuleErrors::from(UsersDomainErrors::UsernameAlreadyTaken));
    }

    repository.create(&user).await
}
