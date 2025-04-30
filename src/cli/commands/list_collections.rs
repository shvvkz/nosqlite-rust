use crate::engine::Nosqlite;

/// 🦀
/// Parses and executes the `db.listCollections()` command from the CLI,
/// retrieving the list of all collections in the database.
///
/// This function does the following:
/// - Calls [`Nosqlite::list_collections`] to fetch the list of collections
/// - Formats the output as a newline-separated string
/// - Returns a message if no collections are found
///
/// # Parameters
///
/// - `db`: A mutable reference to the [`Nosqlite`] instance from which the collections are listed.
///
/// # Returns
///
/// - `Ok(String)` containing the list of collections as a newline-separated string if successful.
/// - `Ok(String)` with a message indicating no collections are found if the database is empty.
/// - `Err(String)` describing any errors encountered during the operation.
///
/// # Errors
///
/// - Returns a string describing errors if the operation fails unexpectedly.
///
/// # See Also
///
/// - [`Nosqlite::list_collections`]
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
