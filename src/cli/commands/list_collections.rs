use crate::engine::Nosqlite;

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
pub fn handle_list_collections(db: &mut Nosqlite) -> Result<String, String> {
    let collections = db.list_collections();
    if collections.is_empty() {
        return Ok("No collections found.".to_string());
    }

    let output = collections
        .iter()
        .map(|collection| collection.to_string())
        .collect::<Vec<String>>()
        .join("\n");
    Ok(output)
}
