use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the current Unix timestamp in seconds.
///
/// This is used for populating the `created_at` and `updated_at` fields
/// in documents and collections.
///
/// # Returns
///
/// A `u64` representing the number of seconds since the Unix epoch.
pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Validates a JSON document against a given schema definition.
///
/// The schema should be a JSON object where each key maps to:
/// - a string representing a basic type (`"string"`, `"number"`, `"boolean"`, `"array"`, `"object"`), or
/// - a nested object representing a sub-schema.
///
/// # Arguments
///
/// * `doc` - A reference to the document to validate (as a map).
/// * `schema` - A reference to the expected structure/schema (as a map).
///
/// # Returns
///
/// `true` if the document matches the schema, `false` otherwise.
///
/// # Examples
///
/// ```rust
/// use serde_json::json;
/// use std::collections::BTreeMap;
/// use crate::engine::models::utils::validate_against_structure;
///
/// let schema = json!({ "title": "string", "views": "number" }).as_object().unwrap().clone();
/// let doc = json!({ "title": "Hello", "views": 42 }).as_object().unwrap().clone();
///
/// assert!(validate_against_structure(&doc, &schema));
/// ```
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

/// Checks if a JSON value matches the expected type.
///
/// # Arguments
///
/// * `expected` - A string representing the expected type (case-insensitive).
/// * `val` - The actual JSON value to check.
///
/// # Supported types
///
/// - `"string"`
/// - `"number"`
/// - `"boolean"`
/// - `"array"`
/// - `"object"`
///
/// # Returns
///
/// `true` if the value matches the type, `false` otherwise.
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
