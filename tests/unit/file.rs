use nosqlite_rust::engine::models::file::model::File;
use nosqlite_rust::engine::models::Database;
use serde_json::json;
use std::fs;
use tempfile::NamedTempFile;

#[test]
fn write_and_load_should_persist_database() {
    let mut db = Database::new();
    db.add_collection("save_me", json!({ "x": "string" }))
        .unwrap();
    let col_mut = db.get_collection_mut("save_me").unwrap();
    col_mut.add_document(json!({ "x": "abc" })).unwrap();

    let tmp = NamedTempFile::new().unwrap();
    let path = tmp.path().to_str().unwrap();
    File::save(path, &db);

    let reloaded = File::load_or_create(path);
    let col = reloaded.get_collection("save_me").unwrap();
    assert_eq!(col.document_count(), 1);
}
