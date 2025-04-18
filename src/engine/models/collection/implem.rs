use super::model::Collection;
use crate::engine::error::{NosqliteError, NosqliteErrorHandler};
use crate::engine::models::document::model::Document;
use crate::engine::models::utils::{get_nested_value, now, validate_against_structure};
use serde_json::Value;
use std::fmt::Display;

impl Collection {
    /// 🦀
    /// Constructs a new, empty [`Collection`] with a specified name and document structure.
    ///
    /// This function initializes a new [`Collection`] instance, which is a container designed to
    /// hold a set of structured documents. The `structure` parameter defines the expected
    /// schema of each document within the collection, typically represented as a JSON object.
    ///
    /// # Parameters
    ///
    /// - `name`: A `String` specifying the name of the collection. This can be used for
    ///   identification or indexing purposes.
    /// - `structure`: A [`serde_json::Value`] representing the schema or blueprint of the
    ///   documents this collection will accept. While not enforced at runtime, this schema
    ///   can be used for validation or schema-driven logic.
    ///
    /// # Returns
    ///
    /// Returns a new instance of [`Collection`] with:
    /// - An empty `documents` vector
    /// - The provided `name`
    /// - The provided `structure`
    /// - A timestamp (`created_at`) set to the current time, retrieved via the [`now()`] function.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nosqlite_rust::engine::models::Collection;
    /// use serde_json::json;
    ///
    /// let schema = json!({
    ///     "id": "number",
    ///     "name": "string",
    ///     "email": "string"
    /// });
    /// let collection = Collection::new("users".to_string(), schema);
    /// assert_eq!(collection.name, "users");
    /// assert!(collection.documents.is_empty());
    /// ```
    ///
    /// # See Also
    /// - [`Collection`]: The main struct representing a collection of structured documents.
    /// - [`now()`]: A utility function to retrieve the current system timestamp.
    /// # Notes
    ///
    /// This constructor does **not** perform any schema validation. It is the caller's responsibility
    /// to ensure the `structure` provided is consistent with the intended document format.
    pub fn new(name: String, structure: Value) -> Self {
        Collection {
            name,
            structure,
            documents: Vec::new(),
            created_at: now(),
        }
    }

