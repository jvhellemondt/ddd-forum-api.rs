use serde_json::Value;

pub fn check(value: &Value) -> bool {
    match value {
        Value::Null => true,
        Value::Bool(_) => false,
        Value::Number(_) => false,
        Value::String(s) => s.is_empty(),
        Value::Array(arr) => arr.is_empty(),
        Value::Object(map) => map.is_empty(),
    }
}
