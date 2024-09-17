use crate::modules::users::domain::user_entity::UserEntity;
use crate::modules::users::errors::UsersModuleErrors::{self, CommonError};
use crate::modules::users::repositories::implementations::postgres_user_repository::PostgresUserRepository;
use crate::modules::users::repositories::user_repository::UserRepository;
use crate::modules::users::use_cases::get_user_by_email::view::GetUserByEmailParams;
use crate::shared::common::errors::CommonErrors::ServerError;

pub async fn execute(
    dto: GetUserByEmailParams,
) -> Result<Option<UserEntity>, UsersModuleErrors> {
    let repository = PostgresUserRepository::new();

    match repository.find_by_email(&dto.email.to_lowercase()).await {
        Ok(Some(user)) => Ok(Some(user)),
        Ok(None) => Ok(None),
        Err(_) => Err(CommonError(ServerError))
    }
}
