use crate::engine::error::{NosqliteError, NosqliteErrorHandler};
use crate::engine::models::database::model::Database;
use crate::engine::models::document::model::Document;
use serde_json::Value;

/// 🦀
/// Inserts a new document into the specified collection.
///
/// This function validates the incoming document against the collection's schema before
/// assigning it a unique ID and timestamps. If the validation fails or the collection
/// does not exist, the error is logged and returned.
///
/// # Parameters
///
/// - `db`: A mutable reference to the [`Database`] instance.
/// - `collection_name`: The name of the target collection.
/// - `data`: The document content as a [`serde_json::Value`] (must be a JSON object).
/// - `handler`: The error handler used for logging schema or collection errors.
///
/// # Returns
///
/// - `Ok(())` on successful validation and insertion
/// - `Err(NosqliteError)` if validation fails or collection is missing
///
/// # Example
///
/// ```rust
/// use serde_json::json;
/// use nosqlite_rust::engine::{error::{NosqliteErrorHandler, NosqliteError}, models::{Database,Collection}};
/// use nosqlite_rust::engine::services::document_service::insert_document;
///
/// let mut db = Database::new("temp/data22.nosqlite");
/// let mut handler = NosqliteErrorHandler::new("temp/data22.nosqlite".to_string());
/// db.add_collection("users", json!({}) ,&mut handler)?;
/// let col = db.get_collection_mut("users").unwrap();
/// insert_document(&mut db, "users", json!({ "id": 1, "name": "Alice" }), &mut handler)?;
/// Ok::<(), NosqliteError>(())
/// ```
///
/// # See Also
///
/// - [`update_document`] — full document replacement
/// - [`get_document_by_id`] — lookup inserted document
pub fn insert_document(
    db: &mut Database,
    collection_name: &str,
    data: Value,
    handler: &mut NosqliteErrorHandler,
) -> Result<(), NosqliteError> {
    let collection = db.get_collection_mut(collection_name).ok_or_else(|| {
        let error = NosqliteError::CollectionNotFound(format!(
            "Collection '{}' not found",
            collection_name
        ));
        handler.log_error(error.clone());
        error
    })?;

    collection.add_document(data, handler)
}

/// 🦀
/// Updates all documents in a given collection that match a specified field and value.
///
/// This service-level function locates the collection within the database,
/// then delegates the update operation to the collection logic. Each document
/// that matches `field_name == field_value` will be entirely replaced by `data` after validation.
///
/// # Parameters
///
/// - `db`: A mutable reference to the [`Database`] instance.
/// - `collection_name`: The name of the collection to target.
/// - `field_name`: The field name used for matching (e.g., `"email"` or `"profile.name"`).
/// - `field_value`: The target value to match.
/// - `data`: The new document content to apply to all matches (must be a valid JSON object).
/// - `handler`: The error handler for logging schema violations and collection/document lookup failures.
///
/// # Returns
///
/// - `Ok(())` if matching documents were found and successfully updated.
/// - `Err(NosqliteError)` if the collection does not exist, no documents match, or the new data is invalid.
///
/// # Example
///
/// ```rust
/// use serde_json::json;
/// use nosqlite_rust::engine::{error::{NosqliteErrorHandler, NosqliteError}, models::{Database, Collection}};
/// use nosqlite_rust::engine::services::document_service::update_documents;
///
/// let mut db = Database::new("temp/data23.nosqlite");
/// let mut handler = NosqliteErrorHandler::new("temp/data23.nosqlite".to_string());
/// db.add_collection("users", json!({}), &mut handler)?;
/// let docs = {
///     let col = db.get_collection_mut("users").unwrap();
///     col.add_document(json!({ "id": "abc123", "name": "Alice" }), &mut handler)?;
///     col.all_documents().clone()
/// };
/// let mut db_clone = db.clone();
/// update_documents(&mut db_clone, "users", "id", &json!("abc123"), json!({ "id": "xyz", "name": "Alice Updated" }), &mut handler)?;
/// Ok::<(), NosqliteError>(())
/// ```
///
/// # See Also
///
/// - [`update_documents_field`] — partial updates
/// - [`get_document_by_id`] — read after update
pub fn update_documents(
    db: &mut Database,
    collection_name: &str,
    field_name: &str,
    field_value: &Value,
    data: Value,
    handler: &mut NosqliteErrorHandler,
) -> Result<(), NosqliteError> {
    let collection = db.get_collection_mut(collection_name).ok_or_else(|| {
        let error = NosqliteError::CollectionNotFound(format!(
            "Collection '{}' not found",
            collection_name
        ));
        handler.log_error(error.clone());
        error
    })?;

    collection.update_documents(field_name, field_value, data, handler)
}

