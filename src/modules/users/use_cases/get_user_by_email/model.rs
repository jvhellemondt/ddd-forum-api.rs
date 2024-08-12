use crate::modules::users::domain::user::UserModel;
use crate::modules::users::errors::UsersErrors::{self, CommonError};
use crate::modules::users::repository::UsersRepository;
use crate::modules::users::use_cases::get_user_by_email::view::GetUserByEmailParams;
use crate::shared::common::errors::CommonErrors::ServerError;
use crate::shared::infrastructure::{database as db, database::repository::Repository};

pub fn execute(
    dto: GetUserByEmailParams,
) -> Result<Option<UserModel>, UsersErrors> {
    let repository = UsersRepository::new(db::connection::get_connection());
    match repository.get_by("email", &dto.email) {
        Ok(Some(user)) => Ok(Some(user)),
        Ok(None) => Ok(None),
        Err(_) => Err(CommonError(ServerError))
    }
}
