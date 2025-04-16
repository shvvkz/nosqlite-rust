use crate::engine::models::document::model::Document;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 🦀
/// Represents a collection of documents within a NoSQL database.
///
/// A `Collection` is a logical grouping of `Document` items,
/// defined by a name and a schema (`structure`). It tracks when it was created
/// and contains all the documents that belong to it.
///
/// # Fields
///
/// - `name`: The name of the collection.
/// - `documents`: A vector containing all the documents in the collection.
/// - `created_at`: The timestamp (in seconds since Unix epoch) when the collection was created.
/// - `structure`: A JSON value defining the schema or structure of the documents within the collection.
///
/// # Example
///
/// ```rust
/// use serde_json::json;
/// use nosqlite_rust::engine::models::Collection;
///
/// let collection = Collection::new("users".to_string(), json!({"name": "string", "age": "number"}));
/// assert_eq!(collection.name, "users");
/// assert_eq!(collection.structure, json!({"name": "string", "age": "number"}));
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Collection {
    pub name: String,
    pub documents: Vec<Document>,
    pub created_at: u64,
    pub structure: Value,
}
