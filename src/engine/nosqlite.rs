use crate::engine::models::{Collection, Database, Document, File};

use crate::engine::services::{
    collection_service::*,
    database_service::{load_or_create_database, save_database},
    document_service::*,
};

use serde_json::Value;

use super::error::{NosqliteError, NosqliteErrorHandler};

/// Public API for interacting with the NoSQLite engine.
pub struct Nosqlite {
    path: String,
    error_handler: NosqliteErrorHandler,
    db: Database,
}

impl Nosqlite {
    /// Opens or initializes a new NoSQLite database from the given file path.
    ///
    /// This method performs a secure load operation from an encrypted `.nosqlite` file.
    /// If the file does not exist, a new empty [`Database`] is created and persisted.
    ///
    /// This is the primary entry point to start working with a database file.
    ///
    /// # Parameters
    ///
    /// - `path`: The path to the encrypted database file (e.g. `"data.nosqlite"`).
    ///
    /// # Returns
    ///
    /// - `Ok(Self)` — if the file is successfully read, decrypted, and deserialized
    /// - `Err(NosqliteError)` — if any part of the process fails (I/O, decryption, deserialization)
    ///
    /// # Example
    ///
    /// ```rust
    /// let db = Nosqlite::open("mydata.nosqlite")?;
    /// ```
    ///
    /// # Notes
    ///
    /// - The database is backed by AES-256-GCM encryption.
    /// - Automatically initializes a [`NosqliteErrorHandler`] for this instance.
    /// - Automatically logs and persists error info to `path.replace(".nosqlite", ".log")`
    ///
    /// # See Also
    ///
    /// - [`File::load_or_create`] — underlying logic
    /// - [`NosqliteErrorHandler`] — error handling system used
    ///
    /// ---  
    ///
    /// 📂 Securely open or initialize a NoSQL database from disk.
    ///
    /// 🔨🤖🔧 Powered by Rust
    pub fn open(path: &str) -> Result<Self, NosqliteError> {
        let mut error_handler = NosqliteErrorHandler::new(path.to_string());
        let db = File::load_or_create(path, &mut error_handler)?;

        Ok(Self {
            db,
            error_handler,
            path: path.to_string(),
        })
    }

    /// Creates a new collection within the current NoSQLite database.
    ///
    /// The collection will use the provided structure (schema) to validate all future documents.
    /// The structure must be a JSON object where each key maps to a type (`"string"`, `"number"`, etc.).
    ///
    /// If the operation succeeds, the database is automatically saved to disk.
    ///
    /// # Parameters
    ///
    /// - `name`: The unique name for the new collection.
    /// - `structure`: A [`serde_json::Value`] representing the schema for documents.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the collection is created successfully
    /// - `Err(NosqliteError)` if:
    ///   - A collection with the same name already exists
    ///   - The schema is invalid (not a JSON object)
    ///
    /// # Example
    ///
    /// ```rust
    /// db.create_collection("users", json!({ "id": "number", "name": "string" }))?;
    /// ```
    ///
    /// # Side Effects
    ///
    /// - Automatically calls `auto_save()` to persist changes after success.
    ///
    /// # See Also
    ///
    /// - [`delete_collection`] — for removing collections
    /// - [`insert_document`] — to begin populating the collection
    ///
    /// ---  
    ///
    /// 🆕 Register a new, schema-aware collection in your NoSQL file.
    ///
    /// 🔨🤖🔧 Powered by Rust
    pub fn create_collection(&mut self, name: &str, structure: Value) -> Result<(), NosqliteError> {
        let result = create_collection(&mut self.db, name, structure, &mut self.error_handler);
        if result.is_ok() {
            self.auto_save();
        }
        result
    }

