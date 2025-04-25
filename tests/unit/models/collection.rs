use nosqlite_rust::engine::{error::NosqliteErrorHandler, models::Collection};
use serde_json::json;
use tempfile::NamedTempFile;

fn sample_doc() -> serde_json::Value {
    json!({ "field": "val" })
}

fn make_collection() -> Collection {
    Collection::new("test".into(), json!({ "field": "string" }))
}

fn make_error_handler() -> NosqliteErrorHandler {
    let tmp_log = NamedTempFile::new().unwrap();
    let path = tmp_log
        .path()
        .with_extension("nosqlite")
        .to_string_lossy()
        .to_string();
    NosqliteErrorHandler::new(path)
}

fn make_collection_nested() -> Collection {
    Collection::new(
        "test".into(),
        json!({"field": "string", "nested": {"field": "string"}}),
    )
}

#[test]
fn create_collection_should_have_fields_filled() {
    let col = make_collection();

    assert_eq!(col.name, "test");
    assert_eq!(col.structure, json!({ "field": "string" }));
    assert_eq!(col.documents.len(), 0);
}

#[test]
fn create_collection_nested_should_have_fields_filled() {
    let col = make_collection_nested();

    assert_eq!(col.name, "test");
    assert_eq!(
        col.structure,
        json!({"field": "string", "nested": {"field": "string"}})
    );
    assert_eq!(col.documents.len(), 0);
}

#[test]
fn get_document_should_return_correct_doc() {
    let mut col = make_collection();
    let mut handler = make_error_handler();
    col.add_document(sample_doc(), &mut handler).unwrap();

    let result = col.get_document("field", &json!("val"));

    assert!(result.is_some());
    assert_eq!(result.unwrap().data["field"], "val");
}

#[test]
fn get_document_nested_return_correct_doc() {
    let mut col = make_collection_nested();
    let mut handler = make_error_handler();
    col.add_document(
        json!({"field": "val", "nested": {"field": "nested_val"}}),
        &mut handler,
    )
    .unwrap();
    let result = col.get_document("field", &json!("val"));

    assert!(result.is_some());
    assert_eq!(result.unwrap().data["nested"]["field"], "nested_val");
}

#[test]
fn get_document_should_return_none_if_not_found() {
    let col = make_collection();
    let result = col.get_document("does-not-exist", &json!("does-not-exist"));
    assert!(result.is_none());
}

#[test]
fn all_documents_should_return_all_docs() {
    let mut col = make_collection();
    let mut handler = make_error_handler();
    col.add_document(sample_doc(), &mut handler).unwrap();
    col.add_document(json!({ "field": "second" }), &mut handler)
        .unwrap();

    let docs = col.all_documents();
    assert_eq!(docs.len(), 2);
}

#[test]
fn all_documents_should_return_empty_if_none() {
    let col = make_collection();
    let docs = col.all_documents();
    assert!(docs.is_empty());
}

#[test]
fn document_count_should_return_correct_number() {
    let mut col = make_collection();
    let mut handler = make_error_handler();
    col.add_document(sample_doc(), &mut handler).unwrap();
    col.add_document(json!({ "field": "yo" }), &mut handler)
        .unwrap();

    assert_eq!(col.document_count(), 2);
}

#[test]
fn document_count_should_return_zero_if_none() {
    let col = make_collection();
    assert_eq!(col.document_count(), 0);
}

#[test]
fn add_valid_document_should_succeed() {
    let mut handler = make_error_handler();
    let mut col = make_collection();
    let result = col.add_document(json!({ "field": "hello" }), &mut handler);
    assert!(result.is_ok());
}

#[test]
fn add_document_with_missing_field_should_fail() {
    let mut col = make_collection();
    let mut handler = make_error_handler();
    let result = col.add_document(json!({ "not_field": "value" }), &mut handler);
    assert!(result.is_err());
}

