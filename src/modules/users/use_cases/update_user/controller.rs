use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::modules::users::errors::UsersModuleErrors;
use crate::modules::users::errors::UsersModuleErrors::CommonError;
use crate::modules::users::use_cases::update_user;
use crate::shared::common::errors::CommonErrors::ValidationError;
use crate::shared::utils::conversion::value_to_hashmap;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdateRequestBody {
    pub email: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

const INSERT_USER_ALLOWED_FIELDS: &[&str] = &["email", "username", "first_name", "last_name"];

fn has_only_insert_user_fields(input: &HashMap<String, Value>) -> bool {
    input.keys().all(|key| INSERT_USER_ALLOWED_FIELDS.contains(&key.as_str()))
}

fn is_user_payload_valid(input: &Value) -> bool {
    let value_map = value_to_hashmap::convert(input.clone());
    has_only_insert_user_fields(&value_map)
}

pub async fn handle(payload: Value, id: i32) -> Result<(), UsersModuleErrors> {
    if !is_user_payload_valid(&payload) {
        return Err(CommonError(ValidationError));
    }
    let body: UserUpdateRequestBody = serde_json::from_value(payload).map_err(|_e| CommonError(ValidationError))?;
    update_user::model::execute(body, id).await
}

