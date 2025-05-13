#[cfg(test)]
mod tests {
    use nosqlite_rust::engine::Nosqlite;
    use serde_json::json;

    fn create_random_file_path() -> String {
        if !std::path::Path::new("./temp").exists() {
            std::fs::create_dir_all("./temp").unwrap();
        }
        let random_string = rand::random::<u64>().to_string();
        let tmp_file_path = format!("./temp/test_db_{}.nosqlite", random_string);
        tmp_file_path
    }

    #[test]
    fn open_should_succeed() {
        let db_path = create_random_file_path();
        let db_path_str = db_path.as_str();

        // Should create a new Nosqlite instance (and file) without error
        let result = Nosqlite::open(db_path_str);
        assert!(result.is_ok());
    }

    #[test]
    fn open_should_fail() {
        let db_path = create_random_file_path();
        let db_path_str = db_path.as_str();

        // On écrit un contenu totalement invalide (ni base64 ni JSON)
        std::fs::write(&db_path, "invalid-encrypted-content").unwrap();

        let result = Nosqlite::open(db_path_str);
        assert!(result.is_err());
    }

    #[test]
    fn create_collection_should_succeed() {
        let db_path = create_random_file_path();
        let db_path_str = db_path.as_str();

        let mut nosqlite = Nosqlite::open(db_path_str).unwrap();
        let schema = json!({ "username": "string", "age": "number" });

        let result = nosqlite.create_collection("users", schema);
        assert!(result.is_ok());

        let collections = nosqlite.list_collections();
        assert_eq!(collections.len(), 1);
        assert_eq!(collections[0].name, "users");
    }

    #[test]
    fn delete_collection_should_succeed() {
        let db_path = create_random_file_path();
        let db_path_str = db_path.as_str();

        let mut db = Nosqlite::open(db_path_str).unwrap();
        db.create_collection("temp", json!({ "key": "string" }))
            .unwrap();

        let result = db.delete_collection("temp");
        assert!(result.is_ok());

        let list = db.list_collections();
        assert!(list.iter().all(|c| c.name != "temp"));
    }

