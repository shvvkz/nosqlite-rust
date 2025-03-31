use crate::engine::models::database::model::Database;
use crate::engine::models::file::model::File;
use std::fs;
use std::path::Path;

/// Loads a database from disk at the given path, or creates a new one if it doesn't exist.
///
/// # Arguments
///
/// * `path` - The file path to load the database from.
///
/// # Returns
///
/// A [`Database`] instance, either loaded from the file or newly created.
pub fn load_or_create_database(path: &str) -> Database {
    File::load_or_create(path)
}

/// Saves the database to disk at the given path.
///
/// # Arguments
///
/// * `path` - The file path to write the database to.
/// * `db` - The [`Database`] instance to save.
pub fn save_database(path: &str, db: &Database) {
    File::save(path, db)
}
