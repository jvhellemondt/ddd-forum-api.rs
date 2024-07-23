use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

pub fn convert<T>(s: &T) -> HashMap<String, String>
where
    T: Serialize,
{
    let json_value = serde_json::to_value(s).expect("Failed to serialize struct");
    match json_value {
        Value::Object(map) => map
            .into_iter()
            .filter_map(|(k, v)| v.as_str().map(|v| (k, v.to_string())))
            .collect(),
        _ => panic!("Expected a JSON object"),
    }
}
