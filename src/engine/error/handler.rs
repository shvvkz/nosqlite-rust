use std::io::Write;

use serde::{Deserialize, Serialize};

use super::NosqliteError;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NosqliteErrorHandler {
    errors: Vec<NosqliteError>,
    db_path: String,
}

impl NosqliteErrorHandler {
    /// 🦀
    /// Creates a new [`NosqliteErrorHandler`] instance with an empty error log and associated database path.
    ///
    /// This handler is responsible for collecting, storing, and optionally reporting structured errors
    /// that occur during collection operations (such as validation, missing documents, or structure mismatches).
    ///
    /// # Parameters
    ///
    /// - `db_path`: A `String` representing the logical or physical path to the associated database.
    ///   This value can be used for contextual error reporting or file logging.
    ///
    /// # Returns
    ///
    /// A new instance of [`NosqliteErrorHandler`] with:
    /// - An empty error list (`errors`)
    /// - The provided `db_path` stored internally
    ///
    /// # Example
    ///
    /// ```rust
    /// use nosqlite_rust::engine::error::NosqliteErrorHandler;
    ///
    /// let handler = NosqliteErrorHandler::new("temp/data12.nosqlite".to_string());
    /// assert!(handler.all_errors().is_empty());
    /// ```
    ///
    /// # Use Cases
    ///
    /// - Error handling in schema-aware document stores
    /// - Diagnostic tools for NoSQL-like systems
    /// - Unit and integration tests capturing specific failure cases
    ///
    /// # See Also
    ///
    /// - [`NosqliteError`] — the error enum used by this handler
    /// - [`NosqliteErrorHandler::log_error`] — method for appending errors to the handler
    pub fn new(db_path: String) -> Self {
        Self {
            errors: Vec::new(),
            db_path,
        }
    }

    /// 🦀
    /// Logs an error into the handler, optionally persisting it for later retrieval or storage.
    ///
    /// This method performs two actions:
    /// 1. **Persists** the provided [`NosqliteError`] using the internal `persist_error` mechanism.
    /// 2. **Stores** the error in the internal `errors` list for in-memory tracking.
    ///
    /// This allows errors to be:
    /// - Programmatically inspected later (via `self.errors`)
    /// - Persisted to disk or external logs (depending on `persist_error`'s implementation)
    ///
    /// # Parameters
    ///
    /// - `error`: The [`NosqliteError`] to log. This typically originates from collection operations
    ///   such as schema validation failures, missing documents, or malformed input.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nosqlite_rust::engine::error::{NosqliteErrorHandler, NosqliteError};
    /// let mut handler = NosqliteErrorHandler::new("temp/data13.nosqlite".to_string());
    /// handler.log_error(NosqliteError::DocumentNotFound("abc123".to_string()));
    /// assert_eq!(handler.all_errors().len(), 1);
    /// ```
    ///
    /// # Side Effects
    ///
    /// - Calls `persist_error(&error)`, which may perform I/O or other logging operations.
    ///
    /// # See Also
    ///
    /// - [`NosqliteError`] — the error type being logged
    pub fn log_error(&mut self, error: NosqliteError) {
        let timestamp = chrono::Utc::now();
        self.persist_error(&error);
        self.errors.push(error);
    }

    /// 🦀
    /// Persists a given [`NosqliteError`] to a log file associated with the database.
    ///
    /// This method appends the error, along with a UTC timestamp, to a `.log` file whose name
    /// is derived from the handler's `db_path`. The log file is created if it does not already exist.
    ///
    /// # Log Format
    ///
    /// Each error is logged in the format:
    /// ```text
    /// [2025-04-10T18:03:12.456Z] DocumentNotFound("abc123")
    /// ```
    ///
    /// # File Path
    ///
    /// The log file is derived from the `db_path` by replacing the `.nosqlite` extension with `.log`.
    /// For example, `"data/users.nosqlite"` → `"data/users.log"`
    ///
    /// # Parameters
    ///
    /// - `error`: A reference to the [`NosqliteError`] that will be written to the log.
    ///
    /// # Panics
    ///
    /// This method will **panic** if the log file cannot be opened or created, with the message:
    /// `"Impossible d'ouvrir le fichier de log"`.
    ///
    /// # Side Effects
    ///
    /// - Appends a log entry to the target `.log` file
    /// - Performs file I/O operations
    ///
    /// # Notes
    ///
    /// - This method is intended to be used internally by [`NosqliteErrorHandler::log_error`].
    /// - Consider adding more robust error handling instead of a hard `expect()` in production environments.
    ///
    /// # See Also
    ///
    /// - [`NosqliteErrorHandler::log_error`] — logs and persists errors together
    /// - [`NosqliteError`] — the structured error type
    fn persist_error(&self, error: &NosqliteError) {
        let log_path = self.db_path.replace(".nosqlite", ".log");

        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)
            .expect("Impossible d'ouvrir le fichier de log");

