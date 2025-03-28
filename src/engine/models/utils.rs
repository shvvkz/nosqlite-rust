use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn validate_against_structure(
    doc: &serde_json::Map<String, Value>,
    schema: &serde_json::Map<String, Value>,
) -> bool {
    for (key, expected_type) in schema {
        match doc.get(key) {
            Some(actual_value) => match (expected_type, actual_value) {
                (Value::String(s_type), val) => {
                    if !type_matches(s_type, val) {
                        return false;
                    }
                }
                (Value::Object(sub_schema), Value::Object(sub_doc)) => {
                    if !validate_against_structure(sub_doc, sub_schema) {
                        return false;
                    }
                }
                _ => return false,
            },
            None => return false,
        }
    }
    true
}

fn type_matches(expected: &str, val: &Value) -> bool {
    match expected.to_lowercase().as_str() {
        "string" => val.is_string(),
        "number" => val.is_number(),
        "boolean" => val.is_boolean(),
        "array" => val.is_array(),
        "object" => val.is_object(),
        _ => false,
    }
}
