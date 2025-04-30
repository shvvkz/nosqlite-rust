use super::model::Database;
use crate::engine::error::{NosqliteError, NosqliteErrorHandler};
use crate::engine::models::collection::model::Collection;
use serde_json::Value;
use std::fmt::Display;

impl Database {
    /// 🦀
    /// Creates a new, empty [`Database`] instance with no collections.
    ///
    /// This constructor initializes an in-memory database structure, prepared to store
    /// and manage multiple named [`Collection`]s. It does **not** load any data from disk.
    ///
    /// # Parameters
    ///
    /// - `db_path`: A string slice representing the path or name of the database.
    ///   While currently unused in this implementation, it may be relevant for future
    ///   persistence features such as loading/saving collections from files.
    ///
    /// # Returns
    ///
    /// A new [`Database`] with:
    /// - An empty collection list (`collections`)
    ///
    /// # Example
    ///
    /// ```rust
    /// use nosqlite_rust::engine::models::Database;
    ///
    /// let db = Database::new("temp/data29.nosqlite");
    /// assert!(db.collections.is_empty());
    /// ```
    ///
    /// # Future Considerations
    ///
    /// - Persistence from `db_path` can be added later via I/O integration.
    /// - You may implement methods like `load()`, `save()`, or `open()` for full file-based support.
    ///
    /// # See Also
    ///
    /// - [`Collection`] — the primary unit of storage inside the database
    pub fn new(db_path: &str) -> Self {
        Database {
            collections: Vec::new(),
        }
    }

    /// 🦀
    /// Adds a new collection to the database with a given name and structure.
    ///
    /// This method performs the following:
    /// - Ensures the collection name is **unique** within the database
    /// - Validates that the provided `structure` is a JSON object (`Value::Object`)
    /// - Constructs a new [`Collection`] and adds it to the database
    /// - Logs any failure using the provided [`NosqliteErrorHandler`]
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the new collection. Must be unique among all existing collections.
    /// - `structure`: A [`serde_json::Value`] defining the schema for documents in the collection. Must be a JSON object.
    /// - `handler`: A mutable reference to a [`NosqliteErrorHandler`] for logging validation or duplication errors.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the collection was created and added successfully.
    /// - `Err(NosqliteError)` if:
    ///   - A collection with the same name already exists
    ///   - The `structure` is not a valid JSON object
    ///
    /// # Errors
    ///
    /// - [`NosqliteError::CollectionAlreadyExists`] if the name is already used
    /// - [`NosqliteError::DocumentInvalid`] if the provided schema is not a valid object
    ///
    /// # Example
    ///
    /// ```rust
    /// use serde_json::json;
    /// use nosqlite_rust::engine::error::NosqliteErrorHandler;
    /// use nosqlite_rust::engine::models::Database;
    ///
    /// let mut db = Database::new("temp/data37.nosqlite");
    /// let mut handler = NosqliteErrorHandler::new("temp/data37.nosqlite".to_string());
    ///
    /// let schema = json!({ "id": "number", "name": "string" });
    /// db.add_collection("users", schema, &mut handler).unwrap();
    ///
    /// assert_eq!(db.collections.len(), 1);
    /// ```
    ///
    /// # See Also
    ///
    /// - [`Collection::new`] — initializes a new collection
    /// - [`NosqliteError`] — for error types
    /// - [`NosqliteErrorHandler`] — for structured error logging
    pub fn add_collection(
        &mut self,
        name: &str,
        structure: Value,
        handler: &mut NosqliteErrorHandler,
    ) -> Result<(), NosqliteError> {
        if self.collections.iter().any(|c| c.name == name) {
            let error = NosqliteError::CollectionAlreadyExists(name.to_string());
            handler.log_error(error.clone());
            return Err(error);
        }
        if name.is_empty() {
            let error = NosqliteError::CollectionNameEmpty();
            handler.log_error(error.clone());
            return Err(error);
        }
        if !structure.is_object() {
            let error =
                NosqliteError::DocumentInvalid("The structure must be a JSON object".into());
            handler.log_error(error.clone());
            return Err(error);
        }

        let collection = Collection::new(name.to_string(), structure);
        self.collections.push(collection);
        Ok(())
    }

