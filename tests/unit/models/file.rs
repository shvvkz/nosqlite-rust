use nosqlite_rust::engine::error::NosqliteErrorHandler;
use nosqlite_rust::engine::models::file::model::File;
use nosqlite_rust::engine::models::Database;
use serde_json::json;
use std::fs;
use tempfile::NamedTempFile;

fn make_error_handler() -> NosqliteErrorHandler {
    let tmp_log = NamedTempFile::new().unwrap();
    let path = tmp_log
        .path()
        .with_extension("nosqlite")
        .to_string_lossy()
        .to_string();
    NosqliteErrorHandler::new(path)
}

#[test]
fn write_and_load_should_persist_database() {
    let mut db = Database::new("test_db.nosqlite");
    let mut handler = make_error_handler();
    db.add_collection("save_me", json!({ "x": "string" }), &mut handler)
        .unwrap();
    let col_mut = db.get_collection_mut("save_me").unwrap();
    col_mut
        .add_document(json!({ "x": "abc" }), &mut handler)
        .unwrap();

    let tmp = NamedTempFile::new().unwrap();
    let path = tmp.path().to_str().unwrap();
    File::save(path, &db, &mut handler);

    let reloaded =
        File::load_or_create(path, &mut NosqliteErrorHandler::new(path.to_string())).unwrap();
    let col = reloaded.get_collection("save_me").unwrap();
    assert_eq!(col.document_count(), 1);
}
