use nosqlite_rust::engine::{
    error::NosqliteErrorHandler, models::database::model::Database, services::document_service::*,
};
use serde_json::json;

fn create_db_and_collection() -> (Database, NosqliteErrorHandler) {
    if !std::path::Path::new("./temp").exists() {
        std::fs::create_dir_all("./temp").unwrap();
    }
    let path = format!("./temp/test_db_{}.nosqlite", rand::random::<u64>());
    let mut db = Database::new(path.as_str());
    let mut handler = NosqliteErrorHandler::new(path);
    db.add_collection("users", json!({ "name": "string" }), &mut handler)
        .unwrap();
    (db, handler)
}

#[test]
fn insert_document_should_succeed() {
    let (mut db, mut handler) = create_db_and_collection();
    let res = insert_document(&mut db, "users", json!({ "name": "Alice" }), &mut handler);
    assert!(res.is_ok());
}

#[test]
fn insert_invalid_document_should_fail() {
    let (mut db, mut handler) = create_db_and_collection();
    let res = insert_document(&mut db, "users", json!({ "invalid": "Bob" }), &mut handler);
    assert!(res.is_err());
}

#[test]
fn update_document_should_replace_correctly() {
    let (mut db, mut handler) = create_db_and_collection();
    insert_document(&mut db, "users", json!({ "name": "Carol" }), &mut handler).unwrap();
    let id = db.get_collection("users").unwrap().documents[0].id.clone();

    let res = update_document(
        &mut db,
        "users",
        &id,
        json!({ "name": "New Carol" }),
        &mut handler,
    );
    assert!(res.is_ok());
    let doc = get_document_by_id(&db, "users", &id, &mut handler).unwrap();
    assert_eq!(doc.data["name"], "New Carol");
}

#[test]
fn update_field_should_work() {
    let (mut db, mut handler) = create_db_and_collection();
    insert_document(&mut db, "users", json!({ "name": "David" }), &mut handler).unwrap();
    let id = db.get_collection("users").unwrap().documents[0].id.clone();

    let res = update_document_field(&mut db, "users", &id, "name", json!("Dave"), &mut handler);
    assert!(res.is_ok());
    let doc = get_document_by_id(&db, "users", &id, &mut handler).unwrap();
    assert_eq!(doc.data["name"], "Dave");
}

#[test]
fn delete_document_should_remove() {
    let (mut db, mut handler) = create_db_and_collection();
    insert_document(&mut db, "users", json!({ "name": "Eva" }), &mut handler).unwrap();
    let id = db.get_collection("users").unwrap().documents[0].id.clone();

    let res = delete_document(&mut db, "users", &id, &mut handler);
    assert!(res.is_ok());
    assert!(get_document_by_id(&db, "users", &id, &mut handler).is_err());
}

#[test]
fn get_document_by_id_should_return_doc() {
    let (mut db, mut handler) = create_db_and_collection();
    insert_document(&mut db, "users", json!({ "name": "Fred" }), &mut handler).unwrap();
    let id = db.get_collection("users").unwrap().documents[0].id.clone();

    let doc = get_document_by_id(&db, "users", &id, &mut handler);
    assert!(doc.is_ok());
    assert_eq!(doc.unwrap().data["name"], "Fred");
}

#[test]
fn get_all_documents_should_list_docs() {
    let (mut db, mut handler) = create_db_and_collection();
    insert_document(&mut db, "users", json!({ "name": "Gabe" }), &mut handler).unwrap();
    insert_document(&mut db, "users", json!({ "name": "Hank" }), &mut handler).unwrap();

    let docs = get_all_documents(&db, "users", &mut handler).unwrap();
    assert_eq!(docs.len(), 2);
}

#[test]
fn get_documents_by_field_should_return_filtered() {
    let (mut db, mut handler) = create_db_and_collection();
    insert_document(&mut db, "users", json!({ "name": "Ivy" }), &mut handler).unwrap();
    insert_document(&mut db, "users", json!({ "name": "John" }), &mut handler).unwrap();
    insert_document(&mut db, "users", json!({ "name": "Ivy" }), &mut handler).unwrap();

    let docs = get_documents_by_field(&db, "users", "name", "Ivy", &mut handler).unwrap();
    assert_eq!(docs.len(), 2);
}

#[test]
fn insert_into_nonexistent_collection_should_fail() {
    let mut db = Database::new("test_doc.nosqlite");
    let mut handler = NosqliteErrorHandler::new("test_doc.nosqlite".into());
    let res = insert_document(&mut db, "unknown", json!({ "name": "Zoe" }), &mut handler);
    assert!(res.is_err());
}

#[test]
fn update_nonexistent_document_should_fail() {
    let (mut db, mut handler) = create_db_and_collection();
    let res = update_document(
        &mut db,
        "users",
        "fake_id",
        json!({ "name": "Doesn't matter" }),
        &mut handler,
    );
    assert!(res.is_err());
}

#[test]
fn delete_nonexistent_document_should_fail() {
    let (mut db, mut handler) = create_db_and_collection();
    let res = delete_document(&mut db, "users", "non-existent", &mut handler);
    assert!(res.is_err());
}
