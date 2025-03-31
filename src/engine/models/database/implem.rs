use super::model::Database;
use crate::engine::models::collection::model::Collection;
use serde_json::Value;
use std::fmt::Display;

impl Database {
    /// Creates a new empty database instance.
    ///
    /// # Returns
    ///
    /// A [`Database`] with no collections.
    pub fn new() -> Self {
        Database {
            collections: Vec::new(),
        }
    }

    /// Adds a new collection to the database.
    ///
    /// # Arguments
    ///
    /// * `name` - The unique name of the collection to add.
    /// * `structure` - A JSON object defining the expected schema for documents in the collection.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - A collection with the same name already exists.
    /// - The provided structure is not a valid JSON object.
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

    /// Removes a collection from the database by its name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the collection to remove.
    ///
    /// # Errors
    ///
    /// Returns an error if the collection does not exist.
    pub fn remove_collection(&mut self, name: &str) -> Result<(), String> {
        let index = self
            .collections
            .iter()
            .position(|c| c.name == name)
            .ok_or_else(|| format!("Collection '{}' not found", name))?;

        self.collections.remove(index);
        Ok(())
    }

    /// Retrieves a reference to a collection by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the collection to retrieve.
    ///
    /// # Returns
    ///
    /// An [`Option`] containing a reference to the [`Collection`] if found, or `None`.
    pub fn get_collection(&self, name: &str) -> Option<&Collection> {
        self.collections.iter().find(|c| c.name == name)
    }

    /// Retrieves a mutable reference to a collection by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the collection to retrieve.
    ///
    /// # Returns
    ///
    /// An [`Option`] containing a mutable reference to the [`Collection`] if found, or `None`.
    pub fn get_collection_mut(&mut self, name: &str) -> Option<&mut Collection> {
        self.collections.iter_mut().find(|c| c.name == name)
    }
}

impl Display for Database {
    /// Formats the database for display purposes.
    ///
    /// Outputs the total number of collections and their names.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Database ({} collections):", self.collections.len())?;
        for collection in &self.collections {
            writeln!(f, "  - {}", collection.name)?;
        }
        Ok(())
    }
}

impl Default for Database {
    /// Creates a new empty database instance.
    ///
    /// # Returns
    ///
    /// A [`Database`] with no collections.
    fn default() -> Self {
        Database::new()
    }
}
