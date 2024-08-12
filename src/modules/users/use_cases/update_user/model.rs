use crate::modules::users::errors::UsersDomainErrors::{EmailAlreadyInUse, UsernameAlreadyTaken, UserNotFound};
use crate::modules::users::errors::UsersErrors::{self, CommonError, DomainError};
use crate::modules::users::repository::UsersRepository;
use crate::modules::users::use_cases::update_user::controller::UserUpdateRequestBody;
use crate::shared::common::errors::CommonErrors::ServerError;
use crate::shared::infrastructure::database as db;
use crate::shared::infrastructure::database::repository::Repository;

pub fn execute(
    payload: UserUpdateRequestBody,
    id: i64,
) -> Result<(), UsersErrors> {
    let repository = UsersRepository::new(db::connection::get_connection());

    let mut user_model = match repository.get_by("id", id) {
        Ok(Some(user)) => user,
        Ok(None) => return Err(DomainError(UserNotFound)),
        Err(_) => return Err(CommonError(ServerError)),
    };

    if let Some(ref email) = payload.email {
        if let Ok(Some(user)) = repository.get_by("email", email) {
            // @TODO: why is it saying payload.id is an Option. It's not?
            if user.id != user_model.id {
                return Err(DomainError(EmailAlreadyInUse));
            }
        }
    }

    if let Some(ref username) = payload.username {
        if let Ok(Some(user)) = repository.get_by("username", username) {
            // @TODO: why is it saying payload.id is an Option. It's not?
            if user.id != Option::from(id) {
                return Err(DomainError(UsernameAlreadyTaken));
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

    repository.update(&user_model)
}
