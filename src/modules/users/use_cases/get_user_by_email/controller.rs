use crate::modules::users::domain::user_entity::UserEntity;
use crate::modules::users::errors::UsersModuleErrors;
use crate::modules::users::use_cases::get_user_by_email::{self, view::GetUserByEmailParams};

pub async fn handle(dto: GetUserByEmailParams) -> Result<Option<UserEntity>, UsersModuleErrors> {
    get_user_by_email::model::execute(dto).await
}