    /// Deletes a collection from the current NoSQLite database.
    ///
    /// This operation removes the collection and all of its documents from memory.
    /// If the specified collection does not exist, an error is logged and returned.
    ///
    /// On successful deletion, the database is automatically saved to disk.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the collection to delete.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the collection was found and deleted
    /// - `Err(NosqliteError::CollectionNotFound)` if no matching collection exists
    ///
    /// # Example
    ///
    /// ```rust
    /// db.delete_collection("logs")?;
    /// ```
    ///
    /// # Side Effects
    ///
    /// - Automatically triggers `auto_save()` if successful
    /// - Logs errors via the embedded [`NosqliteErrorHandler`]
    ///
    /// # See Also
    ///
    /// - [`create_collection`] — for schema creation
    /// - [`list_collections`] — to inspect what exists
    ///
    /// ---  
    ///
    /// 🗑️ Full teardown of a collection and its documents.
    ///
    /// 🔨🤖🔧 Powered by Rust
    pub fn delete_collection(&mut self, name: &str) -> Result<(), NosqliteError> {
        let result = delete_collection(&mut self.db, name, &mut self.error_handler);
        if result.is_ok() {
            self.auto_save();
        }
        result
    }

    /// Inserts a new document into a specified collection in the database.
    ///
    /// The document will be validated against the collection's structure (schema).
    /// If valid, it is assigned a unique ID and timestamp metadata, then added to the collection.
    ///
    /// On success, the database is immediately persisted to disk via `auto_save()`.
    ///
    /// # Parameters
    ///
    /// - `collection`: The name of the target collection.
    /// - `data`: A [`serde_json::Value`] representing the document content. Must be a JSON object.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the document is valid and inserted successfully.
    /// - `Err(NosqliteError)` if the collection does not exist or the document fails schema validation.
    ///
    /// # Example
    ///
    /// ```rust
    /// db.insert_document("users", json!({ "id": 1, "name": "Alice" }))?;
    /// ```
    ///
    /// # See Also
    ///
    /// - [`update_document`] — for replacing an existing document
    /// - [`delete_document`] — for removing one by ID
    ///
    /// ---  
    ///
    /// 📥 Schema-safe insertion with UUID + timestamp support.
    ///
    /// 🔨🤖🔧 Powered by Rust
    pub fn insert_document(&mut self, collection: &str, data: Value) -> Result<(), NosqliteError> {
        let result = insert_document(&mut self.db, collection, data, &mut self.error_handler);
        if result.is_ok() {
            self.auto_save();
        }
        result
    }

    /// Replaces an entire document in a collection by its ID.
    ///
    /// This method fully overwrites the existing document’s content, while preserving its
    /// original `id`, and updating the `updated_at` timestamp.
    ///
    /// The new data is validated against the collection schema before the update is applied.
    /// After a successful update, the database is automatically saved to disk.
    ///
    /// # Parameters
    ///
    /// - `collection`: The name of the collection.
    /// - `id`: The unique identifier of the document to update.
    /// - `new_data`: A complete JSON object to replace the existing document data.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the document exists and the update passes validation.
    /// - `Err(NosqliteError)` if the document or collection is missing, or validation fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// db.update_document("users", "abc123", json!({ "id": 1, "name": "Updated" }))?;
    /// ```
    ///
    /// # See Also
    ///
    /// - [`update_document_field`] — for partial updates
    /// - [`insert_document`] — for adding new documents
    ///
    /// ---  
    ///
    /// 🔁 Full document replacement with structure validation and audit timestamps.
    ///
    /// 🔨🤖🔧 Powered by Rust
    pub fn update_document(
        &mut self,
        collection: &str,
        id: &str,
        new_data: Value,
    ) -> Result<(), NosqliteError> {
        let result = update_document(
            &mut self.db,
            collection,
            id,
            new_data,
            &mut self.error_handler,
        );
        if result.is_ok() {
            self.auto_save();
        }
        result
    }

