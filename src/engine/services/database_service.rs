use crate::engine::error::{NosqliteError, NosqliteErrorHandler};
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
pub fn load_or_create_database(
    path: &str,
    error_handler: &mut NosqliteErrorHandler,
) -> Result<Database, NosqliteError> {
    match File::load_or_create(path, error_handler) {
        Ok(db) => Ok(db),
        Err(e) => {
            error_handler.log_error(e.clone());
            Err(e)
        }
    }
}

/// Saves the database to disk at the given path.
///
/// # Arguments
///
/// * `path` - The file path to write the database to.
/// * `db` - The [`Database`] instance to save.
pub fn save_database(
    path: &str,
    db: &Database,
    error_handler: &mut NosqliteErrorHandler,
) -> Result<(), NosqliteError> {
    match File::save(path, db, error_handler) {
        Ok(_) => Ok(()),
        Err(e) => {
            error_handler.log_error(e.clone());
            Err(e)
        }
    }
}
