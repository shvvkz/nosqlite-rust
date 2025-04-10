use crate::engine::error::{NosqliteError, NosqliteErrorHandler};
use crate::engine::models::collection::model::Collection;
use crate::engine::models::database::model::Database;
use serde_json::Value;

/// Creates a new collection in the database with a specified schema.
///
/// This function wraps the database's internal [`add_collection`] call, adding a layer of
/// centralized control and error logging. It validates that the collection name is unique
/// and that the provided structure is a valid JSON object (i.e., the schema).
///
/// This is the recommended way to define and register a new collection in your NoSQL engine.
///
/// # Parameters
///
/// - `db`: A mutable reference to the [`Database`] where the collection should be added.
/// - `name`: A string slice that uniquely identifies the collection.
/// - `structure`: A [`serde_json::Value`] defining the schema for the collection's documents. Must be a JSON object.
/// - `handler`: A mutable reference to a [`NosqliteErrorHandler`] that will log any validation or conflict errors.
///
/// # Returns
///
/// - `Ok(())` if the collection was created and registered successfully.
/// - `Err(NosqliteError)` if:
///   - The name is already taken
///   - The schema is not a valid JSON object
///
/// # Example
///
/// ```rust
/// use serde_json::json;
/// let schema = json!({ "title": "string", "views": "number" });
/// let mut db = Database::default();
/// let mut handler = NosqliteErrorHandler::new("db.nosqlite".to_string());
///
/// create_collection(&mut db, "posts", schema, &mut handler).unwrap();
/// ```
///
/// # Notes
///
/// - This function should be used in admin tooling, migrations, or at startup when defining collection layouts.
/// - Document validation is not enforced at creation time — but the schema will be used during insertion.
///
/// # See Also
///
/// - [`delete_collection`] — to remove a collection
/// - [`Collection`] — the structure being created
/// - [`Database::add_collection`] — the core insertion logic
///
/// ---
///
/// 🆕 Define a new schema-aware document store inside your NoSQL database.
///
/// 🔨🤖🔧 Powered by Rust
pub fn create_collection(
    db: &mut Database,
    name: &str,
    structure: Value,
    handler: &mut NosqliteErrorHandler,
) -> Result<(), NosqliteError> {
    db.add_collection(name, structure, handler)?;
    Ok(())
}

/// Removes a collection from the database by its name.
///
/// This function deletes the entire collection, including all documents and its structure.
/// If the collection name does not exist in the database, an error is returned and logged.
///
/// This is a **destructive operation**, typically used for cleanup, testing, or schema migration workflows.
///
/// # Parameters
///
/// - `db`: A mutable reference to the [`Database`] instance.
/// - `name`: The name of the collection to remove.
/// - `handler`: A mutable reference to a [`NosqliteErrorHandler`] to record errors if the collection is missing.
///
/// # Returns
///
/// - `Ok(())` if the collection was successfully removed.
/// - `Err(NosqliteError::CollectionNotFound)` if the name was not found in the database.
///
/// # Example
///
/// ```rust
/// let mut db = Database::default();
/// let mut handler = NosqliteErrorHandler::new("db.nosqlite".to_string());
/// create_collection(&mut db, "logs", json!({}), &mut handler).unwrap();
/// delete_collection(&mut db, "logs", &mut handler).unwrap();
/// ```
///
/// # Notes
///
/// - All documents in the collection are dropped from memory with this operation.
/// - The removal is immediate and cannot be undone unless manually backed up.
///
/// # See Also
///
/// - [`create_collection`] — for adding collections
/// - [`NosqliteError`] — error enum including `CollectionNotFound`
///
/// ---
///
/// 🗑️ Complete collection teardown — structure and contents.
///
/// 🔨🤖🔧 Powered by Rust
pub fn delete_collection(
    db: &mut Database,
    name: &str,
    handler: &mut NosqliteErrorHandler,
) -> Result<(), NosqliteError> {
    db.remove_collection(name, handler)?;
    Ok(())
}

