use crate::engine::models::collection::model::Collection;
use crate::engine::models::database::model::Database;
use serde_json::Value;

/// Crée une nouvelle collection avec une structure de base.
pub fn create_collection(db: &mut Database, name: &str, structure: Value) -> Result<(), String> {
    db.add_collection(name, structure)
}

/// Supprime une collection de la base de données.
pub fn delete_collection(db: &mut Database, name: &str) -> Result<(), String> {
    db.remove_collection(name)
}

/// Récupère une collection (référence immuable).
pub fn get_collection<'a>(db: &'a Database, name: &str) -> Result<&'a Collection, String> {
    db.get_collection(name)
        .ok_or_else(|| format!("Collection '{}' not found", name))
}

/// Récupère une collection (référence mutable).
pub fn get_collection_mut<'a>(
    db: &'a mut Database,
    name: &str,
) -> Result<&'a mut Collection, String> {
    db.get_collection_mut(name)
        .ok_or_else(|| format!("Collection '{}' not found", name))
}

/// Renvoie toutes les collections de la base.
pub fn list_collections(db: &Database) -> Vec<&Collection> {
    db.collections.iter().collect()
}
