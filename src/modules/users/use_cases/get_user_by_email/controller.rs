use crate::modules::users::domain::user::UserModel;
use crate::modules::users::errors::UsersErrors;
use crate::modules::users::use_cases::get_user_by_email::{self, view::GetUserByEmailParams};

pub async fn handle(dto: GetUserByEmailParams) -> Result<Option<UserModel>, UsersErrors> {
    get_user_by_email::model::execute(dto)
}