    /// 🦀
    /// Attempts to insert a new document into the collection after validating its structure.
    ///
    /// This method ensures that the incoming document (`data`) conforms to the collection’s
    /// predefined structure before adding it to the internal document list. It relies on a
    /// schema validation function to compare field types and structure consistency.
    ///
    /// # Parameters
    ///
    /// - `data`: A [`serde_json::Value`] representing the document to insert. This must be a
    ///   JSON object (i.e., `serde_json::Value::Object`). Fields are expected to align with
    ///   the schema defined in the collection's `structure`.
    ///
    /// - `handler`: A mutable reference to a [`NosqliteErrorHandler`] instance, which is used
    ///   to log any encountered validation or structural errors.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the document is successfully validated and inserted.
    /// - `Err(NosqliteError)` if any of the following conditions are met:
    ///   - The document is **not** a valid JSON object.
    ///   - The collection's `structure` is **not** a valid JSON object.
    ///   - The document’s structure **does not match** the expected schema.
    ///
    /// # Errors
    ///
    /// Returns [`NosqliteError::DocumentInvalid`] when:
    /// - The input is not a JSON object
    /// - The document’s fields/types do not match the collection's expected structure
    ///
    /// Returns [`NosqliteError::InvalidCollectionStructure`] when:
    /// - The collection's internal `structure` is not a JSON object
    ///
    /// # Example
    ///
    /// ```rust
    /// use serde_json::json;
    /// use nosqlite_rust::engine::models::Collection;
    /// use nosqlite_rust::engine::error::NosqliteErrorHandler;
    ///
    /// let schema = json!({ "id": "number", "title": "string" });
    /// let mut collection = Collection::new("posts".to_string(), schema);
    ///
    /// let valid_doc = json!({ "id": 1, "title": "Hello, world!" });
    /// let mut handler = NosqliteErrorHandler::new("temp/data30.nosqlite".to_string());
    ///
    /// assert!(collection.add_document(valid_doc, &mut handler).is_ok());
    ///
    /// let invalid_doc = json!({ "id": "wrong_type", "title": "Oops" });
    /// assert!(collection.add_document(invalid_doc, &mut handler).is_err());
    /// ```
    ///
    /// # Implementation Notes
    ///
    /// - Validation logic is delegated to [`validate_against_structure`], which checks field presence
    ///   and type compatibility.
    /// - Inserted documents are wrapped in the [`Document`] type before being pushed into `self.documents`.
    ///
    /// # See Also
    ///
    /// - [`validate_against_structure`] - Ensures the document matches the schema.
    /// - [`NosqliteErrorHandler`] - Logs and manages structured errors.
    /// - [`NosqliteError`] - Enum defining possible error cases for document operations.
    pub fn add_document(
        &mut self,
        data: Value,
        handler: &mut NosqliteErrorHandler,
    ) -> Result<(), NosqliteError> {
        if let Value::Object(ref doc_map) = data {
            if let Value::Object(expected_structure) = &self.structure {
                if !validate_against_structure(doc_map, expected_structure) {
                    let error = NosqliteError::DocumentInvalid(
                        "Document does not match the collection's structure".into(),
                    );
                    handler.log_error(error.clone());
                    return Err(error);
                }
            } else {
                let error = NosqliteError::InvalidCollectionStructure(
                    "Collection structure is not a valid JSON object".into(),
                );
                handler.log_error(error.clone());
                return Err(error);
            }
        } else {
            let error = NosqliteError::DocumentInvalid("Document must be a JSON object".into());
            handler.log_error(error.clone());
            return Err(error);
        }

        let document = Document::new(data);
        self.documents.push(document);
        Ok(())
    }

