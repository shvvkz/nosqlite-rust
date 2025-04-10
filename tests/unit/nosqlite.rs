#[cfg(test)]
mod tests {
    use nosqlite_rust::engine::Nosqlite;
    use serde_json::json;

    #[test]
    fn open_should_succeed() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let db_path = tmp_dir.path().join("db_open_test.nosqlite");
        let db_path_str = db_path.to_str().unwrap();
    
        // Should create a new Nosqlite instance (and file) without error
        let result = Nosqlite::open(db_path_str);
        assert!(result.is_ok());
    }
    
    #[test]
    fn open_should_fail() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let db_path = tmp_dir.path().join("corrupted.nosqlite");
        let db_path_str = db_path.to_str().unwrap();
    
        // On écrit un contenu totalement invalide (ni base64 ni JSON)
        std::fs::write(&db_path, "invalid-encrypted-content").unwrap();
    
        let result = Nosqlite::open(db_path_str);
        assert!(result.is_err());
    }
    
    #[test]
    fn create_collection_should_succeed() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let db_path = tmp_dir.path().join("test_create.nosqlite");
        let db_path_str = db_path.to_str().unwrap();
    
        let mut nosqlite = Nosqlite::open(db_path_str).unwrap();
        let schema = json!({ "username": "string", "age": "number" });
    
        let result = nosqlite.create_collection("users", schema);
        assert!(result.is_ok());
    
        let collections = nosqlite.list_collections();
        assert_eq!(collections.len(), 1);
        assert_eq!(collections[0].name, "users");
    }
    
    #[test]
    fn create_collection_should_fail() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let db_path = tmp_dir.path().join("fail_create.nosqlite");
        let db_path_str = db_path.to_str().unwrap();
    
        let mut db = Nosqlite::open(db_path_str).unwrap();
        let schema = json!({ "field": "string" });
    
        // Première création réussie
        assert!(db.create_collection("dup", schema.clone()).is_ok());
    
        // Deuxième création avec le même nom doit échouer
        let result = db.create_collection("dup", schema);
        assert!(result.is_err());
    }
    
    #[test]
    fn delete_collection_should_succeed() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let db_path = tmp_dir.path().join("delete_success.nosqlite");
        let db_path_str = db_path.to_str().unwrap();
    
        let mut db = Nosqlite::open(db_path_str).unwrap();
        db.create_collection("temp", json!({ "key": "string" })).unwrap();
    
        let result = db.delete_collection("temp");
        assert!(result.is_ok());
    
        let list = db.list_collections();
        assert!(list.iter().all(|c| c.name != "temp"));
    }
    
    #[test]
    fn delete_collection_should_fail() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let db_path = tmp_dir.path().join("delete_fail.nosqlite");
        let db_path_str = db_path.to_str().unwrap();
    
        let mut db = Nosqlite::open(db_path_str).unwrap();
    
        let result = db.delete_collection("nonexistent");
        assert!(result.is_err());
    }
    

    #[test]
    fn insert_document_should_succeed() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let db_path = tmp_dir.path().join("insert_ok.nosqlite");
        let db_path_str = db_path.to_str().unwrap();
    
        let mut db = Nosqlite::open(db_path_str).unwrap();
        db.create_collection("users", json!({ "name": "string" })).unwrap();
    
        let result = db.insert_document("users", json!({ "name": "Alice" }));
        assert!(result.is_ok());
    
        let docs = db.get_all_documents("users").unwrap();
        assert_eq!(docs.len(), 1);
        assert_eq!(docs[0].data["name"], "Alice");
    }
    
    #[test]
    fn insert_document_should_fail() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let db_path = tmp_dir.path().join("insert_fail.nosqlite");
        let db_path_str = db_path.to_str().unwrap();
    
        let mut db = Nosqlite::open(db_path_str).unwrap();
    
        // Pas de collection créée
        let result = db.insert_document("ghosts", json!({ "name": "Nobody" }));
        assert!(result.is_err());
    }
    
    #[test]
    fn update_document_should_succeed() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let db_path = tmp_dir.path().join("update_ok.nosqlite");
        let db_path_str = db_path.to_str().unwrap();
    
        let mut db = Nosqlite::open(db_path_str).unwrap();
        db.create_collection("users", json!({ "name": "string" })).unwrap();
        db.insert_document("users", json!({ "name": "Original" })).unwrap();
    
        let doc_id = db.get_all_documents("users").unwrap()[0].id.clone();
        let result = db.update_document("users", &doc_id, json!({ "name": "Updated" }));
    
        assert!(result.is_ok());
        let doc = db.get_document_by_id("users", &doc_id).unwrap();
        assert_eq!(doc.data["name"], "Updated");
    }
    
    #[test]
    fn update_document_should_fail() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let db_path = tmp_dir.path().join("update_fail.nosqlite");
        let db_path_str = db_path.to_str().unwrap();
    
        let mut db = Nosqlite::open(db_path_str).unwrap();
        db.create_collection("users", json!({ "name": "string" })).unwrap();
    
        // ID invalide
        let result = db.update_document("users", "invalid-id", json!({ "name": "Updated" }));
        assert!(result.is_err());
    }
    
    #[test]
    fn update_document_field_should_succeed() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let db_path = tmp_dir.path().join("update_field_ok.nosqlite");
        let db_path_str = db_path.to_str().unwrap();
    
        let mut db = Nosqlite::open(db_path_str).unwrap();
        db.create_collection("users", json!({ "name": "string" })).unwrap();
        db.insert_document("users", json!({ "name": "Old" })).unwrap();
    
        let doc_id = db.get_all_documents("users").unwrap()[0].id.clone();
        let result = db.update_document_field("users", &doc_id, "name", json!("New"));
    
        assert!(result.is_ok());
        let updated = db.get_document_by_id("users", &doc_id).unwrap();
        assert_eq!(updated.data["name"], "New");
    }
    
    #[test]
    fn update_document_field_should_fail() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let db_path = tmp_dir.path().join("update_field_fail.nosqlite");
        let db_path_str = db_path.to_str().unwrap();
    
        let mut db = Nosqlite::open(db_path_str).unwrap();
        db.create_collection("users", json!({ "name": "string" })).unwrap();
    
        let result = db.update_document_field("users", "nonexistent-id", "name", json!("fail"));
        assert!(result.is_err());
    }
    
    #[test]
    fn delete_document_should_succeed() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let db_path = tmp_dir.path().join("delete_doc_ok.nosqlite");
        let db_path_str = db_path.to_str().unwrap();
    
        let mut db = Nosqlite::open(db_path_str).unwrap();
        db.create_collection("users", json!({ "name": "string" })).unwrap();
        db.insert_document("users", json!({ "name": "ToDelete" })).unwrap();
    
        let doc_id = db.get_all_documents("users").unwrap()[0].id.clone();
        let result = db.delete_document("users", &doc_id);
        assert!(result.is_ok());
    
        let remaining = db.get_all_documents("users").unwrap();
        assert!(remaining.is_empty());
    }
    
    #[test]
    fn delete_document_should_fail() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let db_path = tmp_dir.path().join("delete_doc_fail.nosqlite");
        let db_path_str = db_path.to_str().unwrap();
    
        let mut db = Nosqlite::open(db_path_str).unwrap();
        db.create_collection("users", json!({ "name": "string" })).unwrap();
    
        // Supprimer un document inexistant
        let result = db.delete_document("users", "invalid-id");
        assert!(result.is_err());
    }
    
    #[test]
    fn get_document_by_id_should_succeed() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let db_path = tmp_dir.path().join("get_doc_ok.nosqlite");
        let db_path_str = db_path.to_str().unwrap();
    
        let mut db = Nosqlite::open(db_path_str).unwrap();
        db.create_collection("users", json!({ "name": "string" })).unwrap();
        db.insert_document("users", json!({ "name": "Alice" })).unwrap();
    
        let doc_id = db.get_all_documents("users").unwrap()[0].id.clone();
        let result = db.get_document_by_id("users", &doc_id);
    
        assert!(result.is_ok());
        assert_eq!(result.unwrap().data["name"], "Alice");
    }
    
    #[test]
    fn get_document_by_id_should_fail() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let db_path = tmp_dir.path().join("get_doc_fail.nosqlite");
        let db_path_str = db_path.to_str().unwrap();
    
        let mut db = Nosqlite::open(db_path_str).unwrap();
        db.create_collection("users", json!({ "name": "string" })).unwrap();
    
        let result = db.get_document_by_id("users", "nonexistent-id");
        assert!(result.is_err());
    }
    
    #[test]
    fn get_all_documents_should_succeed() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let db_path = tmp_dir.path().join("get_all_docs_ok.nosqlite");
        let db_path_str = db_path.to_str().unwrap();
    
        let mut db = Nosqlite::open(db_path_str).unwrap();
        db.create_collection("users", json!({ "name": "string" })).unwrap();
    
        db.insert_document("users", json!({ "name": "Alice" })).unwrap();
        db.insert_document("users", json!({ "name": "Bob" })).unwrap();
    
        let docs = db.get_all_documents("users").unwrap();
        assert_eq!(docs.len(), 2);
    }
    
    #[test]
    fn get_all_documents_should_fail() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let db_path = tmp_dir.path().join("get_all_fail.nosqlite");
        let db_path_str = db_path.to_str().unwrap();
    
        let mut db = Nosqlite::open(db_path_str).unwrap();
    
        // Pas de collection "missing"
        let result = db.get_all_documents("missing");
        assert!(result.is_err());
    }
    
    #[test]
    fn get_documents_by_field_should_succeed() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let db_path = tmp_dir.path().join("get_by_field_ok.nosqlite");
        let db_path_str = db_path.to_str().unwrap();
    
        let mut db = Nosqlite::open(db_path_str).unwrap();
        db.create_collection("posts", json!({ "author": "string" })).unwrap();
    
        db.insert_document("posts", json!({ "author": "alice" })).unwrap();
        db.insert_document("posts", json!({ "author": "bob" })).unwrap();
        db.insert_document("posts", json!({ "author": "alice" })).unwrap();
    
        let results = db.get_documents_by_field("posts", "author", "alice").unwrap();
        assert_eq!(results.len(), 2);
    }
    
    #[test]
    fn get_documents_by_field_should_fail() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let db_path = tmp_dir.path().join("get_by_field_fail.nosqlite");
        let db_path_str = db_path.to_str().unwrap();
    
        let mut db = Nosqlite::open(db_path_str).unwrap();
    
        // Aucune collection "not_there"
        let result = db.get_documents_by_field("not_there", "field", "value");
        assert!(result.is_err());
    }
    
    #[test]
    fn list_collections_should_return_all() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let db_path = tmp_dir.path().join("list_collections_ok.nosqlite");
        let db_path_str = db_path.to_str().unwrap();
    
        let mut db = Nosqlite::open(db_path_str).unwrap();
    
        db.create_collection("a", json!({ "field": "string" })).unwrap();
        db.create_collection("b", json!({ "field": "string" })).unwrap();
    
        let collections = db.list_collections();
        let names: Vec<_> = collections.iter().map(|c| c.name.as_str()).collect();
    
        assert_eq!(collections.len(), 2);
        assert!(names.contains(&"a"));
        assert!(names.contains(&"b"));
    }
    
    #[test]
    fn auto_save_is_called_on_successful_mutation() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let db_path = tmp_dir.path().join("autosave_test.nosqlite");
        let db_path_str = db_path.to_str().unwrap();
    
        {
            let mut db = Nosqlite::open(db_path_str).unwrap();
            db.create_collection("autosave", json!({ "name": "string" })).unwrap();
            db.insert_document("autosave", json!({ "name": "saved" })).unwrap();
        }
    
        // Réouverture = preuve que ça a été sauvegardé
        let mut reopened = Nosqlite::open(db_path_str).unwrap();
        let docs = reopened.get_all_documents("autosave").unwrap();
        assert_eq!(docs.len(), 1);
        assert_eq!(docs[0].data["name"], "saved");
    }
    
}
