use crate::engine::models::{Collection, Database, Document, File};

use crate::engine::services::{
    collection_service::*,
    database_service::{load_or_create_database, save_database},
    document_service::*,
};

use serde_json::Value;

/// Public API for interacting with the NoSQLite engine.
pub struct Nosqlite {
    path: String,
    db: Database,
}

impl Nosqlite {
    /// Opens or creates a new NoSQLite database using the given path.
    pub fn open(path: &str) -> Self {
        let db = load_or_create_database(path);
        Self {
            path: path.to_string(),
            db,
        }
    }

    /// Creates a new collection in the database.
    pub fn create_collection(&mut self, name: &str, structure: Value) -> Result<(), String> {
        let result = create_collection(&mut self.db, name, structure);
        if result.is_ok() {
            self.auto_save();
        }
        result
    }

    /// Deletes a collection from the database.
    pub fn delete_collection(&mut self, name: &str) -> Result<(), String> {
        let result = delete_collection(&mut self.db, name);
        if result.is_ok() {
            self.auto_save();
        }
        result
    }

    /// Inserts a document into the specified collection.
    pub fn insert_document(&mut self, collection: &str, data: Value) -> Result<(), String> {
        let result = insert_document(&mut self.db, collection, data);
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
    ) -> Result<(), String> {
        let result = update_document(&mut self.db, collection, id, new_data);
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
    ) -> Result<(), String> {
        let result = update_document_field(&mut self.db, collection, id, field, value);
        if result.is_ok() {
            self.auto_save();
        }
        result
    }

    /// Deletes a document from the specified collection.
    pub fn delete_document(&mut self, collection: &str, id: &str) -> Result<(), String> {
        let result = delete_document(&mut self.db, collection, id);
        if result.is_ok() {
            self.auto_save();
        }
        result
    }

    /// Retrieves a document by ID.
    pub fn get_document_by_id(&self, collection: &str, id: &str) -> Result<&Document, String> {
        get_document_by_id(&self.db, collection, id)
    }

    /// Retrieves all documents from a collection.
    pub fn get_all_documents(&self, collection: &str) -> Result<&Vec<Document>, String> {
        get_all_documents(&self.db, collection)
    }

    /// Retrieves all documents where a field equals a specific value.
    pub fn get_documents_by_field(
        &self,
        collection: &str,
        field: &str,
        value: &str,
    ) -> Result<Vec<&Document>, String> {
        get_documents_by_field(&self.db, collection, field, value)
    }

    /// Lists all collections in the database.
    pub fn list_collections(&self) -> Vec<&Collection> {
        list_collections(&self.db)
    }

    /// Internal function to persist changes automatically.
    fn auto_save(&self) {
        save_database(&self.path, &self.db);
    }
}
