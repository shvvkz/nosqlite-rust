use crate::engine::error::{NosqliteError, NosqliteErrorHandler};
use crate::engine::models::database::model::Database;
use crate::engine::models::file::model::File;
use std::fs;
use std::path::Path;

/// 🦀
/// Loads a [`Database`] instance from disk, or creates a new one if the file doesn't exist.
///
/// This high-level helper wraps the internal [`File::load_or_create`] function, providing a convenient
/// way to bootstrap your NoSQL engine from persistent storage. If the target file:
///
/// - **Exists**:
///   - Attempts to read and decrypt it
///   - Deserializes the contents into a [`Database`] instance
///
/// - **Does not exist**:
///   - Creates a new empty [`Database`] with the specified path
///
/// Any encountered errors are logged using the provided [`NosqliteErrorHandler`] and returned.
///
/// # Parameters
///
/// - `path`: The file path to the encrypted `.nosqlite` database file.
/// - `error_handler`: A mutable reference to a [`NosqliteErrorHandler`] for structured logging.
///
/// # Returns
///
/// - `Ok(Database)` if the file is successfully read and parsed, or a new instance is created
/// - `Err(NosqliteError)` if the file cannot be read, decrypted, or deserialized
///
/// # Example
///
/// ```rust
/// use nosqlite_rust::engine::models::database::model::Database;
/// use nosqlite_rust::engine::error::{NosqliteErrorHandler, NosqliteError};
/// use nosqlite_rust::engine::services::database_service::load_or_create_database;
///
/// let mut handler = NosqliteErrorHandler::new("temp/data20.nosqlite".to_string());
/// let db = load_or_create_database("temp/data20.nosqlite", &mut handler)?;
/// println!("{}", db);
/// Ok::<(), NosqliteError>(())
/// ```
///
/// # Notes
///
/// - This function handles decryption, format validation, and automatic creation in one call.
/// - Uses internal AES-256-GCM encryption via the [`File`] module.
///
/// # See Also
///
/// - [`save_database`] — for persisting the database back to disk
/// - [`File::load_or_create`] — internal loader implementation
/// - [`NosqliteError`] — structured error type
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

/// 🦀
/// Saves the current state of the database to disk at the specified path.
///
/// This function wraps [`File::save`] to serialize and securely encrypt the [`Database`] contents,
/// then write them to the filesystem. Any errors during serialization, encryption, or I/O
/// are logged using the provided error handler.
///
/// # Parameters
///
/// - `path`: The file path to write the encrypted database to.
/// - `db`: A reference to the [`Database`] instance to be saved.
/// - `error_handler`: A mutable reference to a [`NosqliteErrorHandler`] for logging any I/O or serialization errors.
///
/// # Returns
///
/// - `Ok(())` if the database is successfully saved
/// - `Err(NosqliteError)` if serialization, encryption, or file write fails
///
/// # Example
///
/// ```rust
/// use nosqlite_rust::engine::models::database::model::Database;
/// use nosqlite_rust::engine::error::{NosqliteErrorHandler, NosqliteError};
/// use nosqlite_rust::engine::services::database_service::save_database;
///
/// let db = Database::new("temp/data21.nosqlite");
/// let mut handler = NosqliteErrorHandler::new("temp/data21.nosqlite".to_string());
/// save_database("temp/data21.nosqlite", &db, &mut handler)?;
/// println!("Database saved successfully.");
/// Ok::<(), NosqliteError>(())
/// ```
///
/// # Notes
///
/// - Uses AES-256-GCM encryption via the [`File`] module.
/// - Existing files will be **overwritten**. Make sure to backup if needed.
///
/// # See Also
///
/// - [`load_or_create_database`] — for the reverse operation
/// - [`File::save`] — internal save logic
/// - [`NosqliteError`] — error enum
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
