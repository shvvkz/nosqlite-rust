use crate::engine::error::{NosqliteError, NosqliteErrorHandler};
use crate::engine::models::database::model::Database;
use crate::engine::models::document::model::Document;
use serde_json::Value;

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
/// use nosqlite_rust::engine::{error::NosqliteErrorHandler, models::Database};
/// use nosqlite_rust::engine::services::document_service::insert_document;
///
/// let mut db = Database::new("db.nosqlite");
/// let mut handler = NosqliteErrorHandler::new("db.nosqlite".to_string());
/// insert_document(&mut db, "users", json!({ "id": 1, "name": "Alice" }), &mut handler)?;
/// ```
///
/// # See Also
///
/// - [`update_document`] — full document replacement
/// - [`get_document_by_id`] — lookup inserted document
///
/// ---  
///
/// 📝 Schema-safe, ID-tracked document insertion.
///
/// 🔨🤖🔧 Powered by Rust
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

/// Replaces an existing document’s entire content by its ID.
///
/// This method validates the new content against the collection schema,
/// and if successful, replaces the existing document's data and updates the timestamp.
///
/// # Parameters
///
/// - `db`: A mutable reference to the [`Database`] instance.
/// - `collection_name`: The name of the collection.
/// - `id`: The ID of the document to update.
/// - `data`: The new content (must be a valid JSON object).
/// - `handler`: Error handler for logging validation and lookup issues.
///
/// # Returns
///
/// - `Ok(())` if the update is successful
/// - `Err(NosqliteError)` if the document or collection is missing or invalid
///
/// # Example
///
/// ```rust
/// use serde_json::json;
/// use nosqlite_rust::engine::{error::NosqliteErrorHandler, models::Database};
/// use nosqlite_rust::engine::services::document_service::update_document;
///
/// let mut db = Database::new("db.nosqlite");
/// let mut handler = NosqliteErrorHandler::new("db.nosqlite".to_string());
/// update_document(&mut db, "users", "abc123", json!({ "id": 1, "name": "Alice Updated" }), &mut handler)?;
/// ```
///
/// # See Also
///
/// - [`update_document_field`] — partial updates
/// - [`get_document_by_id`] — read after update
///
/// ---  
///
/// ✏️ Full document overwrite with schema safety.
///
/// 🔨🤖🔧 Powered by Rust
pub fn update_document(
    db: &mut Database,
    collection_name: &str,
    id: &str,
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

    collection.update_document(id, data, handler)
}

/// Updates a single field in an existing document by its ID.
///
/// This function performs a partial update on a document by setting a specific field
/// to a new value. It does **not** revalidate the document against the schema.
///
/// # Parameters
///
/// - `db`: A mutable reference to the [`Database`] instance.
/// - `collection_name`: The name of the target collection.
/// - `id`: The ID of the document to modify.
/// - `field`: The field name to change.
/// - `value`: The new value for that field.
/// - `handler`: For logging document or collection errors.
///
/// # Returns
///
/// - `Ok(())` if the field is updated
/// - `Err(NosqliteError)` if the document is not found or not an object
///
/// # Example
///
/// ```rust
/// use serde_json::json;
/// use nosqlite_rust::engine::{error::NosqliteErrorHandler, models::Database};
/// use nosqlite_rust::engine::services::document_service::update_document_field;
///
/// let mut db = Database::new("db.nosqlite");
/// let mut handler = NosqliteErrorHandler::new("db.nosqlite".to_string());
/// update_document_field(&mut db, "users", "abc123", "email", json!("alice@example.com"), &mut handler)?;
/// ```
///
/// # See Also
///
/// - [`update_document`] — full document replacement
///
/// ---  
///
/// 🔧 Patch a single field without overwriting the document.
///
/// 🔨🤖🔧 Powered by Rust
pub fn update_document_field(
    db: &mut Database,
    collection_name: &str,
    id: &str,
    field: &str,
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

    collection.update_field_document(id, field, value, handler)
}

/// Deletes a document from a collection by its unique ID.
///
/// This function performs a linear scan of the collection to locate the document
/// by its `id` and removes it. If not found, logs and returns an error.
///
/// # Parameters
///
/// - `db`: A mutable reference to the [`Database`] instance.
/// - `collection_name`: The name of the target collection.
/// - `id`: The ID of the document to delete.
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
/// use nosqlite_rust::engine::{error::NosqliteErrorHandler, models::Database};
/// use nosqlite_rust::engine::services::document_service::delete_document;
///
/// let mut db = Database::new("db.nosqlite");
/// let mut handler = NosqliteErrorHandler::new("db.nosqlite".to_string());
/// delete_document(&mut db, "users", "abc123", &mut handler)?;
/// ```
///
/// ---  
///
/// 🗑️ Permanently remove a document from memory.
///
/// 🔨🤖🔧 Powered by Rust
pub fn delete_document(
    db: &mut Database,
    collection_name: &str,
    id: &str,
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

    collection.delete_document(id, handler)
}

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
/// use nosqlite_rust::engine::{error::NosqliteErrorHandler, models::Database};
/// use nosqlite_rust::engine::services::document_service::get_document_by_id;
///
/// let mut db = Database::new("db.nosqlite");
/// let mut handler = NosqliteErrorHandler::new("db.nosqlite".to_string());
/// let doc = get_document_by_id(&db, "users", "abc123", &mut handler)?;
/// println!("Doc: {}", doc);
/// ```
///
/// # See Also
///
/// - [`get_all_documents`] — to inspect all documents
///
/// ---  
///
/// 🔍 Targeted document retrieval by ID.
///
/// 🔨🤖🔧 Powered by Rust
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
/// use nosqlite_rust::engine::{error::NosqliteErrorHandler, models::Database};
/// use nosqlite_rust::engine::services::document_service::get_all_documents;
///
/// let mut db = Database::new("db.nosqlite");
/// let mut handler = NosqliteErrorHandler::new("db.nosqlite".to_string());
/// for doc in get_all_documents(&db, "users", &mut handler)? {
///     println!("{}", doc);
/// }
/// ```
///
/// # See Also
///
/// - [`get_documents_by_field`] — for conditional filtering
///
/// ---  
///
/// 📄 Dump every document in a collection — read-only.
///
/// 🔨🤖🔧 Powered by Rust
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
/// use nosqlite_rust::engine::{error::NosqliteErrorHandler, models::Database};
/// use nosqlite_rust::engine::services::document_service::get_documents_by_field;
///
/// let mut db = Database::new("db.nosqlite");
/// let mut handler = NosqliteErrorHandler::new("db.nosqlite".to_string());
/// let results = get_documents_by_field(&db, "posts", "author", "alice", &mut handler)?;
/// println!("Found {} posts by Alice", results.len());
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
///
/// ---  
///
/// 🔍 In-memory query by field value — fast and flexible.
///
/// 🔨🤖🔧 Powered by Rust
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
