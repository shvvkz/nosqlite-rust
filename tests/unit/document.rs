use nosqlite_rust::engine::models::Document;
use serde_json::json;

#[test]
fn create_document_should_have_fields_filled() {
    let data = json!({ "key": "value" });
    let doc = Document::new(data.clone());

    assert_eq!(doc.data, data);
    assert!(!doc.id.is_empty());
    assert!(doc.created_at != 0);
    assert_eq!(doc.created_at, doc.updated_at);
}