/// 🦀
/// Updates a specific field in all documents that match a given field and value.
///
/// This function performs a **partial update** by locating every document in the collection
/// where the specified `field_name` equals `field_value`, and setting or inserting
/// the `target_field` with the provided `value`.
///
/// # Parameters
///
/// - `db`: A mutable reference to the [`Database`] instance.
/// - `collection_name`: The name of the collection to target.
/// - `field_name`: The field name used to search documents (supports nested paths like `"profile.name"`).
/// - `field_value`: The value to match within `field_name`.
/// - `target_field`: The name of the field to modify or insert in matching documents.
/// - `value`: The new value to assign to `target_field`.
/// - `handler`: A mutable reference to a [`NosqliteErrorHandler`] for logging lookup or structure issues.
///
/// # Returns
///
/// - `Ok(())` if at least one document matched and was updated successfully.
/// - `Err(NosqliteError)` if:
///   - The collection does not exist,
///   - No document matched the search criteria,
///   - The matched document’s data is not a JSON object.
///
/// # Behavior
///
/// - If `target_field` exists in a matching document, it is overwritten with `value`.
/// - If `target_field` does not exist, it is created.
/// - All modified documents will have their `updated_at` timestamp refreshed.
/// - This function does **not** revalidate the updated documents against the collection schema.
///
/// # Example
///
/// ```rust
/// use serde_json::json;
/// use nosqlite_rust::engine::{error::{NosqliteErrorHandler, NosqliteError}, models::{Database, Collection}};
/// use nosqlite_rust::engine::services::document_service::update_documents_field;
///
/// let mut db = Database::new("temp/data24.nosqlite");
/// let mut handler = NosqliteErrorHandler::new("temp/data24.nosqlite".to_string());
/// db.add_collection("users", json!({
///     "id": "number",
///     "name": "string",
///     "email": "string"
/// }), &mut handler)?;
///
/// let col = db.get_collection_mut("users").unwrap();
/// col.add_document(json!({ "id": 1, "name": "Alice", "email": "old@example.com" }), &mut handler)?;
/// col.add_document(json!({ "id": 2, "name": "Alice", "email": "old2@example.com" }), &mut handler)?;
///
/// update_documents_field(&mut db, "users", "name", &json!("Alice"), "email", json!("new@example.com"), &mut handler)?;
/// Ok::<(), NosqliteError>(())
/// ```
///
/// # See Also
///
/// - [`update_documents`] — for full document replacement
/// - [`get_document_by_id`] — read after update
pub fn update_documents_field(
    db: &mut Database,
    collection_name: &str,
    field_name: &str,
    field_value: &Value,
    target_field: &str,
    value: Value,
    handler: &mut NosqliteErrorHandler,
) -> Result<(), NosqliteError> {
    let collection = db.get_collection_mut(collection_name).ok_or_else(|| {
        let error = NosqliteError::CollectionNotFound(format!(
            "Collection '{}' not found",
            collection_name
        ));
        handler.log_error(error.clone());
        error
    })?;

    collection.update_documents_field(field_name, field_value, target_field, value, handler)
}

