use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a single document stored in a collection.
///
/// A `Document` is a structured JSON object that includes metadata such as a unique ID
/// and timestamps for creation and last update. This is the core unit of data stored in a [`Collection`].
///
/// # Fields
///
/// - `id`: A unique identifier for the document.
/// - `data`: The actual contents of the document, stored as a JSON value.
/// - `updated_at`: The Unix timestamp (in seconds) of the last update to the document.
/// - `created_at`: The Unix timestamp (in seconds) when the document was created.
///
/// # Example
///
/// ```rust
/// use serde_json::Value;
/// use nosqlite_rust::engine::models::Document;;
///
/// let doc = Document {
///     id: "abc123".to_string(),
///     data: serde_json::json!({"title": "Hello", "views": 42}),
///     created_at: 1700000000,
///     updated_at: 1700000000,
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Document {
    pub id: String,
    pub data: Value,
    pub updated_at: u64,
    pub created_at: u64,
}