    /// 🦀
    /// Removes a collection from the database by its name.
    ///
    /// This method searches for a collection by its name and, if found, removes it from the
    /// database. If no collection with the given name exists, an error is returned and logged
    /// via the provided [`NosqliteErrorHandler`].
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the collection to remove.
    /// - `handler`: A mutable reference to a [`NosqliteErrorHandler`] used to record error cases.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the collection was found and successfully removed.
    /// - `Err(NosqliteError::CollectionNotFound)` if the collection does not exist.
    ///
    /// # Errors
    ///
    /// - [`NosqliteError::CollectionNotFound`] — occurs when no collection with the specified name is found.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nosqlite_rust::engine::models::Database;
    /// use nosqlite_rust::engine::error::NosqliteErrorHandler;
    ///
    /// let mut db = Database::new("temp/data38.nosqlite");
    /// let mut handler = NosqliteErrorHandler::new("temp/data38.nosqlite".to_string());
    ///
    /// db.add_collection("logs", serde_json::json!({}), &mut handler).unwrap();
    /// assert_eq!(db.collections.len(), 1);
    ///
    /// db.remove_collection("logs", &mut handler).unwrap();
    /// assert!(db.collections.is_empty());
    /// ```
    ///
    /// # Behavior
    ///
    /// - This is a destructive operation: once removed, the collection and all its documents are lost from memory.
    /// - Collection names are matched **exactly**, case-sensitive.
    ///
    /// # See Also
    ///
    /// - [`Database::add_collection`] — for creating new collections
    /// - [`NosqliteError`] — for all possible errors
    /// - [`NosqliteErrorHandler`] — for structured logging of collection-level issues
    pub fn remove_collection(
        &mut self,
        name: &str,
        handler: &mut NosqliteErrorHandler,
    ) -> Result<(), NosqliteError> {
        let index = self
            .collections
            .iter()
            .position(|c| c.name == name)
            .ok_or_else(|| {
                let error = NosqliteError::CollectionNotFound(name.to_string());
                handler.log_error(error.clone());
                error
            })?;

        self.collections.remove(index);
        Ok(())
    }

    /// 🦀
    /// Retrieves a reference to a collection by its name.
    ///
    /// This method performs a read-only search within the database's collections. If a collection
    /// with the specified name exists, a reference to it is returned. Otherwise, `None` is returned.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the collection to retrieve. Must match exactly (case-sensitive).
    ///
    /// # Returns
    ///
    /// - `Some(&Collection)` if a collection with the specified name exists.
    /// - `None` if no such collection is found.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nosqlite_rust::engine::models::Database;
    /// use nosqlite_rust::engine::error::NosqliteErrorHandler;
    ///
    /// let mut db = Database::new("temp/data39.nosqlite");
    /// let mut handler = NosqliteErrorHandler::new("temp/data39.nosqlite".to_string());
    ///
    /// db.add_collection("users", serde_json::json!({ "id": "number" }), &mut handler).unwrap();
    ///
    /// if let Some(collection) = db.get_collection("users") {
    ///     println!("Collection '{}' found with {} document(s).", collection.name, collection.document_count());
    /// } else {
    ///     println!("Collection not found.");
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// - Matching is case-sensitive: `"users"` and `"Users"` are considered different.
    /// - For mutable access, consider implementing a `get_collection_mut` variant.
    ///
    /// # Use Cases
    ///
    /// - Querying collection metadata
    /// - Accessing documents through collection-level methods
    /// - Verifying collection existence before performing write operations
    ///
    /// # See Also
    ///
    /// - [`Database::add_collection`] — to create a new collection
    /// - [`Database::remove_collection`] — to delete a collection
    /// - [`Collection`] — the structure returned by this method
    pub fn get_collection(&self, name: &str) -> Option<&Collection> {
        self.collections.iter().find(|c| c.name == name)
    }