/// 🦀
/// Deletes a document from a collection by its unique ID.
///
/// This function performs a linear scan of the collection to locate the document
/// by its `id` and removes it. If not found, logs and returns an error.
///
/// # Parameters
///
/// - `db`: A mutable reference to the [`Database`] instance.
/// - `collection_name`: The name of the target collection.
/// - `field_name`: The field name used to search for the document (e.g., `"id"`).
/// - `field_value`: The value to match against `field_name`.
/// - `handler`: Used to log document/collection errors.
///
/// # Returns
///
/// - `Ok(())` if the document was deleted
/// - `Err(NosqliteError::DocumentNotFound)` if not found
///
/// # Example
///
/// ```rust
/// use serde_json::json;
/// use nosqlite_rust::engine::{error::{NosqliteErrorHandler, NosqliteError}, models::{Collection, Database, Document, File}};
/// use nosqlite_rust::engine::services::document_service::delete_documents;
///
/// let mut db = Database::new("temp/data25.nosqlite");
/// let mut handler = NosqliteErrorHandler::new("temp/data25.nosqlite".to_string());
/// db.add_collection("users", json!({}), &mut handler)?;
/// let col = db.get_collection_mut("users").unwrap();
/// col.add_document(json!({ "id": "abc123", "name": "Alice" }), &mut handler)?;
///
/// delete_documents(&mut db, "users", "id", &json!("abc123"), &mut handler)?;
/// Ok::<(), NosqliteError>(())
/// ```
pub fn delete_documents(
    db: &mut Database,
    collection_name: &str,
    field_name: &str,
    field_value: &Value,
    handler: &mut NosqliteErrorHandler,
) -> Result<(), NosqliteError> {
    let collection = db.get_collection_mut(collection_name).ok_or_else(|| {
        let error = NosqliteError::CollectionNotFound(format!(
            "Collection '{}' not found",
            collection_name
        ));
        handler.log_error(error.clone());
        error
    })?;

    collection.delete_documents(field_name, field_value, handler)
}

/// 🦀
/// Retrieves a document by its ID from a specific collection.
///
/// Searches the target collection and returns a reference to the matching [`Document`],
/// or logs and returns an error if it does not exist.
///
/// # Parameters
///
/// - `db`: A reference to the [`Database`] instance.
/// - `collection_name`: The name of the target collection.
/// - `id`: The document's ID to locate.
/// - `handler`: Logs a lookup failure if not found.
///
/// # Returns
///
/// - `Ok(&Document)` if the document exists
/// - `Err(NosqliteError::DocumentNotFound)` otherwise
///
/// # Example
///
/// ```rust
/// use serde_json::json;
/// use nosqlite_rust::engine::{error::{NosqliteErrorHandler, NosqliteError}, models::{Database,Collection}};
/// use nosqlite_rust::engine::services::document_service::get_document_by_id;
///
/// let mut db = Database::new("temp/data26.nosqlite");
/// let mut handler = NosqliteErrorHandler::new("temp/data26.nosqlite".to_string());
/// db.add_collection("users", json!({}), &mut handler)?;
/// let docs = {
///     let col = db.get_collection_mut("users").unwrap();
///     col.add_document(json!({ "id": "abc123", "name": "Alice" }), &mut handler)?;
///     col.all_documents().clone()
/// };
///
/// let db_clone = db.clone();
/// get_document_by_id(&db_clone, "users", &docs[0].id, &mut handler)?;
/// Ok::<(), NosqliteError>(())
/// ```
///
/// # See Also
///
/// - [`get_all_documents`] — to inspect all documents
pub fn get_document_by_id<'a>(
    db: &'a Database,
    collection_name: &str,
    id: &str,
    handler: &mut NosqliteErrorHandler,
) -> Result<&'a Document, NosqliteError> {
    let collection = db.get_collection(collection_name).ok_or_else(|| {
        let error = NosqliteError::CollectionNotFound(format!(
            "Collection '{}' not found",
            collection_name
        ));
        handler.log_error(error.clone());
        error
    })?;

    collection.get_document(id).ok_or_else(|| {
        let error = NosqliteError::DocumentNotFound(format!("Document '{}' not found", id));
        handler.log_error(error.clone());
        error
    })
}

