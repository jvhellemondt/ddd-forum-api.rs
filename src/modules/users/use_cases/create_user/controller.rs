use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::modules::users::errors::UsersModuleErrors;
use crate::modules::users::use_cases::create_user;
use crate::shared::common::errors::CommonErrors::ValidationError;
use crate::shared::utils::conversion::value_to_hashmap;
use crate::shared::utils::validation::is_empty_value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreateRequestBody {
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

const INSERT_USER_ALLOWED_FIELDS: &[&str] = &["email", "username", "first_name", "last_name"];


fn has_every_insert_user_field(input: &HashMap<String, Value>) -> bool {
    INSERT_USER_ALLOWED_FIELDS.iter().all(|&field| {
        input.get(field).map_or(false, |value| !is_empty_value::check(value))
    })
}


fn has_only_insert_user_fields(input: &HashMap<String, Value>) -> bool {
    input.keys().all(|key| INSERT_USER_ALLOWED_FIELDS.contains(&key.as_str()))
}


fn is_user_payload_valid(input: &Value) -> bool {
    let value_map = value_to_hashmap::convert(input.clone());
    has_every_insert_user_field(&value_map) && has_only_insert_user_fields(&value_map)
}


pub async fn handle(payload: Value) -> Result<i32, UsersModuleErrors> {
    if !is_user_payload_valid(&payload) {
        return Err(UsersModuleErrors::from(ValidationError));
    }
    let user_create_request: UserCreateRequestBody = serde_json::from_value(payload).map_err(|_e| ValidationError)?;
    create_user::model::execute(user_create_request).await
}
