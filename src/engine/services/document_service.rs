use crate::engine::error::{NosqliteError, NosqliteErrorHandler};
use crate::engine::models::database::model::Database;
use crate::engine::models::document::model::Document;
use serde_json::Value;

/// Inserts a new document into a specific collection.
///
/// # Arguments
///
/// * `db` - A mutable reference to the [`Database`] instance.
/// * `collection_name` - The name of the target collection.
/// * `data` - The JSON content of the document to insert.
///
/// # Errors
///
/// Returns an error if:
/// - The collection does not exist.
/// - The document does not match the collection's structure.
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

/// Fully replaces the content of an existing document by ID.
///
/// # Arguments
///
/// * `db` - A mutable reference to the [`Database`] instance.
/// * `collection_name` - The name of the target collection.
/// * `id` - The ID of the document to update.
/// * `data` - The new content of the document as JSON.
///
/// # Errors
///
/// Returns an error if:
/// - The collection or document does not exist.
/// - The new data does not match the collection's structure.
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

/// Updates a specific field of a document by ID.
///
/// # Arguments
///
/// * `db` - A mutable reference to the [`Database`] instance.
/// * `collection_name` - The name of the target collection.
/// * `id` - The ID of the document to update.
/// * `field` - The name of the field to update.
/// * `value` - The new value for the field.
///
/// # Errors
///
/// Returns an error if:
/// - The collection or document does not exist.
/// - The document data is not a JSON object.
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

/// Deletes a document from a collection by ID.
///
/// # Arguments
///
/// * `db` - A mutable reference to the [`Database`] instance.
/// * `collection_name` - The name of the target collection.
/// * `id` - The ID of the document to delete.
///
/// # Errors
///
/// Returns an error if:
/// - The collection or document does not exist.
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

/// Retrieves a document by ID from a collection.
///
/// # Arguments
///
/// * `db` - A reference to the [`Database`] instance.
/// * `collection_name` - The name of the target collection.
/// * `id` - The ID of the document to retrieve.
///
/// # Returns
///
/// A reference to the [`Document`] if found.
///
/// # Errors
///
/// Returns an error if the collection or document does not exist.
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

/// Retrieves all documents within a collection.
///
/// # Arguments
///
/// * `db` - A reference to the [`Database`] instance.
/// * `collection_name` - The name of the collection.
///
/// # Returns
///
/// A reference to a vector of [`Document`]s.
///
/// # Errors
///
/// Returns an error if the collection does not exist.
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

/// Retrieves all documents in a collection where a specific field matches a given value.
///
/// # Arguments
///
/// * `db` - A reference to the [`Database`] instance.
/// * `collection_name` - The name of the collection.
/// * `field` - The name of the field to match.
/// * `value` - The expected value of the field (as a string).
///
/// # Returns
///
/// A vector of references to [`Document`]s where the field matches the value.
///
/// # Errors
///
/// Returns an error if the collection does not exist.
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