    #[test]
    fn delete_collection_should_fail() {
        let db_path = create_random_file_path();
        let db_path_str = db_path.as_str();

        let mut db = Nosqlite::open(db_path_str).unwrap();

        let result = db.delete_collection("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn insert_document_should_succeed() {
        let db_path = create_random_file_path();
        let db_path_str = db_path.as_str();

        let mut db = Nosqlite::open(db_path_str).unwrap();
        db.create_collection("users", json!({ "name": "string" }))
            .unwrap();

        let result = db.insert_document("users", json!({ "name": "Alice" }));
        assert!(result.is_ok());

        let docs = db.get_all_documents("users").unwrap();
        assert_eq!(docs.len(), 1);
        assert_eq!(docs[0].data["name"], "Alice");
    }

    #[test]
    fn insert_document_should_fail() {
        let db_path = create_random_file_path();
        let db_path_str = db_path.as_str();

        let mut db = Nosqlite::open(db_path_str).unwrap();

        // Pas de collection créée
        let result = db.insert_document("ghosts", json!({ "name": "Nobody" }));
        assert!(result.is_err());
    }

    #[test]
    fn update_document_should_succeed() {
        let db_path = create_random_file_path();
        let db_path_str = db_path.as_str();

        let mut db = Nosqlite::open(db_path_str).unwrap();
        db.create_collection("users", json!({ "name": "string" }))
            .unwrap();
        db.insert_document("users", json!({ "name": "Original" }))
            .unwrap();
        let result = db.update_documents(
            "users",
            "name",
            &json!("Original"),
            json!({ "name": "Updated" }),
        );

        assert!(result.is_ok());
        let doc = db.get_document("users", "name", &json!("Updated")).unwrap();
        assert_eq!(doc.data["name"], "Updated");
    }

    #[test]
    fn update_document_should_fail() {
        let db_path = create_random_file_path();
        let db_path_str = db_path.as_str();

        let mut db = Nosqlite::open(db_path_str).unwrap();
        db.create_collection("users", json!({ "name": "string" }))
            .unwrap();

        // ID invalide
        let result = db.update_documents(
            "users",
            "name",
            &json!("invalid-id"),
            json!({ "name": "Updated" }),
        );
        assert!(result.is_err());
    }

    #[test]
    fn update_document_field_should_succeed() {
        let db_path = create_random_file_path();
        let db_path_str = db_path.as_str();

        let mut db = Nosqlite::open(db_path_str).unwrap();
        db.create_collection("users", json!({ "name": "string" }))
            .unwrap();
        db.insert_document("users", json!({ "name": "Old" }))
            .unwrap();
        let result =
            db.update_documents_field("users", "name", &json!("Old"), "name", json!("New"));

        assert!(result.is_ok());
        let updated = db.get_document("users", "name", &json!("New")).unwrap();
        assert_eq!(updated.data["name"], "New");
    }

    #[test]
    fn update_document_field_should_fail() {
        let db_path = create_random_file_path();
        let db_path_str = db_path.as_str();

        let mut db = Nosqlite::open(db_path_str).unwrap();
        db.create_collection("users", json!({ "name": "string" }))
            .unwrap();

        let result = db.update_documents_field(
            "users",
            "nonexistent-id",
            &json!("nonexistent"),
            "name",
            json!("fail"),
        );
        assert!(result.is_err());
    }

    #[test]
    fn delete_documents_should_succeed() {
        let db_path = create_random_file_path();
        let db_path_str = db_path.as_str();

        let mut db = Nosqlite::open(db_path_str).unwrap();
        db.create_collection("users", json!({ "name": "string" }))
            .unwrap();
        db.insert_document("users", json!({ "name": "ToDelete" }))
            .unwrap();
        let result = db.delete_documents("users", "name", &json!("ToDelete"));
        assert!(result.is_ok());

        let remaining = db.get_all_documents("users").unwrap();
        assert!(remaining.is_empty());
    }

    #[test]
    fn delete_documents_should_fail() {
        let db_path = create_random_file_path();
        let db_path_str = db_path.as_str();

        let mut db = Nosqlite::open(db_path_str).unwrap();
        db.create_collection("users", json!({ "name": "string" }))
            .unwrap();

        // Supprimer un document inexistant
        let result = db.delete_documents("users", "invalid-id", &json!("nonexistent"));
        assert!(result.is_err());
    }

    #[test]
    fn get_document_should_succeed() {
        let db_path = create_random_file_path();
        let db_path_str = db_path.as_str();

        let mut db = Nosqlite::open(db_path_str).unwrap();
        db.create_collection("users", json!({ "name": "string" }))
            .unwrap();
        db.insert_document("users", json!({ "name": "Alice" }))
            .unwrap();
        let result = db.get_document("users", "name", &json!("Alice"));

        assert!(result.is_ok());
        assert_eq!(result.unwrap().data["name"], "Alice");
    }

    #[test]
    fn get_document_should_fail() {
        let db_path = create_random_file_path();
        let db_path_str = db_path.as_str();

        let mut db = Nosqlite::open(db_path_str).unwrap();
        db.create_collection("users", json!({ "name": "string" }))
            .unwrap();

        let result = db.get_document("users", "nonexistent-id", &json!("nonexistent"));
        assert!(result.is_err());
    }

    #[test]
    fn get_all_documents_should_succeed() {
        let db_path = create_random_file_path();
        let db_path_str = db_path.as_str();

        let mut db = Nosqlite::open(db_path_str).unwrap();
        db.create_collection("users", json!({ "name": "string" }))
            .unwrap();

        db.insert_document("users", json!({ "name": "Alice" }))
            .unwrap();
        db.insert_document("users", json!({ "name": "Bob" }))
            .unwrap();

        let docs = db.get_all_documents("users").unwrap();
        assert_eq!(docs.len(), 2);
    }

    #[test]
    fn get_all_documents_should_fail() {
        let db_path = create_random_file_path();
        let db_path_str = db_path.as_str();

        let mut db = Nosqlite::open(db_path_str).unwrap();

        // Pas de collection "missing"
        let result = db.get_all_documents("missing");
        assert!(result.is_err());
    }

    #[test]
    fn get_documents_by_field_should_succeed() {
        let db_path = create_random_file_path();
        let db_path_str = db_path.as_str();

        let mut db = Nosqlite::open(db_path_str).unwrap();
        db.create_collection("posts", json!({ "author": "string" }))
            .unwrap();

        db.insert_document("posts", json!({ "author": "alice" }))
            .unwrap();
        db.insert_document("posts", json!({ "author": "bob" }))
            .unwrap();
        db.insert_document("posts", json!({ "author": "alice" }))
            .unwrap();

        let results = db
            .get_documents("posts", &json!({ "author": "alice" }), &json!({}))
            .unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn get_documents_by_field_should_fail() {
        let db_path = create_random_file_path();
        let db_path_str = db_path.as_str();

        let mut db = Nosqlite::open(db_path_str).unwrap();

        // Aucune collection "not_there"
        let result = db.get_documents("not_there", &json!({ "field": "value" }), &json!({}));
        assert!(result.is_err());
    }

    #[test]
    fn list_collections_should_return_all() {
        let db_path = create_random_file_path();
        let db_path_str = db_path.as_str();

        let mut db = Nosqlite::open(db_path_str).unwrap();

        db.create_collection("a", json!({ "field": "string" }))
            .unwrap();
        db.create_collection("b", json!({ "field": "string" }))
            .unwrap();

        let collections = db.list_collections();
        let names: Vec<_> = collections.iter().map(|c| c.name.as_str()).collect();

        assert_eq!(collections.len(), 2);
        assert!(names.contains(&"a"));
        assert!(names.contains(&"b"));
    }

    #[test]
    fn auto_save_is_called_on_successful_mutation() {
        let db_path = create_random_file_path();
        let db_path_str = db_path.as_str();

        {
            let mut db = Nosqlite::open(db_path_str).unwrap();
            db.create_collection("autosave", json!({ "name": "string" }))
                .unwrap();
            db.insert_document("autosave", json!({ "name": "saved" }))
                .unwrap();
        }

        // Réouverture = preuve que ça a été sauvegardé
        let mut reopened = Nosqlite::open(db_path_str).unwrap();
        let docs = reopened.get_all_documents("autosave").unwrap();
        assert_eq!(docs.len(), 1);
        assert_eq!(docs[0].data["name"], "saved");
    }
}
