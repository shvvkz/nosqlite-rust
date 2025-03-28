use crate::engine::models::document::model::Document;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Collection {
    pub name: String,
    pub documents: Vec<Document>,
    pub created_at: u64,
    pub structure: Value,
}
