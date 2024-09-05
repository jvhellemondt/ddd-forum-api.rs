use crate::modules::users::domain::user::User;
use crate::modules::users::errors::UsersModuleErrors;
use crate::modules::users::use_cases::get_user_by_email::{self, view::GetUserByEmailParams};

pub async fn handle(dto: GetUserByEmailParams) -> Result<Option<User>, UsersModuleErrors> {
    get_user_by_email::model::execute(dto).await
}