#[test]
fn add_document_with_wrong_type_should_fail() {
    let mut col = make_collection();
    let mut handler = make_error_handler();
    let result = col.add_document(json!({ "field": 123 }), &mut handler); // int au lieu de string
    assert!(result.is_err());
}

#[test]
fn add_document_with_extra_fields_should_succeed() {
    let mut col = make_collection();
    let mut handler = make_error_handler();
    let result = col.add_document(json!({ "field": "val", "extra": 123 }), &mut handler);
    assert!(result.is_ok()); // Extra field toléré
}

#[test]
fn add_valid_document_nested_should_succeed() {
    let mut col = make_collection_nested();
    let mut handler = make_error_handler();
    let result = col.add_document(
        json!({"field": "hello", "nested": {"field": "nested_val"}}),
        &mut handler,
    );
    assert!(result.is_ok());
}

#[test]
fn add_document_with_missing_nested_field_should_fail() {
    let mut col = make_collection_nested();
    let mut handler = make_error_handler();
    let result = col.add_document(
        json!({"field": "value", "nested": {"not_field": "nested_val"}}),
        &mut handler,
    );
    assert!(result.is_err());
}

#[test]
fn add_document_with_wrong_nested_type_should_fail() {
    let mut col = make_collection_nested();
    let mut handler = make_error_handler();
    let result = col.add_document(
        json!({"field": 123, "nested": {"field": 123}}),
        &mut handler,
    );
    assert!(result.is_err());
}

#[test]
fn add_document_with_extra_nested_fields_should_succeed() {
    let mut col = make_collection_nested();
    let mut handler = make_error_handler();
    let result = col.add_document(
        json!({"field": "val", "nested": {"field": "nested_val", "extra": 123}}),
        &mut handler,
    );
    assert!(result.is_ok());
}

#[test]
fn update_existing_document_should_work() {
    let mut col = make_collection();
    let mut handler = make_error_handler();
    col.add_document(json!({ "field": "before" }), &mut handler)
        .unwrap();
    let id = &col.documents[0].id.clone();

    let res = col.update_documents(
        "field",
        &json!("before"),
        json!({ "field": "after" }),
        &mut handler,
    );
    assert!(res.is_ok());
    assert_eq!(col.documents[0].data["field"], "after");
}

#[test]
fn update_document_with_invalid_structure_should_fail() {
    let mut col = make_collection();
    let mut handler = make_error_handler();
    col.add_document(json!({ "field": "original" }), &mut handler)
        .unwrap();
    let id = &col.documents[0].id.clone();

    let res = col.update_documents(
        "field",
        &json!("original"),
        json!({ "wrong_field": "nope" }),
        &mut handler,
    );
    assert!(res.is_err());
}

#[test]
fn update_document_field_should_work() {
    let mut col = make_collection();
    let mut handler = make_error_handler();
    col.add_document(json!({ "field": "init" }), &mut handler)
        .unwrap();
    let res = col.update_documents_field(
        "field",
        &json!("init"),
        "field",
        json!("changed"),
        &mut handler,
    );
    assert!(res.is_ok());
    assert_eq!(col.documents[0].data["field"], "changed");
}

#[test]
fn update_field_on_nonexistent_document_should_fail() {
    let mut col = make_collection();
    let mut handler = make_error_handler();
    let res = col.update_documents_field(
        "wrong-id",
        &json!("wrong"),
        "field",
        json!("new"),
        &mut handler,
    );
    assert!(res.is_err());
}

#[test]
fn delete_existing_documents_should_succeed() {
    let mut col = make_collection();
    let mut handler = make_error_handler();
    col.add_document(json!({ "field": "ok" }), &mut handler)
        .unwrap();
    let res = col.delete_documents("field", &json!("ok"), &mut handler);
    assert!(res.is_ok());
    assert!(col.documents.is_empty());
}

#[test]
fn delete_nonexistent_documents_should_fail() {
    let mut col = make_collection();
    let mut handler = make_error_handler();
    let res = col.delete_documents("not-found-id", &json!("not-found"), &mut handler);
    assert!(res.is_err());
}
