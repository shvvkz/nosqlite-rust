use super::model::Document;
use crate::engine::models::utils::now;
use serde_json::Value;
use std::fmt::Display;
use uuid::Uuid;

impl Document {
    /// Creates a new document with a unique ID and timestamps.
    ///
    /// The document will have the current timestamp for both `created_at` and `updated_at`,
    /// and the provided data as its content.
    ///
    /// # Arguments
    ///
    /// * `data` - A JSON value representing the contents of the document.
    ///
    /// # Returns
    ///
    /// A new instance of [`Document`] with a generated UUID and current timestamps.
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
    /// Formats the document for display purposes.
    ///
    /// Outputs the document's ID, contents, and timestamps.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Document: '{}'", self.id)?;
        writeln!(f, "  Data: {}", self.data)?;
        writeln!(f, "  Created at: {}", self.created_at)?;
        writeln!(f, "  Updated at: {}", self.updated_at)?;
        Ok(())
    }
}

impl Default for Document {
    /// Creates a new document with a unique ID and current timestamps.
    ///
    /// # Returns
    ///
    /// A new instance of [`Document`] with a generated UUID and current timestamps.
    fn default() -> Self {
        Document {
            id: Uuid::new_v4().to_string(),
            data: Value::Null,
            created_at: now(),
            updated_at: now(),
        }
    }
}