    /// Replaces the contents of all documents in the collection that match a specific field value.
    ///
    /// This method performs a **full update** for each matching document, overwriting their entire
    /// content with `new_data`, after verifying that the structure conforms to the collection's schema.
    ///
    /// # Parameters
    ///
    /// - `field_name`: The name of the field to match (supports nested paths like `"profile.age"`).
    /// - `field_value`: The target value to match against.
    /// - `new_data`: A [`serde_json::Value`] representing the new content for each matching document. Must be a valid JSON object.
    /// - `handler`: A mutable reference to a [`NosqliteErrorHandler`] for logging validation or lookup errors.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if at least one document was found and successfully updated.
    /// - `Err(NosqliteError)` if:
    ///   - No document matched the criteria,
    ///   - The new data does not conform to the collection schema,
    ///   - The data is not a JSON object.
    /// 
    /// # Errors
    /// 
    /// - [`NosqliteError::DocumentNotFound`] is returned if no document matches the criteria.
    /// - [`NosqliteError::DocumentInvalid`] is returned if the new data is not a JSON object or does not match the collection's structure.
    /// - [`NosqliteError::InvalidCollectionStructure`] is returned if the collection's structure is not a JSON object.
    ///
    /// # Behavior
    ///
    /// - All documents where `field_name == field_value` will be fully overwritten.
    /// - Each updated document receives a fresh `updated_at` timestamp.
    /// - Partial updates are not supported by this method.
    /// # Example
    ///
    /// ```rust
    /// use serde_json::json;
    /// use nosqlite_rust::engine::models::Collection;
    /// use nosqlite_rust::engine::error::NosqliteErrorHandler;
    ///
    /// let schema = json!({ "id": "number", "name": "string" });
    /// let mut collection = Collection::new("users".to_string(), schema);
    ///
    /// let mut handler = NosqliteErrorHandler::new("temp/data31.nosqlite".to_string());
    /// let original = json!({ "id": 1, "name": "Alice" });
    /// collection.add_document(original, &mut handler).unwrap();
    ///
    /// let updated = json!({ "id": 1, "name": "Alice Updated" });
    /// let doc_id = collection.documents[0].id.clone();
    ///
    /// collection.update_documents("id", &json!(1), updated, &mut handler).unwrap();
    /// ```
    ///
    /// # See Also
    ///
    /// - [`Collection::add_document`]: For inserting new documents into the collection.
    /// - [`validate_against_structure`]: Validates structural conformance.
    /// - [`NosqliteError`], [`NosqliteErrorHandler`], [`Document`]
    pub fn update_documents(
        &mut self,
        field_name: &str,
        field_value: &Value,
        new_data: Value,
        handler: &mut NosqliteErrorHandler,
    ) -> Result<(), NosqliteError> {
        // Vérifie que new_data est bien un objet JSON
        let doc_map = if let Value::Object(ref doc_map) = new_data {
            doc_map
        } else {
            let error = NosqliteError::DocumentInvalid("New data must be a JSON object".into());
            handler.log_error(error.clone());
            return Err(error);
        };
    
        // Vérifie que la structure correspond à celle de la collection
        if let Value::Object(expected_structure) = &self.structure {
            if !validate_against_structure(doc_map, expected_structure) {
                let error = NosqliteError::DocumentInvalid(
                    "New data does not match the collection's structure".into(),
                );
                handler.log_error(error.clone());
                return Err(error);
            }
        }
    
        // Trouve tous les documents correspondants
        let matching_indices: Vec<usize> = self
            .documents
            .iter()
            .enumerate()
            .filter_map(|(i, doc)| {
                if get_nested_value(&doc.data, field_name) == Some(field_value) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();
    
        if matching_indices.is_empty() {
            let error = NosqliteError::DocumentNotFound(format!(
                "No document found where '{}' == '{}'", field_name, field_value
            ));
            handler.log_error(error.clone());
            return Err(error);
        }
    
        // Met à jour tous les documents trouvés
        for index in matching_indices {
            let mut document = self.documents[index].clone();
            document.data = new_data.clone(); // Cloner car on modifie plusieurs documents
            document.updated_at = now();
            self.documents[index] = document;
        }
    
        Ok(())
    }
    

    /// 🦀
    /// Updates a single field within a document, identified by its unique ID.
    ///
    /// This method performs a **partial update** by setting the value of a specified field
    /// inside an existing document. It directly mutates the JSON object in place and records
    /// the update timestamp.
    ///
    /// # Parameters
    ///
    /// - `id`: A string slice identifying the document to be updated.
    /// - `field`: The name of the field to update or insert (if it doesn't already exist).
    /// - `value`: A [`serde_json::Value`] representing the new value for the given field.
    /// - `handler`: A mutable reference to a [`NosqliteErrorHandler`] used to track and log errors.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the document exists and the update was successful.
    /// - `Err(NosqliteError)` if:
    ///   - The document could not be found
    ///   - The document’s `data` is not a JSON object
    ///
    /// # Errors
    ///
    /// - [`NosqliteError::DocumentNotFound`] is returned if no document with the given ID is found.
    /// - [`NosqliteError::DocumentInvalid`] is returned if the internal `data` field is not a JSON object.
    ///
    /// # Example
    ///
    /// ```rust
    /// use serde_json::json;
    /// use nosqlite_rust::engine::error::NosqliteErrorHandler;
    /// use nosqlite_rust::engine::models::Collection;
    ///
    /// let schema = json!({ "id": "number", "name": "string", "age": "number" });
    /// let mut collection = Collection::new("users".to_string(), schema);
    ///
    /// let mut handler = NosqliteErrorHandler::new("temp/data32.nosqlite".to_string());
    /// collection.add_document(json!({ "id": 1, "name": "Alice", "age": 30 }), &mut handler).unwrap();
    ///
    /// let doc_id = collection.documents[0].id.clone();
    /// collection.update_field_document(&doc_id, "age", json!(31), &mut handler).unwrap();
    ///
    /// assert_eq!(collection.documents[0].data["age"], json!(31));
    /// ```
    ///
    /// # Behavior
    ///
    /// - If the field exists, its value is overwritten.
    /// - If the field does not exist, it is inserted.
    /// - The `updated_at` timestamp is refreshed using [`now()`].
    ///
    /// # Limitations
    ///
    /// - This method does **not** validate field types against the collection schema.
    ///   Use with caution if strict typing is required.
    ///
    /// # See Also
    ///
    /// - [`Collection::update_document`] for replacing an entire document
    /// - [`NosqliteErrorHandler`], [`NosqliteError`], [`Document`]
    pub fn update_field_document(
        &mut self,
        id: &str,
        field: &str,
        value: Value,
        handler: &mut NosqliteErrorHandler,
    ) -> Result<(), NosqliteError> {
        let position = self
            .documents
            .iter()
            .position(|d| d.id == id)
            .ok_or_else(|| {
                let error = NosqliteError::DocumentNotFound(id.to_string());
                handler.log_error(error.clone());
                error
            })?;

        if let Value::Object(ref mut doc_map) = self.documents[position].data {
            doc_map.insert(field.to_string(), value);
        } else {
            let error = NosqliteError::DocumentInvalid("Document data is not a JSON object".into());
            handler.log_error(error.clone());
            return Err(error);
        }

        self.documents[position].updated_at = now();
        Ok(())
    }

    /// 🦀
    /// Removes a document from the collection by its unique identifier.
    ///
    /// This method searches for a document by its `id` and removes it from the collection if found.
    /// It is a destructive operation — the document is permanently deleted from the in-memory store.
    ///
    /// # Parameters
    ///
    /// - `id`: A string slice (`&str`) representing the unique identifier of the document to remove.
    /// - `handler`: A mutable reference to a [`NosqliteErrorHandler`], used to log error cases such as "not found".
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the document was found and successfully deleted.
    /// - `Err(NosqliteError)` if the document could not be found.
    ///
    /// # Errors
    ///
    /// - [`NosqliteError::DocumentNotFound`] is returned if no document with the specified `id` exists in the collection.
    ///
    /// # Example
    ///
    /// ```rust
    /// use serde_json::json;
    /// use nosqlite_rust::engine::models::Collection;
    /// use nosqlite_rust::engine::error::NosqliteErrorHandler;
    ///
    /// let schema = json!({ "id": "number", "title": "string" });
    /// let mut collection = Collection::new("notes".to_string(), schema);
    ///
    /// let mut handler = NosqliteErrorHandler::new("temp/data33.nosqlite".to_string());
    /// collection.add_document(json!({ "id": 1, "title": "First note" }), &mut handler).unwrap();
    ///
    /// let doc_id = collection.documents[0].id.clone();
    /// collection.delete_document(&doc_id, &mut handler).unwrap();
    ///
    /// assert!(collection.documents.is_empty());
    /// ```
    ///
    /// # Behavior
    ///
    /// - Performs a linear search for the document by ID.
    /// - If found, the document is removed from the internal `documents` vector.
    /// - If not found, an error is returned and logged via the provided `handler`.
    ///
    /// # See Also
    ///
    /// - [`Collection::add_document`] for inserting documents
    /// - [`Collection::update_document`] for replacing document content
    /// - [`NosqliteErrorHandler`], [`NosqliteError`], [`Document`]
    pub fn delete_document(
        &mut self,
        id: &str,
        handler: &mut NosqliteErrorHandler,
    ) -> Result<(), NosqliteError> {
        let position = self
            .documents
            .iter()
            .position(|d| d.id == id)
            .ok_or_else(|| {
                let error = NosqliteError::DocumentNotFound(id.to_string());
                handler.log_error(error.clone());
                error
            })?;

        self.documents.remove(position);
        Ok(())
    }

    /// 🦀
    /// Retrieves a reference to a document from the collection by its unique identifier.
    ///
    /// This method performs a non-mutating lookup in the collection’s document list, searching
    /// for a document with the specified `id`. If found, a reference to the [`Document`] is returned.
    /// If no match is found, `None` is returned.
    ///
    /// # Parameters
    ///
    /// - `id`: A string slice (`&str`) representing the unique ID of the document to retrieve.
    ///
    /// # Returns
    ///
    /// - `Some(&Document)` if a document with the given ID exists in the collection.
    /// - `None` if no document matches the given ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use serde_json::json;
    /// use nosqlite_rust::engine::models::Collection;
    /// use nosqlite_rust::engine::error::NosqliteErrorHandler;
    ///
    /// let schema = json!({ "id": "number", "title": "string" });
    /// let mut collection = Collection::new("articles".to_string(), schema);
    /// let mut handler = NosqliteErrorHandler::new("temp/data34.nosqlite".to_string());
    ///
    /// let doc = json!({ "id": 1, "title": "Intro to Rust" });
    /// collection.add_document(doc, &mut handler).unwrap();
    ///
    /// let doc_id = collection.documents[0].id.clone();
    /// let result = collection.get_document(&doc_id);
    /// assert!(result.is_some());
    /// assert_eq!(result.unwrap().data["title"], json!("Intro to Rust"));
    /// ```
    ///
    /// # Performance
    ///
    /// - Performs a linear scan through the collection's internal document vector.
    /// - For large collections, consider indexing for more efficient lookups.
    ///
    /// # See Also
    ///
    /// - [`Collection::add_document`] for inserting new documents
    /// - [`Collection::delete_document`] for removing documents
    /// - [`Document`] for the structure being returned
    pub fn get_document(&self, id: &str) -> Option<&Document> {
        self.documents.iter().find(|d| d.id == id)
    }

    /// 🦀
    /// Returns a reference to all documents currently stored in the collection.
    ///
    /// This method provides read-only access to the internal list of documents held by the
    /// collection. It is useful for inspection, iteration, exporting, or reporting purposes.
    ///
    /// # Returns
    ///
    /// A reference to the internal vector of [`Document`] instances:
    /// `&Vec<Document>`
    ///
    /// # Example
    ///
    /// ```rust
    /// use serde_json::json;
    /// use nosqlite_rust::engine::models::Collection;
    /// use nosqlite_rust::engine::error::NosqliteErrorHandler;
    ///
    /// let schema = json!({ "id": "number", "name": "string" });
    /// let mut collection = Collection::new("people".to_string(), schema);
    /// let mut handler = NosqliteErrorHandler::new("temp/data35.nosqlite".to_string());
    ///
    /// collection.add_document(json!({ "id": 1, "name": "Alice" }), &mut handler).unwrap();
    /// collection.add_document(json!({ "id": 2, "name": "Bob" }), &mut handler).unwrap();
    ///
    /// let docs = collection.all_documents();
    /// assert_eq!(docs.len(), 2);
    /// for doc in docs {
    ///     println!("Document ID: {}", doc.id);
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// - The returned reference is read-only. To modify document contents, use methods like
    ///   [`Collection::update_document`], [`Collection::update_field_document`], or create a mutable variant.
    /// - The order of documents in the vector reflects the order of insertion, unless modified.
    ///
    /// # Performance
    ///
    /// - This is a zero-cost accessor; no allocation or transformation is performed.
    ///
    /// # See Also
    ///
    /// - [`Collection::get_document`] — for retrieving a single document by ID
    /// - [`Collection::add_document`] — for inserting documents
    /// - [`Document`] — the structure returned by this method
    pub fn all_documents(&self) -> &Vec<Document> {
        &self.documents
    }

    /// 🦀
    /// Returns the total number of documents stored in the collection.
    ///
    /// This method provides a quick way to determine the size of the collection. It simply
    /// returns the length of the internal documents vector.
    ///
    /// # Returns
    ///
    /// - A `usize` representing the number of documents currently in the collection.
    ///
    /// # Example
    ///
    /// ```rust
    /// use serde_json::json;
    /// use nosqlite_rust::engine::models::Collection;
    /// use nosqlite_rust::engine::error::NosqliteErrorHandler;
    ///
    /// let schema = json!({ "id": "number", "name": "string" });
    /// let mut collection = Collection::new("users".to_string(), schema);
    /// let mut handler = NosqliteErrorHandler::new("temp/data36.nosqlite".to_string());
    ///
    /// collection.add_document(json!({ "id": 1, "name": "Alice" }), &mut handler).unwrap();
    /// collection.add_document(json!({ "id": 2, "name": "Bob" }), &mut handler).unwrap();
    ///
    /// assert_eq!(collection.document_count(), 2);
    /// ```
    ///
    /// # Performance
    ///
    /// - Constant time (`O(1)`) operation; no iteration or allocation.
    ///
    /// # See Also
    ///
    /// - [`Collection::all_documents`] — for retrieving a reference to all stored documents
    /// - [`Collection::add_document`] — for inserting new documents
    /// - [`Collection::delete_document`] — for removing documents
    pub fn document_count(&self) -> usize {
        self.documents.len()
    }
}

impl Display for Collection {
    /// 🦀
    /// Formats the [`Collection`] for human-readable display.
    ///
    /// This implementation provides a clean, structured textual representation of the collection,
    /// including:
    /// - The collection's name
    /// - The expected document structure (as a JSON value)
    /// - A list of stored document IDs
    ///
    /// # Example Output
    ///
    /// ```text
    /// Collection 'users'
    ///   Required Structure: {"id":"number","name":"string"}
    ///   2 document(s):
    ///     - a1f3c2d9
    ///     - b7e1d5f0
    /// ```
    ///
    /// # Usage
    ///
    /// This formatting is ideal for:
    /// - CLI tools or REPLs displaying collections
    /// - Debug logs and diagnostics
    /// - Quick summaries in developer-facing tools
    ///
    /// # Returns
    ///
    /// Returns a [`std::fmt::Result`] indicating success or formatting failure.
    ///
    /// # See Also
    ///
    /// - [`std::fmt::Display`] — the Rust trait being implemented here
    /// - [`Collection`] — the data structure being formatted
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Collection '{}'", self.name)?;
        writeln!(f, "  Required Structure: {}", self.structure)?;
        writeln!(f, "  {} document(s):", self.documents.len())?;
        for doc in &self.documents {
            writeln!(f, "    - {}", doc.id)?;
        }
        Ok(())
    }
}

impl Default for Collection {
    /// 🦀
    /// Creates a default [`Collection`] instance with the name `"default"` and an empty structure.
    ///
    /// This implementation is useful for initializing a placeholder or fallback collection.
    /// The resulting collection:
    /// - Has the name `"default"`
    /// - Expects an empty JSON object as its schema (`{}`)
    /// - Contains no documents
    ///
    /// # Returns
    ///
    /// A [`Collection`] initialized with default values:
    /// - `name`: `"default"`
    /// - `structure`: `{}` (empty JSON object)
    /// - `documents`: empty vector
    /// - `created_at`: current system timestamp
    ///
    /// # Example
    ///
    /// ```rust
    /// use nosqlite_rust::engine::models::Collection;
    ///
    /// let default_collection = Collection::default();
    /// assert_eq!(default_collection.name, "default");
    /// assert!(default_collection.structure.is_object());
    /// assert!(default_collection.documents.is_empty());
    /// ```
    ///
    /// # Use Cases
    ///
    /// - Fallback collections
    /// - Simplified test setup
    /// - Placeholder values when working with generics
    ///
    /// # See Also
    ///
    /// - [`Collection::new`] — for custom initialization
    /// - [`serde_json::Value::Object`] — for creating JSON structures
    fn default() -> Self {
        Collection::new("default".to_string(), Value::Object(serde_json::Map::new()))
    }
}
