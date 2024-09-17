use crate::modules::users::errors::{UsersDomainErrors, UsersModuleErrors};
use crate::modules::users::errors::UsersModuleErrors::CommonError;
use crate::modules::users::repositories::implementations::postgres_user_repository::PostgresUserRepository;
use crate::modules::users::repositories::user_repository::UserRepository;
use crate::modules::users::use_cases::update_user::controller::UserUpdateRequestBody;
use crate::shared::common::errors::CommonErrors::ServerError;

pub async fn execute(
    payload: UserUpdateRequestBody,
    id: i32,
) -> Result<(), UsersModuleErrors> {
    let repository = PostgresUserRepository::new();

    let mut user_model = match repository.find_by_id(&id).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(UsersModuleErrors::from(UsersDomainErrors::UserNotFound)),
        Err(_) => return Err(CommonError(ServerError)),
    };

    if let Some(ref email) = payload.email {
        if let Ok(Some(user)) = repository.find_by_email(email).await {
            if user.id != user_model.id {
                return Err(UsersModuleErrors::from(UsersDomainErrors::EmailAlreadyInUse));
            }
        }
    }

    if let Some(ref username) = payload.username {
        if let Ok(Some(user)) = repository.find_by_username(username).await {
            if user.id != id {
                return Err(UsersModuleErrors::from(UsersDomainErrors::UsernameAlreadyTaken));
            }
        }
    }

    if let Some(email) = payload.email {
        user_model.email = email;
    }
    if let Some(username) = payload.username {
        user_model.username = username;
    }
    if let Some(first_name) = payload.first_name {
        user_model.first_name = first_name;
    }
    if let Some(last_name) = payload.last_name {
        user_model.last_name = last_name;
    }

    repository.update(&user_model).await
}