    /// Updates a single field within a document in a given collection.
    ///
    /// This is a partial update operation. It updates only the specified field,
    /// leaving the rest of the document unchanged. Timestamps (`updated_at`) are refreshed automatically.
    ///
    /// No structure revalidation is performed on partial updates, so use with care.
    ///
    /// After a successful change, the database is auto-saved to disk.
    ///
    /// # Parameters
    ///
    /// - `collection`: The name of the collection.
    /// - `id`: The ID of the document to update.
    /// - `field`: The field key to set or replace.
    /// - `value`: The new value to assign to the field.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the update is successful.
    /// - `Err(NosqliteError)` if the document or collection is missing, or if the document data is not a JSON object.
    ///
    /// # Example
    ///
    /// ```rust
    /// db.update_document_field("users", "abc123", "email", json!("new@email.com"))?;
    /// ```
    ///
    /// # Notes
    ///
    /// - This does not validate the updated document against the schema after mutation.
    /// - Use [`update_document`] for strict schema-validated replacement.
    ///
    /// # See Also
    ///
    /// - [`get_document_by_id`] — for inspecting before or after
    /// - [`delete_document`] — for removing by ID
    ///
    /// ---  
    ///
    /// ✏️ Lightweight patching of document fields — fast and flexible.
    ///
    /// 🔨🤖🔧 Powered by Rust
    pub fn update_document_field(
        &mut self,
        collection: &str,
        id: &str,
        field: &str,
        value: Value,
    ) -> Result<(), NosqliteError> {
        let result = update_document_field(
            &mut self.db,
            collection,
            id,
            field,
            value,
            &mut self.error_handler,
        );
        if result.is_ok() {
            self.auto_save();
        }
        result
    }

    /// Deletes a document from a specified collection by its ID.
    ///
    /// This operation removes the document from memory if it exists, and updates the database file
    /// via `auto_save()` upon success. If the document or collection is missing, the error is logged.
    ///
    /// # Parameters
    ///
    /// - `collection`: The name of the collection.
    /// - `id`: The unique ID of the document to delete.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the document was successfully removed.
    /// - `Err(NosqliteError)` if the document or collection does not exist.
    ///
    /// # Example
    ///
    /// ```rust
    /// db.delete_document("users", "doc-123")?;
    /// ```
    ///
    /// # See Also
    ///
    /// - [`insert_document`] — to add new documents
    /// - [`get_document_by_id`] — for checking if a document exists before deletion
    ///
    /// ---  
    ///
    /// ❌ Remove a document and persist the change automatically.
    ///
    /// 🔨🤖🔧 Powered by Rust
    pub fn delete_document(&mut self, collection: &str, id: &str) -> Result<(), NosqliteError> {
        let result = delete_document(&mut self.db, collection, id, &mut self.error_handler);
        if result.is_ok() {
            self.auto_save();
        }
        result
    }

    /// Retrieves a document from a collection by its unique ID.
    ///
    /// If the collection and document exist, this returns an immutable reference to the document.
    /// Otherwise, the error is logged and returned.
    ///
    /// # Parameters
    ///
    /// - `collection`: The name of the collection.
    /// - `id`: The ID of the document to retrieve.
    ///
    /// # Returns
    ///
    /// - `Ok(&Document)` if found
    /// - `Err(NosqliteError)` if the collection or document is not found
    ///
    /// # Example
    ///
    /// ```rust
    /// let doc = db.get_document_by_id("users", "abc123")?;
    /// println!("Found doc: {}", doc);
    /// ```
    ///
    /// # See Also
    ///
    /// - [`get_all_documents`] — for bulk access
    /// - [`update_document`] — for full mutation
    ///
    /// ---  
    ///
    /// 🔍 Fetch a document by ID with full validation and error logging.
    ///
    /// 🔨🤖🔧 Powered by Rust
    pub fn get_document_by_id(
        &mut self,
        collection: &str,
        id: &str,
    ) -> Result<&Document, NosqliteError> {
        get_document_by_id(&self.db, collection, id, &mut self.error_handler)
    }

