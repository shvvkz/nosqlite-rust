use crate::engine::models::collection::model::Collection;
use crate::engine::models::database::model::Database;
use serde_json::Value;

/// Creates a new collection with a specified structure and adds it to the database.
///
/// # Arguments
///
/// * `db` - A mutable reference to the [`Database`] instance.
/// * `name` - The name of the new collection.
/// * `structure` - A JSON object representing the expected structure of the documents.
///
/// # Errors
///
/// Returns an error if a collection with the same name already exists or if the structure is invalid.
pub fn create_collection(db: &mut Database, name: &str, structure: Value) -> Result<(), String> {
    db.add_collection(name, structure)
}

/// Removes a collection from the database by name.
///
/// # Arguments
///
/// * `db` - A mutable reference to the [`Database`] instance.
/// * `name` - The name of the collection to delete.
///
/// # Errors
///
/// Returns an error if the collection does not exist.
pub fn delete_collection(db: &mut Database, name: &str) -> Result<(), String> {
    db.remove_collection(name)
}

/// Retrieves an immutable reference to a collection by name.
///
/// # Arguments
///
/// * `db` - A reference to the [`Database`] instance.
/// * `name` - The name of the collection to retrieve.
///
/// # Returns
///
/// A reference to the [`Collection`] if found, or an error if it does not exist.
pub fn get_collection<'a>(db: &'a Database, name: &str) -> Result<&'a Collection, String> {
    db.get_collection(name)
        .ok_or_else(|| format!("Collection '{}' not found", name))
}

/// Retrieves a mutable reference to a collection by name.
///
/// # Arguments
///
/// * `db` - A mutable reference to the [`Database`] instance.
/// * `name` - The name of the collection to retrieve.
///
/// # Returns
///
/// A mutable reference to the [`Collection`] if found, or an error if it does not exist.
pub fn get_collection_mut<'a>(
    db: &'a mut Database,
    name: &str,
) -> Result<&'a mut Collection, String> {
    db.get_collection_mut(name)
        .ok_or_else(|| format!("Collection '{}' not found", name))
}

/// Lists all collections stored in the database.
///
/// # Arguments
///
/// * `db` - A reference to the [`Database`] instance.
///
/// # Returns
///
/// A vector of references to all [`Collection`]s in the database.
pub fn list_collections(db: &Database) -> Vec<&Collection> {
    db.collections.iter().collect()
}
