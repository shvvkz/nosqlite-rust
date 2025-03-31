use super::model::Collection;
use crate::engine::models::document::model::Document;
use crate::engine::models::utils::{now, validate_against_structure};
use serde_json::Value;
use std::fmt::Display;

impl Collection {
    pub fn new(name: String, structure: Value) -> Self {
        Collection {
            name,
            structure,
            documents: Vec::new(),
            created_at: now(),
        }
    }
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

    pub fn delete_document(&mut self, id: &str) -> Result<(), String> {
        let position = self
            .documents
            .iter()
            .position(|d| d.id == id)
            .ok_or_else(|| format!("Document with ID '{}' not found", id))?;

        self.documents.remove(position);
        Ok(())
    }

    pub fn get_document(&self, id: &str) -> Option<&Document> {
        self.documents.iter().find(|d| d.id == id)
    }

    pub fn all_documents(&self) -> &Vec<Document> {
        &self.documents
    }

    pub fn document_count(&self) -> usize {
        self.documents.len()
    }
}

impl Display for Collection {
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
