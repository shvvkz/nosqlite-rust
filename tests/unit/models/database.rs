use nosqlite_rust::engine::{error::NosqliteErrorHandler, models::Database};
use serde_json::json;
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
fn get_collection_should_return_reference() {
    let mut db = Database::new("test_db.nosqlite");
    let mut handler = make_error_handler();
    db.add_collection("users", json!({ "name": "string" }), &mut handler)
        .unwrap();

    let col = db.get_collection("users");
    assert!(col.is_some());
    assert_eq!(col.unwrap().name, "users");
}
#[test]
fn get_collection_should_return_none_if_not_found() {
    let db = Database::new("test_db.nosqlite");
    let col = db.get_collection("non_existent");
    assert!(col.is_none());
}

#[test]
fn get_collection_mut_should_return_mutable_reference() {
    let mut db = Database::new("test_db.nosqlite");
    let mut handler = make_error_handler();
    db.add_collection("users", json!({ "name": "string" }), &mut handler)
        .unwrap();

    let col_mut = db.get_collection_mut("users");
    assert!(col_mut.is_some());
    let mut handler = make_error_handler();
    col_mut
        .unwrap()
        .add_document(json!({ "name": "X" }), &mut handler)
        .unwrap();
}

#[test]
fn get_collection_mut_should_return_none_if_not_found() {
    let mut db = Database::new("test_db.nosqlite");
    let col_mut = db.get_collection_mut("non_existent");
    assert!(col_mut.is_none());
}

#[test]
fn add_collection_should_work() {
    let mut db = Database::new("test_db.nosqlite");
    let mut handler = make_error_handler();
    assert!(db
        .add_collection("test", json!({ "a": "string" }), &mut handler)
        .is_ok());
    assert!(db.get_collection("test").is_some());
}

#[test]
fn add_duplicate_collection_should_fail() {
    let mut db = Database::new("test_db.nosqlite");
    let mut handler = make_error_handler();
    db.add_collection("test", json!({ "a": "string" }), &mut handler)
        .unwrap();
    let result = db.add_collection("test", json!({ "a": "string" }), &mut handler);
    assert!(result.is_err());
}

#[test]
fn add_collection_with_invalid_structure_should_fail() {
    let mut db = Database::new("test_db.nosqlite");
    let mut handler = make_error_handler();
    let result = db.add_collection("invalid", json!("not-an-object"), &mut handler);
    assert!(result.is_err());
}

#[test]
fn remove_existing_collection_should_succeed() {
    let mut db = Database::new("test_db.nosqlite");
    let mut handler = make_error_handler();
    db.add_collection("to_remove", json!({ "x": "string" }), &mut handler)
        .unwrap();
    let res = db.remove_collection("to_remove", &mut handler);
    assert!(res.is_ok());
    assert!(db.get_collection("to_remove").is_none());
}

#[test]
fn remove_nonexistent_collection_should_fail() {
    let mut db = Database::new("test_db.nosqlite");
    let mut handler = make_error_handler();
    let res = db.remove_collection("does_not_exist", &mut handler);
    assert!(res.is_err());
}
