use crate::engine::models::database::model::Collection;
use crate::engine::models::document::model::Document;
use crate::engine::models::collection::json_utils::get_nested_value;
use serde_json::Value;

/// Inserts a new document into a specific collection.
///
/// # Arguments
///
/// * `collection` - A mutable reference to the [`Collection`] where the document resides.
/// * `data` - The JSON content of the document to insert.
///
/// # Errors
///
/// Returns an error if:
/// - The collection does not exist.
/// - The document does not match the collection's structure.
pub fn insert_document(collection: &mut Collection, data: Value) -> Result<(), String> {
    collection.add_document(data)
}

/// Fully replaces the content of an existing document by ID.
///
/// # Arguments
///
/// * `collection` - A mutable reference to the [`Collection`] where the document resides.
/// * `id` - The ID of the document to update.
/// * `data` - The new content of the document as JSON.
///
/// # Errors
///
/// Returns an error if:
/// - The collection or document does not exist.
/// - The new data does not match the collection's structure.
pub fn update_document_by_id(collection: &mut Collection, id: &str, data: Value) -> Result<(), String> {
    collection.update_document_by_id(id, data)
}

/// Updates a specific field of a document by a given field and value.
///
/// # Arguments
///
/// * `collection` - A mutable reference to the [`Collection`] where the document resides.
/// * `field` - The name of the field to match.
/// * `value` - The value of the field to match.
/// * `data` - The new content of the document as JSON.
///
/// # Errors
///
/// Returns an error if:
/// - The collection or document does not exist.
/// - The new data does not match the collection's structure.
/// - The field does not exist in the document.
/// - The value does not match the expected type.
/// - The document is not a JSON object.
/// - The field is not a string.
pub fn update_document_by_field(
    collection: &mut Collection,
    field: &str,
    value: &str,
    data: Value,
) -> Result<(), String> {
    collection.update_document_by_field(field, value, data)
}

/// Updates a specific field of a document by ID.
///
/// # Arguments
///
/// * `collection` - A mutable reference to the [`Collection`] where the document resides.
/// * `id` - The ID of the document to update.
/// * `field` - The name of the field to update.
/// * `value` - The new value for the field.
///
/// # Errors
///
/// Returns an error if:
/// - The collection or document does not exist.
/// - The document data is not a JSON object.
pub fn update_field_by_id(
    collection: &mut Collection,
    id: &str,
    field_path: &str,
    value: Value,
) -> Result<(), String> {
    collection.update_field_document_by_id(id, field_path, value)
}

/// Updates a specific field of a document by a given field and value.
///
/// # Arguments
///
/// * `collection` - A mutable reference to the [`Collection`] where the document resides.
/// * `search_field` - The name of the field to match.
/// * `search_value` - The value of the field to match.
/// * `field_path` - The path to the field to update.
/// * `value` - The new value for the field.
///
/// # Errors
///
/// Returns an error if:
/// - The collection or document does not exist.
/// - The document data is not a JSON object.
/// - The field does not exist in the document.
/// - The value does not match the expected type.
/// - The field is not a string.
pub fn update_field_by_field(
    collection: &mut Collection,
    search_field: &str,
    search_value: &str,
    field_path: &str,
    value: Value,
) -> Result<(), String> {
    collection.update_field_document_by_field(search_field, search_value, field_path, value)
}

/// Deletes a document from a collection by ID.
///
/// # Arguments
///
/// * `collection` - A mutable reference to the [`Collection`] where the document resides.
/// * `id` - The ID of the document to delete.
///
/// # Errors
///
/// Returns an error if:
/// - The collection or document does not exist.
pub fn delete_document_by_id(collection: &mut Collection, id: &str) -> Result<(), String> {
    collection.delete_document_by_id(id)
}

/// Deletes a document from a collection by a specific field and value.
///
/// # Arguments
///
/// * `collection` - A mutable reference to the [`Collection`] where the document resides.
/// * `field` - The name of the field to match.
/// * `value` - The expected value of the field (as a string).
///
/// # Errors
///
/// Returns an error if:
/// - The collection or document does not exist.
/// - The field does not exist in the document.
/// - The value does not match the expected type.
/// - The document is not a JSON object.
pub fn delete_document_by_field(
    collection: &mut Collection,
    field: &str,
    value: &str,
) -> Result<(), String> {
    collection.delete_document_by_field(field, value)
}

/// Retrieves a document by ID from a collection.
///
/// # Arguments
///
/// * `collection` - A mutable reference to the [`Collection`] where the document resides.
/// * `id` - The ID of the document to retrieve.
///
/// # Returns
///
/// A reference to the [`Document`] if found.
///
/// # Errors
///
/// Returns an error if the collection or document does not exist.
pub fn get_document_by_id<'a>(collection: &'a Collection, id: &str) -> Option<&'a Document> {
    collection.get_document_by_id(id)
}

/// Retrieves a document by a specific field and value.
///
/// # Arguments
///
/// * `collection` - A mutable reference to the [`Collection`] where the document resides.
/// * `field` - The name of the field to match.
/// * `value` - The expected value of the field (as a string).
///
/// # Returns
///
/// A reference to the [`Document`] if found.
pub fn get_document_by_field<'a>(
    collection: &'a Collection,
    field: &str,
    value: &str,
) -> Option<&'a Document> {
    collection.get_document_by_field(field, value)
}

/// Retrieves all documents within a collection.
///
/// # Arguments
///
/// * `collection` - A mutable reference to the [`Collection`] where the document resides.
///
/// # Returns
///
/// A reference to a vector of [`Document`]s.
///
/// # Errors
///
/// Returns an error if the collection does not exist.
pub fn get_all_documents(collection: &Collection) -> &Vec<Document> {
    collection.all_documents()
}

/// Retrieves all documents in a collection where a specific field matches a given value.
///
/// # Arguments
///
/// * `collection` - A mutable reference to the [`Collection`] where the document resides.
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
    collection: &'a Collection,
    field: &str,
    value: &str,
) -> Vec<&'a Document> {
    collection
        .all_documents()
        .iter()
        .filter(|doc| {
            get_nested_value(&doc.data, field)
                .is_some_and(|v| v == value)
        })
        .collect()
}