/// Retrieves an immutable reference to a collection by name from the database.
///
/// This function is used to fetch a collection when you need to read its documents or schema
/// without modifying it. If the collection doesn't exist, the error is logged via the handler.
///
/// This is typically used in data access layers or query interfaces.
///
/// # Parameters
///
/// - `db`: A reference to the [`Database`] instance.
/// - `name`: The name of the collection to fetch.
/// - `handler`: A mutable reference to a [`NosqliteErrorHandler`] for logging if lookup fails.
///
/// # Returns
///
/// - `Ok(&Collection)` if found
/// - `Err(NosqliteError::CollectionNotFound)` if no collection with that name exists
///
/// # Example
///
/// ```rust
/// let collection = get_collection(&db, "users", &mut handler)?;
/// println!("Schema: {}", collection.structure);
/// ```
///
/// # Notes
///
/// - Matching is case-sensitive.
/// - If you need to modify the collection, use [`get_collection_mut`] instead.
///
/// # See Also
///
/// - [`get_collection_mut`] — for mutable access
/// - [`Collection`] — the return type
///
/// ---
///
/// 🔎 Safe, read-only access to a named collection.
///
/// 🔨🤖🔧 Powered by Rust
pub fn get_collection<'a>(
    db: &'a Database,
    name: &str,
    handler: &mut NosqliteErrorHandler,
) -> Result<&'a Collection, NosqliteError> {
    db.get_collection(name).ok_or_else(|| {
        let error = NosqliteError::CollectionNotFound(format!("Collection '{}' not found", name));
        handler.log_error(error.clone());
        error
    })
}

/// Retrieves a mutable reference to a collection by name from the database.
///
/// This function grants write access to a collection — allowing you to insert, update, or
/// delete documents inside it. If the collection is missing, an error is logged and returned.
///
/// # Parameters
///
/// - `db`: A mutable reference to the [`Database`] instance.
/// - `name`: The name of the collection to access.
/// - `handler`: A mutable error handler for logging any lookup failures.
///
/// # Returns
///
/// - `Ok(&mut Collection)` if the collection is found
/// - `Err(NosqliteError::CollectionNotFound)` otherwise
///
/// # Example
///
/// ```rust
/// let collection = get_collection_mut(&mut db, "users", &mut handler)?;
/// collection.add_document(json!({ "id": 1, "name": "Jane" }), &mut handler)?;
/// ```
///
/// # Notes
///
/// - This grants full access to modify the collection’s contents.
/// - Use with caution in concurrent/multi-threaded environments.
///
/// # See Also
///
/// - [`get_collection`] — for immutable access
/// - [`Collection`] — the structure being mutated
///
/// ---
///
/// ✍️ Direct access to collection internals for data manipulation.
///
/// 🔨🤖🔧 Powered by Rust
pub fn get_collection_mut<'a>(
    db: &'a mut Database,
    name: &str,
    handler: &mut NosqliteErrorHandler,
) -> Result<&'a mut Collection, NosqliteError> {
    db.get_collection_mut(name).ok_or_else(|| {
        let error = NosqliteError::CollectionNotFound(format!("Collection '{}' not found", name));
        handler.log_error(error.clone());
        error
    })
}

/// Lists all collections currently registered in the database.
///
/// This method returns an immutable reference to every [`Collection`] in the database,
/// allowing inspection of metadata, structure, or document count for each one.
///
/// # Parameters
///
/// - `db`: A reference to the [`Database`] instance.
///
/// # Returns
///
/// - A `Vec<&Collection>` containing references to all collections, in insertion order.
///
/// # Example
///
/// ```rust
/// for col in list_collections(&db) {
///     println!("📁 {} ({} docs)", col.name, col.document_count());
/// }
/// ```
///
/// # Notes
///
/// - Returned references are read-only.
/// - Useful for CLI dashboards, admin panels, or diagnostics.
///
/// # See Also
///
/// - [`Collection`] — the unit being returned
/// - [`Database::collections`] — the underlying data source
///
/// ---
///
/// 📋 Overview of all schema-defined collections in your NoSQL instance.
///
/// 🔨🤖🔧 Powered by Rust
pub fn list_collections(db: &Database) -> Vec<&Collection> {
    db.collections.iter().collect()
}
