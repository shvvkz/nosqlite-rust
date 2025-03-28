use nosqlite_rust::engine::models::Collection;
use serde_json::json;

fn sample_doc() -> serde_json::Value {
    json!({ "field": "val" })
}

fn make_collection() -> Collection {
    Collection::new("test".into(), json!({ "field": "string" }))
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
    col.add_document(sample_doc()).unwrap();

    let id = col.documents[0].id.clone();
    let result = col.get_document(&id);

    assert!(result.is_some());
    assert_eq!(result.unwrap().data["field"], "val");
}

#[test]
fn get_document_nested_return_correct_doc() {
    let mut col = make_collection_nested();
    col.add_document(json!({"field": "val", "nested": {"field": "nested_val"}}))
        .unwrap();

    let id = col.documents[0].id.clone();
    let result = col.get_document(&id);

    assert!(result.is_some());
    assert_eq!(result.unwrap().data["nested"]["field"], "nested_val");
}

#[test]
fn get_document_should_return_none_if_not_found() {
    let col = make_collection();
    let result = col.get_document("does-not-exist");
    assert!(result.is_none());
}

#[test]
fn all_documents_should_return_all_docs() {
    let mut col = make_collection();
    col.add_document(sample_doc()).unwrap();
    col.add_document(json!({ "field": "second" })).unwrap();

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
    col.add_document(sample_doc()).unwrap();
    col.add_document(json!({ "field": "yo" })).unwrap();

    assert_eq!(col.document_count(), 2);
}

#[test]
fn document_count_should_return_zero_if_none() {
    let col = make_collection();
    assert_eq!(col.document_count(), 0);
}

#[test]
fn add_valid_document_should_succeed() {
    let mut col = make_collection();
    let result = col.add_document(json!({ "field": "hello" }));
    assert!(result.is_ok());
}

#[test]
fn add_document_with_missing_field_should_fail() {
    let mut col = make_collection();
    let result = col.add_document(json!({ "not_field": "value" }));
    assert!(result.is_err());
}

#[test]
fn add_document_with_wrong_type_should_fail() {
    let mut col = make_collection();
    let result = col.add_document(json!({ "field": 123 })); // int au lieu de string
    assert!(result.is_err());
}

#[test]
fn add_document_with_extra_fields_should_succeed() {
    let mut col = make_collection();
    let result = col.add_document(json!({ "field": "val", "extra": 123 }));
    assert!(result.is_ok()); // Extra field toléré
}

#[test]
fn add_valid_document_nested_should_succeed() {
    let mut col = make_collection_nested();
    let result = col.add_document(json!({"field": "hello", "nested": {"field": "nested_val"}}));
    assert!(result.is_ok());
}

#[test]
fn add_document_with_missing_nested_field_should_fail() {
    let mut col = make_collection_nested();
    let result = col.add_document(json!({"field": "value", "nested": {"not_field": "nested_val"}}));
    assert!(result.is_err());
}

#[test]
fn add_document_with_wrong_nested_type_should_fail() {
    let mut col = make_collection_nested();
    let result = col.add_document(json!({"field": 123, "nested": {"field": 123}}));
    assert!(result.is_err());
}

#[test]
fn add_document_with_extra_nested_fields_should_succeed() {
    let mut col = make_collection_nested();
    let result =
        col.add_document(json!({"field": "val", "nested": {"field": "nested_val", "extra": 123}}));
    assert!(result.is_ok());
}

#[test]
fn update_existing_document_should_work() {
    let mut col = make_collection();
    col.add_document(json!({ "field": "before" })).unwrap();
    let id = &col.documents[0].id.clone();

    let res = col.update_document(id, json!({ "field": "after" }));
    assert!(res.is_ok());
    assert_eq!(col.documents[0].data["field"], "after");
}

#[test]
fn update_document_with_invalid_structure_should_fail() {
    let mut col = make_collection();
    col.add_document(json!({ "field": "original" })).unwrap();
    let id = &col.documents[0].id.clone();

    let res = col.update_document(id, json!({ "wrong_field": "nope" }));
    assert!(res.is_err());
}

#[test]
fn update_document_field_should_work() {
    let mut col = make_collection();
    col.add_document(json!({ "field": "init" })).unwrap();
    let id = &col.documents[0].id.clone();

    let res = col.update_field_document(id, "field", json!("changed"));
    assert!(res.is_ok());
    assert_eq!(col.documents[0].data["field"], "changed");
}

#[test]
fn update_field_on_nonexistent_document_should_fail() {
    let mut col = make_collection();
    let res = col.update_field_document("wrong-id", "field", json!("new"));
    assert!(res.is_err());
}

#[test]
fn delete_existing_document_should_succeed() {
    let mut col = make_collection();
    col.add_document(json!({ "field": "ok" })).unwrap();
    let id = &col.documents[0].id.clone();

    let res = col.delete_document(id);
    assert!(res.is_ok());
    assert!(col.documents.is_empty());
}

#[test]
fn delete_nonexistent_document_should_fail() {
    let mut col = make_collection();
    let res = col.delete_document("not-found-id");
    assert!(res.is_err());
}