    /// Retrieves all documents stored in a specified collection.
    ///
    /// This function returns a reference to the entire in-memory vector of documents
    /// associated with a collection. If the collection is not found, logs and returns an error.
    ///
    /// # Parameters
    ///
    /// - `collection`: The name of the collection to inspect.
    ///
    /// # Returns
    ///
    /// - `Ok(&Vec<Document>)` if the collection exists
    /// - `Err(NosqliteError)` if not found
    ///
    /// # Example
    ///
    /// ```rust
    /// for doc in db.get_all_documents("users")? {
    ///     println!("{}", doc);
    /// }
    /// ```
    ///
    /// # See Also
    ///
    /// - [`get_documents_by_field`] — for filtering based on field values
    ///
    /// ---  
    ///
    /// 📄 Access all documents in a collection at once.
    ///
    /// 🔨🤖🔧 Powered by Rust
    pub fn get_all_documents(&mut self, collection: &str) -> Result<&Vec<Document>, NosqliteError> {
        get_all_documents(&self.db, collection, &mut self.error_handler)
    }

    /// Retrieves all documents in a collection where a specific field equals a given value.
    ///
    /// This function performs a linear scan of the collection and returns all documents
    /// whose JSON object contains a field equal to the provided string value.
    ///
    /// # Parameters
    ///
    /// - `collection`: The name of the collection to query.
    /// - `field`: The field key to match against.
    /// - `value`: The string value to compare against the document field (equality check).
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<&Document>)` with matching documents
    /// - `Err(NosqliteError)` if the collection is not found
    ///
    /// # Example
    ///
    /// ```rust
    /// let results = db.get_documents_by_field("posts", "author", "alice")?;
    /// for post in results {
    ///     println!("{}", post);
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// - Only supports string equality (`==`) comparisons.
    /// - Documents must be valid JSON objects with the specified field.
    ///
    /// # See Also
    ///
    /// - [`get_all_documents`] — for manual filtering
    ///
    /// ---  
    ///
    /// 🔍 Lightweight field-based filtering — fast and expressive.
    ///
    /// 🔨🤖🔧 Powered by Rust
    pub fn get_documents_by_field(
        &mut self,
        collection: &str,
        field: &str,
        value: &str,
    ) -> Result<Vec<&Document>, NosqliteError> {
        get_documents_by_field(&self.db, collection, field, value, &mut self.error_handler)
    }

    /// Lists all collections currently stored in the database.
    ///
    /// Returns an immutable reference to all [`Collection`]s registered in the system.
    /// Useful for introspection, UI display, or admin tools.
    ///
    /// # Returns
    ///
    /// - A `Vec<&Collection>` representing the list of known collections.
    ///
    /// # Example
    ///
    /// ```rust
    /// for col in db.list_collections() {
    ///     println!("Collection: {}", col.name);
    /// }
    /// ```
    ///
    /// # See Also
    ///
    /// - [`create_collection`] — to define a new collection
    /// - [`delete_collection`] — to remove one
    ///
    /// ---  
    ///
    /// 📚 Full view of all registered collections.
    ///
    /// 🔨🤖🔧 Powered by Rust
    pub fn list_collections(&self) -> Vec<&Collection> {
        list_collections(&self.db)
    }

    /// Persists the current in-memory database state to disk.
    ///
    /// This internal utility is automatically invoked after successful mutations
    /// (e.g., inserting, updating, or deleting documents or collections).
    ///
    /// It delegates to [`save_database`], using the path associated with the current instance
    /// and logs any encountered errors using the internal [`NosqliteErrorHandler`].
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the save completes successfully.
    /// - `Err(NosqliteError)` if serialization, encryption, or I/O fails.
    ///
    /// # Notes
    ///
    /// - This method should not be called directly in most cases — it's managed automatically.
    ///
    /// # See Also
    ///
    /// - [`save_database`] — internal implementation
    ///
    /// ---  
    ///
    /// 💾 Transparent, automatic persistence after every mutation.
    ///
    /// 🔐  
    /// 🔨🤖🔧 Powered by Rust
    fn auto_save(&mut self) -> Result<(), NosqliteError> {
        save_database(&self.path, &self.db, &mut self.error_handler)?;
        Ok(())
    }
}
