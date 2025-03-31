use super::model::Collection;
use crate::engine::models::document::model::Document;
use crate::engine::models::utils::{now, validate_against_structure};
use serde_json::Value;
use std::fmt::Display;

impl Collection {
    /// Creates a new empty collection with a given name and expected document structure.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the collection.
    /// * `structure` - A JSON object representing the expected structure/schema of documents.
    ///
    /// # Returns
    ///
    /// A new instance of [`Collection`] with no documents and a creation timestamp.
    pub fn new(name: String, structure: Value) -> Self {
        Collection {
            name,
            structure,
            documents: Vec::new(),
            created_at: now(),
        }
    }

    /// Adds a new document to the collection after validating it against the collection's structure.
    ///
    /// # Arguments
    ///
    /// * `data` - A JSON object representing the document to insert.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The document is not a JSON object.
    /// - The collection structure is not a valid JSON object.
    /// - The document does not match the expected structure.
    pub fn add_document(&mut self, data: Value) -> Result<(), String> {
        if let Value::Object(ref doc_map) = data {
            if let Value::Object(expected_structure) = &self.structure {
                if !validate_against_structure(doc_map, expected_structure) {
                    return Err("Document does not match the collection's structure".into());
                }
            } else {
                return Err("Collection structure is invalid".into());
            }
        } else {
            return Err("Document must be a JSON object".into());
        }

        let document = Document::new(data);
        self.documents.push(document);
        Ok(())
    }

    /// Updates the entire content of an existing document identified by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the document to update.
    /// * `new_data` - A new JSON object to replace the current document data.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The document is not found.
    /// - The new data does not match the expected structure.
    pub fn update_document(&mut self, id: &str, new_data: Value) -> Result<(), String> {
        let position = self
            .documents
            .iter()
            .position(|d| d.id == id)
            .ok_or_else(|| format!("Document with ID '{}' not found", id))?;

        if let Value::Object(ref doc_map) = new_data {
            if let Value::Object(expected_structure) = &self.structure {
                if !validate_against_structure(doc_map, expected_structure) {
                    return Err("Updated document does not match the collection's structure".into());
                }
            }
        }

        let mut document = self.documents[position].clone();
        document.data = new_data;
        document.updated_at = now();
        self.documents[position] = document;

        Ok(())
    }

    /// Updates a single field in a document identified by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the document to update.
    /// * `field` - The field name to update.
    /// * `value` - The new value to set for the specified field.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The document is not found.
    /// - The document's data is not a valid JSON object.
    pub fn update_field_document(
        &mut self,
        id: &str,
        field: &str,
        value: Value,
    ) -> Result<(), String> {
        let position = self
            .documents
            .iter()
            .position(|d| d.id == id)
            .ok_or_else(|| format!("Document with ID '{}' not found", id))?;

        if let Value::Object(ref mut doc_map) = self.documents[position].data {
            doc_map.insert(field.to_string(), value);
        } else {
            return Err("Document data is not a JSON object".into());
        }

        self.documents[position].updated_at = now();
        Ok(())
    }

    /// Deletes a document from the collection based on its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the document to delete.
    ///
    /// # Errors
    ///
    /// Returns an error if the document is not found.
    pub fn delete_document(&mut self, id: &str) -> Result<(), String> {
        let position = self
            .documents
            .iter()
            .position(|d| d.id == id)
            .ok_or_else(|| format!("Document with ID '{}' not found", id))?;

        self.documents.remove(position);
        Ok(())
    }

    /// Retrieves a reference to a document by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the document.
    ///
    /// # Returns
    ///
    /// An [`Option`] containing a reference to the [`Document`] if found, or `None` otherwise.
    pub fn get_document(&self, id: &str) -> Option<&Document> {
        self.documents.iter().find(|d| d.id == id)
    }

    /// Returns a reference to all documents stored in the collection.
    ///
    /// # Returns
    ///
    /// A reference to a vector of [`Document`] instances.
    pub fn all_documents(&self) -> &Vec<Document> {
        &self.documents
    }

    pub fn document_count(&self) -> usize {
        self.documents.len()
    }
}

impl Display for Collection {
    /// Formats the collection for display purposes.
    ///
    /// Outputs the collection name, its expected structure, and a list of document IDs.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Collection '{}'", self.name)?;
        writeln!(f, "  Required Structure: {}", self.structure)?;
        writeln!(f, "  {} document(s):", self.documents.len())?;
        for doc in &self.documents {
            writeln!(f, "    - {}", doc.id)?;
        }
        Ok(())
    }
}

impl Default for Collection {
    /// Creates a default collection with the name "default" and an empty JSON object as its structure.
    ///
    /// # Returns
    ///
    /// A new instance of [`Collection`] with the default name and structure.
    fn default() -> Self {
        Collection::new("default".to_string(), Value::Object(serde_json::Map::new()))
    }
}
