use crate::engine::nosqlite::Nosqlite;
use serde_json::Value;

/// 🦀
/// Handles the `db.findDocuments()` CLI command.
///
/// Supported formats:
/// - `db.findDocuments("collection")`
/// - `db.findDocuments("collection", {filter})`
/// - `db.findDocuments("collection", {filter}, {projection})`
///
/// # Parameters
/// - `input`: Raw CLI command string.
/// - `db`: Mutable reference to the NoSQLite instance.
///
/// # Returns
/// - `Ok(String)` with pretty-printed JSON documents.
/// - `Err(String)` on syntax or execution errors.
pub fn handle_find_documents(input: &str, db: &mut Nosqlite) -> Result<String, String> {
    let (collection, filter, projection) = parse_find_command_args(input)?;

    let docs = db
        .get_documents(collection, &filter, &projection)
        .map_err(|e| format!("Error retrieving documents, {e}"))?;

    let mut output = String::new();

    for doc in docs {
        let formatted = serde_json::to_string_pretty(&doc)
            .unwrap_or_else(|_| "{ \"error\": \"Failed to serialize document\" }".to_string());
        output.push_str(&formatted);
        output.push('\n');
    }

    Ok(output)
}

/// Parses the CLI arguments for `db.findDocuments(...)`.
fn parse_find_command_args(input: &str) -> Result<(&str, Value, Value), String> {
    let args = input
        .strip_prefix("db.findDocuments(")
        .and_then(|s| s.strip_suffix(')'))
        .ok_or_else(|| "Syntax error: missing closing ')'.".to_string())?;

    let mut parts = args.splitn(3, ',').map(str::trim);

    // Collection name
    let collection = parts
        .next()
        .ok_or_else(|| "Syntax error: missing collection name.".to_string())?
        .trim_matches(|c| c == '"' || c == '\'');

    // Filter (optional)
    let filter = match parts.next() {
        Some(json_str) => {
            let json_clean = json_str.replace('\'', "\"");
            serde_json::from_str(&json_clean).map_err(|_| "Invalid JSON filter.".to_string())?
        }
        None => Value::Object(serde_json::Map::new()), // No filter → match all
    };

    // Projection (optional)
    let projection = match parts.next() {
        Some(json_str) => {
            let json_clean = json_str.replace('\'', "\"");
            serde_json::from_str(&json_clean).map_err(|_| "Invalid JSON projection.".to_string())?
        }
        None => Value::Object(serde_json::Map::new()), // No projection → return full document
    };

    Ok((collection, filter, projection))
}
