use crate::engine::nosqlite::Nosqlite;
use serde_json::Value;

/// 🦀
/// Parses and executes the `db.createCollection()` command from the CLI,
/// extracting the collection name and optional schema from the input.
///
/// This function does the following:
/// - Parses the input to extract the collection name and an optional JSON schema
/// - Falls back to an empty schema (`{}`) if no schema is provided
/// - Calls [`Nosqlite::create_collection`] to create and register the collection in the database
///
/// # Parameters
///
/// - `input`: A string containing the CLI command to create the collection. Must follow the format:
///   - `db.createCollection("name")`
///   - `db.createCollection("name", {schema})`
/// - `db`: A mutable reference to the [`Nosqlite`] instance where the collection should be added.
///
/// # Returns
///
/// - `Ok(String)` containing a success message if the collection is created successfully.
/// - `Err(String)` describing the error if parsing fails or collection creation fails.
///
/// # Errors
///
/// - Returns a string describing:
///   - Syntax errors (e.g. missing closing parenthesis or name)
///   - Invalid JSON in the schema
///   - Errors returned by the internal `create_collection` call
///
/// # See Also
///
/// - [`todo!`]
pub fn handle_insert_document(input: &str, db: &mut Nosqlite) -> Result<String, String> {
    let (name, document_json) = parse_command_args(input)?;
    db.insert_document(name, document_json)
        .map(|_| format!("Document has been inserted successfully."))
        .map_err(|e| format!("Failed to insert document: {e}"))
}

fn parse_command_args(input: &str) -> Result<(&str, Value), String> {
    let args = input
        .strip_prefix("db.insertDocument(")
        .and_then(|s| s.strip_suffix(')'))
        .ok_or_else(|| "Syntax error: missing closing ')'".to_string())?;

    let mut parts = args.splitn(2, ',').map(str::trim);

    let name_raw = parts
        .next()
        .ok_or_else(|| "Syntax error: collection name missing".to_string())?
        .trim_matches(|c| c == '"' || c == '\'');

    let document_json = match parts.next() {
        Some(json_str) => {
            let json_str = json_str.replace('\'', "\"");
            serde_json::from_str(&json_str).map_err(|_| "Invalid JSON schema".to_string())?
        }
        None => Value::Object(serde_json::Map::new()),
    };
    Ok((name_raw, document_json))
}
