use nosqlite_rust::engine::{
    error::{NosqliteError, NosqliteErrorHandler},
    models::{collection::model::Collection, database::model::Database},
    services::collection_service::*,
};
use serde_json::json;

/// Crée un handler de test temporaire
fn make_handler() -> NosqliteErrorHandler {
    if !std::path::Path::new("./temp").exists() {
        std::fs::create_dir_all("./temp").unwrap();
    }
    NosqliteErrorHandler::new(format!("./temp/test_db_{}.nosqlite", rand::random::<u64>().to_string()).into())
}

/// Crée une base de données de test
fn make_db() -> Database {
    if !std::path::Path::new("./temp").exists() {
        std::fs::create_dir_all("./temp").unwrap();
    }
    let db_path = format!("./temp/test_db_{}.nosqlite", rand::random::<u64>().to_string());
    Database::new(db_path.as_str())
}

#[test]
fn create_and_retrieve_collection_should_succeed() {
    let mut db = make_db();
    let mut handler = make_handler();
    let res = create_collection(&mut db, "users", json!({ "name": "string" }), &mut handler);
    assert!(res.is_ok());

    let col = get_collection(&db, "users", &mut handler).unwrap();
    assert_eq!(col.name, "users");
}

#[test]
fn create_duplicate_collection_should_fail() {
    let mut db = make_db();
    let mut handler = make_handler();
    create_collection(&mut db, "users", json!({ "name": "string" }), &mut handler).unwrap();
    let res = create_collection(&mut db, "users", json!({ "name": "string" }), &mut handler);
    assert!(res.is_err());
}

#[test]
fn delete_existing_collection_should_succeed() {
    let mut db = make_db();
    let mut handler = make_handler();
    create_collection(&mut db, "to_delete", json!({ "x": "string" }), &mut handler).unwrap();

    let res = delete_collection(&mut db, "to_delete", &mut handler);
    assert!(res.is_ok());

    let lookup = get_collection(&db, "to_delete", &mut handler);
    assert!(lookup.is_err());
}

#[test]
fn delete_nonexistent_collection_should_fail() {
    let mut db = make_db();
    let mut handler = make_handler();
    let res = delete_collection(&mut db, "nope", &mut handler);
    assert!(res.is_err());
}

#[test]
fn get_collection_should_return_correct_reference() {
    let mut db = make_db();
    let mut handler = make_handler();
    create_collection(&mut db, "data", json!({ "v": "string" }), &mut handler).unwrap();

    let col = get_collection(&db, "data", &mut handler).unwrap();
    assert_eq!(col.name, "data");
}

#[test]
fn get_collection_should_fail_if_missing() {
    let db = make_db();
    let mut handler = make_handler();
    let col = get_collection(&db, "ghost", &mut handler);
    assert!(col.is_err());
}

#[test]
fn get_collection_mut_should_allow_modification() {
    let mut db = make_db();
    let mut handler = make_handler();
    create_collection(&mut db, "mutable", json!({ "z": "string" }), &mut handler).unwrap();

    let col_mut = get_collection_mut(&mut db, "mutable", &mut handler).unwrap();
    assert_eq!(col_mut.name, "mutable");
    col_mut.documents.clear(); // just modifying to test access
    assert_eq!(col_mut.documents.len(), 0);
}

#[test]
fn get_collection_mut_should_fail_if_missing() {
    let mut db = make_db();
    let mut handler = make_handler();
    let col = get_collection_mut(&mut db, "unknown", &mut handler);
    assert!(col.is_err());
}

#[test]
fn list_collections_should_return_all() {
    let mut db = make_db();
    let mut handler = make_handler();
    create_collection(&mut db, "a", json!({ "x": "string" }), &mut handler).unwrap();
    create_collection(&mut db, "b", json!({ "y": "string" }), &mut handler).unwrap();

    let list = list_collections(&db);
    let names: Vec<_> = list.iter().map(|c| c.name.as_str()).collect();
    assert!(names.contains(&"a"));
    assert!(names.contains(&"b"));
    assert_eq!(list.len(), 2);
}