    /// 🦀
    /// Retrieves a **mutable** reference to a collection by its name.
    ///
    /// This method searches the internal list of collections and returns a mutable reference
    /// to the matching [`Collection`], allowing in-place modifications such as:
    /// - Adding or removing documents
    /// - Updating existing documents
    /// - Mutating the collection's structure or metadata (if applicable)
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the collection to retrieve. Matching is case-sensitive.
    ///
    /// # Returns
    ///
    /// - `Some(&mut Collection)` if the collection exists
    /// - `None` if no collection with the given name is found
    ///
    /// # Example
    ///
    /// ```rust
    /// use nosqlite_rust::engine::models::Database;
    /// use nosqlite_rust::engine::error::NosqliteErrorHandler;
    ///
    /// let mut db = Database::new("temp/data40.nosqlite");
    /// let mut handler = NosqliteErrorHandler::new("temp/data40.nosqlite".to_string());
    ///
    /// db.add_collection("products", serde_json::json!({ "id": "number", "name": "string" }), &mut handler).unwrap();
    ///
    /// if let Some(collection) = db.get_collection_mut("products") {
    ///     let product = serde_json::json!({ "id": 1, "name": "Keyboard" });
    ///     collection.add_document(product, &mut handler).unwrap();
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// - This method grants direct mutable access. Use cautiously in multi-threaded contexts or shared references.
    /// - If you only need read access, use [`Database::get_collection`] instead.
    ///
    /// # Use Cases
    ///
    /// - Database-driven document mutations
    /// - Middleware-style logic (e.g. validation, transformation)
    /// - Admin tooling or command-line updates
    ///
    /// # See Also
    ///
    /// - [`Database::get_collection`] — immutable variant
    /// - [`Collection`] — the structure being returned
    pub fn get_collection_mut(&mut self, name: &str) -> Option<&mut Collection> {
        self.collections.iter_mut().find(|c| c.name == name)
    }
}

impl Display for Database {
    /// 🦀
    /// Formats the [`Database`] for display in a human-readable format.
    ///
    /// The output includes:
    /// - The total number of collections in the database
    /// - A list of each collection’s name
    ///
    /// This is useful for debugging, logging, CLI tools, and user-facing summaries.
    ///
    /// # Example Output
    ///
    /// ```text
    /// Database (3 collections):
    ///   - users
    ///   - products
    ///   - logs
    /// ```
    ///
    /// # Behavior
    ///
    /// - Output is printed line-by-line using [`std::fmt::Formatter`].
    /// - The order of collections reflects their insertion order.
    ///
    /// # Usage Example
    ///
    /// ```rust
    /// use nosqlite_rust::engine::models::Database;
    ///
    /// let db = Database::new("temp/data41.nosqlite");
    /// println!("{}", db); // Triggers this Display implementation
    /// ```
    ///
    /// # See Also
    ///
    /// - [`Collection`] — individual items being listed
    /// - [`std::fmt::Display`] — the trait being implemented
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Database ({} collections):", self.collections.len())?;
        for collection in &self.collections {
            writeln!(f, "  - {}", collection.name)?;
        }
        Ok(())
    }
}

impl Default for Database {
    /// 🦀
    /// Creates a new default [`Database`] instance with no collections.
    ///
    /// This method provides a zero-configuration entry point to create an empty database,
    /// using `"db.nosqlite"` as a placeholder path. It is ideal for use cases such as:
    /// - Testing and prototyping
    /// - Temporary in-memory databases
    /// - API defaults or fallbacks
    ///
    /// # Returns
    ///
    /// A new [`Database`] initialized with:
    /// - No collections
    /// - A placeholder path ("db.nosqlite") for future compatibility
    ///
    /// # Example
    ///
    /// ```rust
    /// use nosqlite_rust::engine::models::Database;
    ///
    /// let db = Database::default();
    /// assert!(db.collections.is_empty());
    /// println!("{}", db); // Database (0 collections)
    /// ```
    ///
    /// # Notes
    ///
    /// - The path `"db.nosqlite"` is not used internally for persistence (yet),
    ///   but can serve as a label for error logging or future I/O integration.
    /// - You may override this behavior by using [`Database::new`] with a custom path.
    ///
    /// # See Also
    ///
    /// - [`Database::new`] — for creating a database with a custom path
    /// - [`Default`] — the trait being implemented
    fn default() -> Self {
        Database::new("db.nosqlite")
    }
}
