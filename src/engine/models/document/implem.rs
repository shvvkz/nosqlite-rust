use super::model::Document;
use crate::engine::models::utils::now;
use serde_json::Value;
use uuid::Uuid;
use std::fmt::Display;

impl Document {
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Document: '{}'", self.id)?;
        writeln!(f, "  Data: {}", self.data)?;
        writeln!(f, "  Created at: {}", self.created_at)?;
        writeln!(f, "  Updated at: {}", self.updated_at)?;
        Ok(())
    }
}