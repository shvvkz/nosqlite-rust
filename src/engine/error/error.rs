use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NosqliteError {
    DatabaseNotFound(String),
    DatabaseAlreadyExists(String),
    InvalidDatabaseFormat(String),
    CollectionAlreadyExists(String),
    CollectionNotFound(String),
    InvalidCollectionStructure(String),
    DocumentInvalid(String),
    DocumentNotFound(String),
    IoError(String),
    SerializationError(String),
    EncryptionError(String),
    DeserializationError(String),
    HexDecodeError(String),
    Base64DecodeError(String),
    // Ajoute ce dont tu as besoin ici
}

impl From<std::io::Error> for NosqliteError {
    fn from(error: std::io::Error) -> Self {
        NosqliteError::IoError(error.to_string())
    }
}

impl Display for NosqliteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NosqliteError::DatabaseNotFound(name) => write!(f, "Database not found: `{}`", name),
            NosqliteError::DatabaseAlreadyExists(name) => {
                write!(f, "Database already exists: `{}`", name)
            }
            NosqliteError::InvalidDatabaseFormat(msg) => {
                write!(f, "Invalid database format: {}", msg)
            }
            NosqliteError::CollectionAlreadyExists(name) => {
                write!(f, "Collection already exists: `{}`", name)
            }
            NosqliteError::CollectionNotFound(name) => {
                write!(f, "Collection not found: `{}`", name)
            }
            NosqliteError::InvalidCollectionStructure(msg) => {
                write!(f, "Invalid collection structure: {}", msg)
            }
            NosqliteError::DocumentInvalid(msg) => write!(f, "Document invalid: {}", msg),
            NosqliteError::DocumentNotFound(id) => {
                write!(f, "Document not found: `{}`", id)
            }
            NosqliteError::IoError(msg) => write!(f, "IO error: {}", msg),
            NosqliteError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            NosqliteError::EncryptionError(msg) => write!(f, "Encryption error: {}", msg),
            NosqliteError::DeserializationError(msg) => write!(f, "Deserialization error: {}", msg),
            NosqliteError::HexDecodeError(msg) => write!(f, "Hex decode error: {}", msg),
            NosqliteError::Base64DecodeError(msg) => write!(f, "Base64 decode error: {}", msg),
        }
    }
}
