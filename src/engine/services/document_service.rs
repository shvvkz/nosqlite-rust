use crate::engine::models::database::model::Database;
use crate::engine::models::document::model::Document;
use serde_json::Value;

/// Insère un document dans une collection donnée.
pub fn insert_document(
    db: &mut Database,
    collection_name: &str,
    data: Value,
) -> Result<(), String> {
    let collection = db
        .get_collection_mut(collection_name)
        .ok_or_else(|| format!("Collection '{}' not found", collection_name))?;

    collection.add_document(data)
}

/// Met à jour totalement un document existant dans une collection.
pub fn update_document(
    db: &mut Database,
    collection_name: &str,
    id: &str,
    data: Value,
) -> Result<(), String> {
    let collection = db
        .get_collection_mut(collection_name)
        .ok_or_else(|| format!("Collection '{}' not found", collection_name))?;

    collection.update_document(id, data)
}

/// Met à jour un champ spécifique d’un document existant.
pub fn update_document_field(
    db: &mut Database,
    collection_name: &str,
    id: &str,
    field: &str,
    value: Value,
) -> Result<(), String> {
    let collection = db
        .get_collection_mut(collection_name)
        .ok_or_else(|| format!("Collection '{}' not found", collection_name))?;

    collection.update_field_document(id, field, value)
}

/// Supprime un document d'une collection.
pub fn delete_document(db: &mut Database, collection_name: &str, id: &str) -> Result<(), String> {
    let collection = db
        .get_collection_mut(collection_name)
        .ok_or_else(|| format!("Collection '{}' not found", collection_name))?;

    collection.delete_document(id)
}

/// Récupère un document par son ID.
pub fn get_document_by_id<'a>(
    db: &'a Database,
    collection_name: &str,
    id: &str,
) -> Result<&'a Document, String> {
    let collection = db
        .get_collection(collection_name)
        .ok_or_else(|| format!("Collection '{}' not found", collection_name))?;

    collection
        .get_document(id)
        .ok_or_else(|| format!("Document with ID '{}' not found", id))
}

/// Récupère tous les documents d’une collection.
pub fn get_all_documents<'a>(
    db: &'a Database,
    collection_name: &str,
) -> Result<&'a Vec<Document>, String> {
    let collection = db
        .get_collection(collection_name)
        .ok_or_else(|| format!("Collection '{}' not found", collection_name))?;

    Ok(collection.all_documents())
}

/// Récupère tous les documents dont le champ `field` a la valeur `value`
pub fn get_documents_by_field<'a>(
    db: &'a Database,
    collection_name: &str,
    field: &str,
    value: &str,
) -> Result<Vec<&'a Document>, String> {
    let collection = db
        .get_collection(collection_name)
        .ok_or_else(|| format!("Collection '{}' not found", collection_name))?;

    let result = collection
        .all_documents()
        .iter()
        .filter(|doc| doc.data.get(field).map_or(false, |v| v == value))
        .collect();

    Ok(result)
}