        let timestamp = chrono::Utc::now();
        let entry = format!("[{}] {}\n", timestamp, error);

        let _ = file.write_all(entry.as_bytes());
    }

    /// 🦀
    /// Returns a reference to all errors logged by the handler so far.
    ///
    /// This method provides immutable access to the internal list of errors collected
    /// during document and collection operations. It is useful for diagnostics, reporting,
    /// and testing — allowing external systems to inspect what went wrong and when.
    ///
    /// # Returns
    ///
    /// - A slice (&[`NosqliteError`]) containing all previously logged errors in insertion order.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nosqlite_rust::engine::error::{NosqliteErrorHandler, NosqliteError};;
    ///
    /// let mut handler = NosqliteErrorHandler::new("temp/data14.nosqlite".to_string());
    /// handler.log_error(NosqliteError::DocumentNotFound("missing-id".to_string()));
    ///
    /// let errors = handler.all_errors();
    /// assert_eq!(errors.len(), 1);
    /// assert!(matches!(errors[0], NosqliteError::DocumentNotFound(_)));
    /// ```
    ///
    /// # Behavior
    ///
    /// - Errors are returned in the order they were logged (FIFO).
    /// - This method replaces direct access to the private `errors` field.
    /// - The returned slice is immutable — use [`NosqliteErrorHandler::log_error`] to add new entries.
    ///
    /// # Use Cases
    ///
    /// - Post-mortem inspection of recent operations
    /// - API endpoints that return structured error summaries
    /// - Testing error conditions and their propagation
    ///
    /// # See Also
    ///
    /// - [`NosqliteErrorHandler::log_error`] — for inserting new errors
    /// - [`NosqliteError`] — the error type stored and returned
    pub fn all_errors(&self) -> &[NosqliteError] {
        &self.errors
    }

    /// 🦀
    /// Wraps a fallible operation, logs any error using the handler, and returns a unified [`NosqliteError`].
    ///
    /// This utility method transforms a `Result<T, E>` into `Result<T, NosqliteError>`, using a provided closure
    /// to convert the original error type `E` into a [`NosqliteError`]. If the result is an error, it will be:
    /// - Transformed via the `wrap` closure
    /// - Logged using [`NosqliteErrorHandler::log_error`]
    /// - Returned to the caller as a `Result::Err`
    ///
    /// This method is especially useful for composing fallible operations in I/O, parsing, or collection logic,
    /// without repeating error logging boilerplate.
    ///
    /// # Type Parameters
    ///
    /// - `T`: The success type of the incoming result.
    /// - `E`: The error type to convert into a [`NosqliteError`].
    ///
    /// # Parameters
    ///
    /// - `result`: The `Result<T, E>` to evaluate.
    /// - `wrap`: A closure used to convert the error `E` into a [`NosqliteError`].
    ///
    /// # Returns
    ///
    /// - `Ok(T)` if the original result is successful.
    /// - `Err(NosqliteError)` if the original result is an error, with logging side-effects.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nosqlite_rust::engine::error::{NosqliteErrorHandler, NosqliteError};
    ///
    /// let result: Result<i32, std::io::Error> = Err(std::io::Error::new(std::io::ErrorKind::Other, "fail"));
    ///
    /// let mut handler = NosqliteErrorHandler::new("temp/data15.nosqlite".to_string());
    /// let mapped = handler.try_or_log(result, |e| NosqliteError::IoError(format!("IO failed: {}", e)));
    ///
    /// assert!(mapped.is_err());
    /// assert_eq!(handler.all_errors().len(), 1);
    /// ```
    ///
    /// # See Also
    ///
    /// - [`NosqliteErrorHandler::log_error`] — used internally to record the error
    /// - [`NosqliteError`] — the target error type
    pub fn try_or_log<T, E>(
        &mut self,
        result: Result<T, E>,
        wrap: impl FnOnce(E) -> NosqliteError,
    ) -> Result<T, NosqliteError> {
        result.map_err(|e| {
            let err = wrap(e);
            self.log_error(err.clone());
            err
        })
    }
}
