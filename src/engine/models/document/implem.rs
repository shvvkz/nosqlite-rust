use super::model::Document;
use crate::engine::models::utils::now;
use serde_json::Value;
use std::fmt::Display;
use uuid::Uuid;

impl Document {
    /// Creates a new [`Document`] with a unique ID and timestamped metadata.
    ///
    /// This constructor wraps the provided JSON `data` into a fully-initialized document.
    /// Each document is assigned:
    /// - A unique [`UUID`] as its `id`
    /// - The current timestamp for both `created_at` and `updated_at`
    ///
    /// # Parameters
    ///
    /// - `data`: A [`serde_json::Value`] representing the content of the document. This may be
    ///   validated externally against a collection schema before insertion.
    ///
    /// # Returns
    ///
    /// A new [`Document`] instance containing:
    /// - `id`: A randomly generated `UUID` string
    /// - `data`: The input content, untouched
    /// - `created_at`: Current UTC time
    /// - `updated_at`: Same as `created_at` on initial creation
    ///
    /// # Example
    ///
    /// ```rust
    /// use serde_json::json;
    ///
    /// let raw = json!({ "id": 1, "title": "Intro to Rust" });
    /// let doc = Document::new(raw);
    ///
    /// println!("Document ID: {}", doc.id);
    /// println!("Created at: {}", doc.created_at);
    /// ```
    ///
    /// # Notes
    ///
    /// - This method does **not** validate the structure of `data` — use in combination with collection schema logic.
    /// - Timestamps are generated via the `now()` utility function.
    /// - The UUID ensures uniqueness across documents.
    ///
    /// # See Also
    ///
    /// - [`Collection::add_document`] — inserts a `Document` into a collection
    /// - [`Uuid::new_v4`] — used to generate the document ID
    /// - [`now()`] — returns the current UTC timestamp
    ///
    /// ---  
    ///
    /// 📄 Raw JSON → fully tracked, uniquely identified document object.
    ///
    /// 🔨🤖🔧 Powered by Rust
    pub fn new(data: Value) -> Document {
        let now = now();
        Document {
            id: Uuid::new_v4().to_string(),
            data,
            created_at: now,
            updated_at: now,
        }
    }
}

impl Display for Document {
    /// Formats the [`Document`] for human-readable output.
    ///
    /// This implementation displays:
    /// - The document's unique ID
    /// - Its raw JSON content
    /// - Its creation and last updated timestamps
    ///
    /// The format is ideal for terminal output, debugging, and user-facing tools
    /// that need to quickly inspect a document's identity and contents.
    ///
    /// # Example Output
    ///
    /// ```text
    /// Document: 'a84c1f17-5b6d-4c03-a93e-1e2bc0ddf4f0'
    ///   Data: {"id":1,"name":"Alice"}
    ///   Created at: 2025-04-10T22:15:32.401Z
    ///   Updated at: 2025-04-10T22:15:32.401Z
    /// ```
    ///
    /// # Usage
    ///
    /// ```rust
    /// let doc = Document::new(serde_json::json!({ "id": 1, "name": "Alice" }));
    /// println!("{}", doc);
    /// ```
    ///
    /// # See Also
    ///
    /// - [`Document::new`] — for creating a document instance
    /// - [`std::fmt::Display`] — trait being implemented
    ///
    /// ---  
    ///
    /// 🖨️ Easy inspection of document metadata and content.
    ///
    /// 🔨🤖🔧 Powered by Rust
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Document: '{}'", self.id)?;
        writeln!(f, "  Data: {}", self.data)?;
        writeln!(f, "  Created at: {}", self.created_at)?;
        writeln!(f, "  Updated at: {}", self.updated_at)?;
        Ok(())
    }
}

impl Default for Document {
    /// Creates a default [`Document`] with a unique ID and current timestamps.
    ///
    /// This constructor generates a placeholder document with:
    /// - A randomly generated UUID as its `id`
    /// - `Value::Null` as its content
    /// - The current UTC timestamp for both `created_at` and `updated_at`
    ///
    /// This is useful for initializing empty or stubbed documents, especially
    /// in testing, prototyping, or builder-style APIs.
    ///
    /// # Returns
    ///
    /// A new [`Document`] instance with default content and full metadata.
    ///
    /// # Example
    ///
    /// ```rust
    /// let doc = Document::default();
    /// assert!(doc.data.is_null());
    /// println!("{}", doc); // Uses Display to show ID, timestamps, and empty content
    /// ```
    ///
    /// # Notes
    ///
    /// - This method is equivalent to:
    ///   ```rust
    ///   Document::new(serde_json::Value::Null)
    ///   ```
    /// - The UUID ensures uniqueness even in placeholder mode.
    ///
    /// # See Also
    ///
    /// - [`Document::new`] — for creating documents with actual content
    /// - [`Default`] — the trait being implemented
    ///
    /// ---  
    ///
    /// 🧪 Perfect for mocks, tests, and lazy initializations.
    ///
    /// 🔨🤖🔧 Powered by Rust
    fn default() -> Self {
        Document {
            id: Uuid::new_v4().to_string(),
            data: Value::Null,
            created_at: now(),
            updated_at: now(),
        }
    }
}
