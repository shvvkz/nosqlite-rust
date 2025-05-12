use crate::engine::nosqlite::Nosqlite;
use serde_json::Value;

/// 🦀
/// Parses and executes the `db.insertDocument()` command from the CLI,
/// extracting the collection name and the document JSON to insert.
///
/// This function performs the following steps:
/// - Parses the input to extract the target collection name and document data as JSON.
/// - If no document is provided, it defaults to an empty JSON object `{}`.
/// - Calls [`Nosqlite::insert_document`] to insert the document into the specified collection.
///
/// # Parameters
///
/// - `input`: A string containing the CLI command to insert the document. Must follow the format:
///   - `db.insertDocument("collection_name", {document})`
/// - `db`: A mutable reference to the [`Nosqlite`] instance where the document should be inserted.
///
/// # Returns
///
/// - `Ok(String)` containing a success message if the document is inserted successfully.
/// - `Err(String)` describing the error if parsing fails or the document insertion fails.
///
/// # Errors
///
/// - Returns a string describing:
///   - Syntax errors (e.g., missing closing parenthesis or collection name).
///   - Invalid JSON in the document argument.
///   - Errors returned by the internal `insert_document` call (e.g., schema validation failure).
///
/// # Examples
///
/// ```text
/// > db.insertDocument("users", { "name": "Alice", "age": 30 })
/// Document has been inserted successfully.
/// ```
///
/// # See Also
///
/// - [`Nosqlite::insert_document`]
pub fn handle_insert_document(input: &str, db: &mut Nosqlite) -> Result<String, String> {
    let (name, document_json) = parse_command_args(input)?;
    db.insert_document(name, document_json)
        .map(|_| "Document has been inserted successfully.".to_string())
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
