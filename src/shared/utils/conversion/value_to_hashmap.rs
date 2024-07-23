use serde_json::Value;
use std::collections::HashMap;

pub fn convert(value: Value) -> HashMap<String, Value> {
    match value {
        Value::Object(map) => map.into_iter().collect(),
        _ => panic!("Expected a JSON object"),
    }
}
