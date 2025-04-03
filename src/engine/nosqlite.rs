use crate::engine::models::{collection::model::Collection, database::model::Database, document::model::Document, file::model::File};
use crate::engine::services::{
    collection_service::*,
    document_service::*,
};

use serde_json::Value;

/// Objet central de l'API NoSQLite utilisable par la CLI et l'ORM.
pub struct Nosqlite {
    path: String,
    db: Database,
}

impl Nosqlite {
    /// Ouvre ou crée une nouvelle base NoSQLite à partir du chemin fourni.
    pub fn open(path: &str) -> Self {
        let db = File::load_or_create(path);
        Self {
            path: path.to_string(),
            db,
        }
    }

    /// Sauvegarde automatiquement la base de données.
    fn auto_save(&self) {
        File::save(&self.path, &self.db);
    }

    // ========================
    // === Collection CRUD ===
    // ========================

    pub fn create_collection(&mut self, name: &str, structure: Value) -> Result<(), String> {
        let result = create_collection(&mut self.db, name, structure);
        if result.is_ok() {
            self.auto_save();
        }
        result
    }

    pub fn delete_collection(&mut self, name: &str) -> Result<(), String> {
        let result = delete_collection(&mut self.db, name);
        if result.is_ok() {
            self.auto_save();
        }
        result
    }

    pub fn list_collections(&self) -> Vec<&Collection> {
        list_collections(&self.db)
    }

    // =======================
    // === Document CRUD  ===
    // =======================

    pub fn insert_document(&mut self, collection_name: &str, data: Value) -> Result<(), String> {
        let collection = get_collection_mut(&mut self.db, collection_name)?;
        let result = insert_document(collection, data);
        if result.is_ok() {
            self.auto_save();
        }
        result
    }

    pub fn update_document(
        &mut self,
        collection_name: &str,
        id: &str,
        new_data: Value,
    ) -> Result<(), String> {
        let collection = get_collection_mut(&mut self.db, collection_name)?;
        let result = update_document_by_id(collection, id, new_data);
        if result.is_ok() {
            self.auto_save();
        }
        result
    }

    pub fn update_document_by_field(
        &mut self,
        collection_name: &str,
        field: &str,
        value: &str,
        new_data: Value,
    ) -> Result<(), String> {
        let collection = get_collection_mut(&mut self.db, collection_name)?;
        let result = update_document_by_field(collection, field, value, new_data);
        if result.is_ok() {
            self.auto_save();
        }
        result
    }

    pub fn update_field_by_id(
        &mut self,
        collection_name: &str,
        id: &str,
        field_path: &str,
        value: Value,
    ) -> Result<(), String> {
        let collection = get_collection_mut(&mut self.db, collection_name)?;
        let result = update_field_by_id(collection, id, field_path, value);
        if result.is_ok() {
            self.auto_save();
        }
        result
    }

    pub fn update_field_by_field(
        &mut self,
        collection_name: &str,
        search_field: &str,
        search_value: &str,
        field_path: &str,
        value: Value,
    ) -> Result<(), String> {
        let collection = get_collection_mut(&mut self.db, collection_name)?;
        let result =
            update_field_by_field(collection, search_field, search_value, field_path, value);
        if result.is_ok() {
            self.auto_save();
        }
        result
    }

    pub fn delete_document_by_id(
        &mut self,
        collection_name: &str,
        id: &str,
    ) -> Result<(), String> {
        let collection = get_collection_mut(&mut self.db, collection_name)?;
        let result = delete_document_by_id(collection, id);
        if result.is_ok() {
            self.auto_save();
        }
        result
    }

    pub fn delete_document_by_field(
        &mut self,
        collection_name: &str,
        field: &str,
        value: &str,
    ) -> Result<(), String> {
        let collection = get_collection_mut(&mut self.db, collection_name)?;
        let result = delete_document_by_field(collection, field, value);
        if result.is_ok() {
            self.auto_save();
        }
        result
    }

    pub fn get_document_by_id(
        &self,
        collection_name: &str,
        id: &str,
    ) -> Result<&Document, String> {
        let collection = get_collection(&self.db, collection_name)?;
        Ok(get_document_by_id(collection, id).unwrap())
    }

    pub fn get_document_by_field(
        &self,
        collection_name: &str,
        field: &str,
        value: &str,
    ) -> Option<&Document> {
        let collection = get_collection(&self.db, collection_name).ok()?;
        get_document_by_field(collection, field, value)
    }

    pub fn get_all_documents(&self, collection_name: &str) -> Result<&Vec<Document>, String> {
        let collection = get_collection(&self.db, collection_name)?;
        Ok(get_all_documents(collection))
    }

    pub fn get_documents_by_field(
        &self,
        collection_name: &str,
        field: &str,
        value: &str,
    ) -> Result<Vec<&Document>, String> {
        let collection = get_collection(&self.db, collection_name)?;
        Ok(get_documents_by_field(collection, field, value))
    }
}
