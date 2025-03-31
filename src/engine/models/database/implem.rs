use super::model::Database;
use crate::engine::models::collection::model::Collection;
use serde_json::Value;
use std::fmt::Display;

impl Database {
    pub fn new() -> Self {
        Database {
            collections: Vec::new(),
        }
    }

    pub fn add_collection(&mut self, name: &str, structure: Value) -> Result<(), String> {
        if self.collections.iter().any(|c| c.name == name) {
            return Err("A collection with this name already exists".into());
        }

        if !structure.is_object() {
            return Err("The structure must be a JSON object".into());
        }

        let collection = Collection::new(name.to_string(), structure);
        self.collections.push(collection);
        Ok(())
    }

    pub fn remove_collection(&mut self, name: &str) -> Result<(), String> {
        let index = self
            .collections
            .iter()
            .position(|c| c.name == name)
            .ok_or_else(|| format!("Collection '{}' not found", name))?;

        self.collections.remove(index);
        Ok(())
    }

    pub fn get_collection(&self, name: &str) -> Option<&Collection> {
        self.collections.iter().find(|c| c.name == name)
    }

    pub fn get_collection_mut(&mut self, name: &str) -> Option<&mut Collection> {
        self.collections.iter_mut().find(|c| c.name == name)
    }
}

impl Display for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Database ({} collections):", self.collections.len())?;
        for collection in &self.collections {
            writeln!(f, "  - {}", collection.name)?;
        }
        Ok(())
    }
}

impl Default for Database {
    fn default() -> Self {
        Database::new()
    }
}
