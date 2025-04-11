use crate::engine::models::Document;
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};

/// 🦀
/// Returns the current Unix timestamp in seconds.
///
/// This utility function is used throughout the system to timestamp entities such as:
/// - Document creation (`created_at`)
/// - Document updates (`updated_at`)
/// - Collection initialization
///
/// It calculates the number of seconds elapsed since the Unix epoch (`1970-01-01T00:00:00Z`)
/// using the system clock.
///
/// # Returns
///
/// - A `u64` representing the number of seconds since the Unix epoch.
///
/// # Example
///
/// ```rust
/// use nosqlite_rust::engine::models::utils::now;
///
/// let timestamp = now();
/// println!("Current UNIX timestamp: {}", timestamp);
/// ```
///
/// # Panics
///
/// This function will **panic** if the system clock is set before the Unix epoch,
/// which is highly unlikely on modern systems.
///
/// # Notes
///
/// - This function is timezone-independent (always UTC)
/// - Consider formatting timestamps for human output using a library like `chrono`
///
/// # See Also
///
/// - [`SystemTime::now`] — standard library source of time
/// - [`UNIX_EPOCH`] — constant used for offset
/// - [`Document::new`] — sets both `created_at` and `updated_at` using this
pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// 🦀
/// Validates a JSON document against a given schema definition.
///
/// This function performs recursive structural validation of a document based on a JSON schema.
/// The schema is defined as a map (`serde_json::Map`) where:
/// - Keys represent field names
/// - Values define the expected type as a string (e.g. `"string"`, `"number"`, `"boolean"`)
///   or a nested JSON object (representing a sub-schema for nested documents)
///
/// All fields defined in the schema must be present in the document, and their types must match.
/// Extra fields in the document are allowed (non-strict mode).
///
/// # Parameters
///
/// - `doc`: A reference to the document (as a JSON object) to validate.
/// - `schema`: A reference to the expected schema structure (also a JSON object).
///
/// # Returns
///
/// - `true` if the document fully conforms to the schema.
/// - `false` if any required field is missing or if any type mismatches occur.
///
/// # Supported Types
///
/// - `"string"`
/// - `"number"`
/// - `"boolean"`
/// - `"array"`
/// - `"object"`
///
/// # Example
///
/// ```rust
/// use serde_json::json;
/// use std::collections::BTreeMap;
/// use nosqlite_rust::engine::models::utils::validate_against_structure;
///
/// let schema = json!({
///     "title": "string",
///     "views": "number",
///     "meta": {
///         "tags": "array",
///         "published": "boolean"
///     }
/// }).as_object().unwrap().clone();
///
/// let doc = json!({
///     "title": "My Post",
///     "views": 120,
///     "meta": {
///         "tags": ["rust", "nosql"],
///         "published": true
///     }
/// }).as_object().unwrap().clone();
///
/// assert!(validate_against_structure(&doc, &schema));
/// ```
///
/// # Notes
///
/// - The function performs **recursive** validation for nested objects.
/// - Only fields present in the schema are checked; extra fields in the document are ignored.
/// - Type matching is handled internally by `type_matches` helper function.
///
/// # See Also
///
/// - [`serde_json::Value`] — JSON representation used in both document and schema
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

/// 🦀
/// Checks if a JSON value matches the expected type (case-insensitive).
///
/// This helper function performs a basic type check between a user-defined type string
/// and a [`serde_json::Value`]. It supports simple, high-level JSON data types and is
/// intended for use in schema validation (e.g., [`validate_against_structure`]).
///
/// # Parameters
///
/// - `expected`: A case-insensitive string representing the expected type.
/// - `val`: The actual [`Value`] to be checked.
///
/// # Supported Types
///
/// - `"string"` → [`Value::is_string()`]
/// - `"number"` → [`Value::is_number()`]
/// - `"boolean"` → [`Value::is_boolean()`]
/// - `"array"` → [`Value::is_array()`]
/// - `"object"` → [`Value::is_object()`]
///
/// # Returns
///
/// - `true` if the value matches the expected type
/// - `false` if the types are incompatible or the type string is unrecognized
///
/// # Notes
///
/// - Type names are matched case-insensitively (`"String"` and `"string"` are equivalent)
/// - Unrecognized type strings return `false`
///
/// # See Also
///
/// - [`validate_against_structure`] — uses this function for document schema enforcement
/// - [`serde_json::Value`] — the dynamic JSON type used for validation
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