/// 🦀
/// Returns all documents stored in a specific collection.
///
/// This function retrieves the internal document vector of a collection for inspection,
/// listing, or iteration. If the collection does not exist, the error is logged.
///
/// # Parameters
///
/// - `db`: A reference to the [`Database`] instance.
/// - `collection_name`: The name of the collection to inspect.
/// - `handler`: Logs collection lookup failures.
///
/// # Returns
///
/// - `Ok(&Vec<Document>)` — if the collection exists
/// - `Err(NosqliteError::CollectionNotFound)` otherwise
///
/// # Example
///
/// ```rust
/// use serde_json::json;
/// use nosqlite_rust::engine::{error::{NosqliteErrorHandler, NosqliteError}, models::{Collection, Database}};
/// use nosqlite_rust::engine::services::document_service::{get_all_documents, insert_document};
///
/// let mut db = Database::new("temp/data27.nosqlite");
/// let mut handler = NosqliteErrorHandler::new("temp/data27.nosqlite".to_string());
/// db.add_collection("users", json!({}) ,&mut handler)?;
/// let col = db.get_collection_mut("users").unwrap();
/// col.add_document(json!({ "id": "abc123", "name": "Alice" }), &mut handler)?;
/// for doc in get_all_documents(&db, "users", &mut handler)? {
///     println!("{}", doc);
/// }
/// Ok::<(), NosqliteError>(())
/// ```
///
/// # See Also
///
/// - [`get_documents_by_field`] — for conditional filtering
pub fn get_all_documents<'a>(
    db: &'a Database,
    collection_name: &str,
    handler: &mut NosqliteErrorHandler,
) -> Result<&'a Vec<Document>, NosqliteError> {
    let collection = db.get_collection(collection_name).ok_or_else(|| {
        let error = NosqliteError::CollectionNotFound(format!(
            "Collection '{}' not found",
            collection_name
        ));
        handler.log_error(error.clone());
        error
    })?;

    Ok(collection.all_documents())
}

/// 🦀
/// Retrieves documents where a specific field equals the given value (string match).
///
/// This function performs a filter on all documents in the collection,
/// returning only those where the given field exists and equals the specified value.
///
/// # Parameters
///
/// - `db`: A reference to the [`Database`] instance.
/// - `collection_name`: The name of the collection to query.
/// - `field`: The field name to match against.
/// - `value`: The expected field value (as a string).
/// - `handler`: For logging collection lookup failures.
///
/// # Returns
///
/// - `Ok(Vec<&Document>)` with matching documents
/// - `Err(NosqliteError)` if the collection is not found
///
/// # Example
///
/// ```rust
/// use serde_json::json;
/// use nosqlite_rust::engine::{error::{NosqliteErrorHandler, NosqliteError}, models::{Database,Collection}};
/// use nosqlite_rust::engine::services::document_service::get_documents_by_field;
///
/// let mut db = Database::new("temp/data28.nosqlite");
/// let mut handler = NosqliteErrorHandler::new("temp/data28.nosqlite".to_string());
/// db.add_collection("posts", json!({}) ,&mut handler)?;
/// let col = db.get_collection_mut("posts").unwrap();
/// col.add_document(json!({ "id": "post1", "author": "alice" }), &mut handler)?;
/// col.add_document(json!({ "id": "post2", "author": "bob" }), &mut handler)?;
/// let results = get_documents_by_field(&db, "posts", "author", "alice", &mut handler)?;
/// println!("Found {} posts by Alice", results.len());
/// Ok::<(), NosqliteError>(())
/// ```
///
/// # Notes
///
/// - Only exact string matches are supported.
/// - No schema validation is enforced here.
///
/// # See Also
///
/// - [`get_all_documents`] — fetch all then filter manually
pub fn get_documents_by_field<'a>(
    db: &'a Database,
    collection_name: &str,
    field: &str,
    value: &str,
    handler: &mut NosqliteErrorHandler,
) -> Result<Vec<&'a Document>, NosqliteError> {
    let collection = db.get_collection(collection_name).ok_or_else(|| {
        let error = NosqliteError::CollectionNotFound(format!(
            "Collection '{}' not found",
            collection_name
        ));
        handler.log_error(error.clone());
        error
    })?;

    let result = collection
        .all_documents()
        .iter()
        .filter(|doc| doc.data.get(field).is_some_and(|v| v == value))
        .collect();

    Ok(result)
}
