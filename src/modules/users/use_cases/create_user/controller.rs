use serde_json::Value;
use std::collections::HashMap;

use crate::modules::users::use_cases::create_user;
use crate::modules::users::use_cases::create_user::view::{
    UserCreateRequestBody, UserCreatedResponse,
};
use crate::shared::common::errors::CommonErrors;
use crate::shared::utils::conversion::value_to_hashmap;
use crate::shared::utils::validation::is_empty_value;

const INSERT_USER_ALLOWED_FIELDS: &[&str] = &["email", "username", "first_name", "last_name"];

fn has_every_insert_user_field(input: &HashMap<String, Value>) -> bool {
    INSERT_USER_ALLOWED_FIELDS.iter().all(|&field| {
        input
            .get(field)
            .map_or(false, |value| !is_empty_value::check(value))
    })
}

fn has_only_insert_user_fields(input: &HashMap<String, Value>) -> bool {
    input
        .keys()
        .all(|key| INSERT_USER_ALLOWED_FIELDS.contains(&key.as_str()))
}

fn is_user_insert_valid(input: &Value) -> bool {
    let value_map = value_to_hashmap::convert(input.clone());
    has_every_insert_user_field(&value_map) && has_only_insert_user_fields(&value_map)
}

pub fn handle(payload: Value) -> Result<UserCreatedResponse, CommonErrors> {
    if !is_user_insert_valid(&payload) {
        return Err(CommonErrors::ValidationError);
    }
    let user_create_request: UserCreateRequestBody =
        serde_json::from_value(payload).map_err(|_e| CommonErrors::ValidationError)?;

    match create_user::model::create(user_create_request) {
        Ok(user) => Ok(user),
        _ => Err(CommonErrors::UnexpectedServerError),
    }
}
