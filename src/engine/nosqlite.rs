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
    /// Opens or creates a new NoSQLite database using the given path.
    pub fn open(path: &str) -> Result<Self, NosqliteError> {
        let mut error_handler = NosqliteErrorHandler::new(path.to_string());
        let db = File::load_or_create(path, &mut error_handler)?;

        Ok(Self {
            db,
            error_handler,
            path: path.to_string(),
        })
    }

    /// Creates a new collection in the database.
    pub fn create_collection(&mut self, name: &str, structure: Value) -> Result<(), NosqliteError> {
        let result = create_collection(&mut self.db, name, structure, &mut self.error_handler);
        if result.is_ok() {
            self.auto_save();
        }
        result
    }

    /// Deletes a collection from the database.
    pub fn delete_collection(&mut self, name: &str) -> Result<(), NosqliteError> {
        let result = delete_collection(&mut self.db, name, &mut self.error_handler);
        if result.is_ok() {
            self.auto_save();
        }
        result
    }

    /// Inserts a document into the specified collection.
    pub fn insert_document(&mut self, collection: &str, data: Value) -> Result<(), NosqliteError> {
        let result = insert_document(&mut self.db, collection, data, &mut self.error_handler);
        if result.is_ok() {
            self.auto_save();
        }
        result
    }

    /// Updates an entire document in the specified collection.
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

    /// Updates a single field of a document in the specified collection.
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

    /// Deletes a document from the specified collection.
    pub fn delete_document(&mut self, collection: &str, id: &str) -> Result<(), NosqliteError> {
        let result = delete_document(&mut self.db, collection, id, &mut self.error_handler);
        if result.is_ok() {
            self.auto_save();
        }
        result
    }

    /// Retrieves a document by ID.
    pub fn get_document_by_id(
        &mut self,
        collection: &str,
        id: &str,
    ) -> Result<&Document, NosqliteError> {
        get_document_by_id(&self.db, collection, id, &mut self.error_handler)
    }

    /// Retrieves all documents from a collection.
    pub fn get_all_documents(&mut self, collection: &str) -> Result<&Vec<Document>, NosqliteError> {
        get_all_documents(&self.db, collection, &mut self.error_handler)
    }

    /// Retrieves all documents where a field equals a specific value.
    pub fn get_documents_by_field(
        &mut self,
        collection: &str,
        field: &str,
        value: &str,
    ) -> Result<Vec<&Document>, NosqliteError> {
        get_documents_by_field(&self.db, collection, field, value, &mut self.error_handler)
    }

    /// Lists all collections in the database.
    pub fn list_collections(&self) -> Vec<&Collection> {
        list_collections(&self.db)
    }

    /// Internal function to persist changes automatically.
    fn auto_save(&mut self) -> Result<(), NosqliteError> {
        save_database(&self.path, &self.db, &mut self.error_handler)?;
        Ok(())
    }
}
