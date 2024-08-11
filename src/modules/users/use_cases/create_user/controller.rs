use std::collections::HashMap;

use serde_json::Value;

use crate::modules::users::use_cases::create_user;
use crate::modules::users::use_cases::create_user::errors::CreateUserErrors;
use crate::modules::users::use_cases::create_user::errors::CreateUserErrors::CommonError;
use crate::modules::users::use_cases::create_user::view::UserCreateRequestBody;
use crate::shared::common::errors::CommonErrors::ValidationError;
use crate::shared::utils::conversion::value_to_hashmap;
use crate::shared::utils::validation::is_empty_value;

const INSERT_USER_ALLOWED_FIELDS: &[&str] = &["email", "username", "first_name", "last_name"];

fn has_every_insert_user_field(input: &HashMap<String, Value>) -> bool {
    INSERT_USER_ALLOWED_FIELDS.iter().all(|&field| {
        input.get(field).map_or(false, |value| !is_empty_value::check(value))
    })
}

fn has_only_insert_user_fields(input: &HashMap<String, Value>) -> bool {
    input.keys().all(|key| INSERT_USER_ALLOWED_FIELDS.contains(&key.as_str()))
}

fn is_user_insert_valid(input: &Value) -> bool {
    let value_map = value_to_hashmap::convert(input.clone());
    has_every_insert_user_field(&value_map) && has_only_insert_user_fields(&value_map)
}

pub async fn handle(payload: Value) -> Result<i64, CreateUserErrors> {
    if !is_user_insert_valid(&payload) {
        return Err(CommonError(ValidationError));
    }
    let user_create_request: UserCreateRequestBody = serde_json::from_value(payload).map_err(|_e| ValidationError)?;
    create_user::model::create(user_create_request)
}
