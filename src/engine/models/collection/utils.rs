use crate::engine::models::collection::json_utils::get_nested_value;
use crate::engine::models::collection::model::Collection;
use crate::engine::models::document::model::Document;
use serde_json::Value;

pub fn find_by_id<'a>(collection: &'a Collection, id: &str) -> Option<&'a Document> {
    collection.documents.iter().find(|d| d.id == id)
}

pub fn find_by_field<'a>(
    collection: &'a Collection,
    field_path: &str,
    value: &str,
) -> Option<&'a Document> {
    collection
        .documents
        .iter()
        .find(|d| get_nested_value(&d.data, field_path).and_then(|v| v.as_str()) == Some(value))
}

pub fn find_mut_by_id<'a>(collection: &'a mut Collection, id: &str) -> Option<&'a mut Document> {
    collection.documents.iter_mut().find(|d| d.id == id)
}

pub fn find_mut_by_field<'a>(
    collection: &'a mut Collection,
    field_path: &str,
    value: &str,
) -> Option<&'a mut Document> {
    collection
        .documents
        .iter_mut()
        .find(|d| get_nested_value(&d.data, field_path).and_then(|v| v.as_str()) == Some(value))
}
