use crate::engine::{error::NosqliteErrorHandler, models::collection::model::Collection};
use serde::{Deserialize, Serialize};

/// 🦀
/// Represents a NoSQL-style database containing multiple collections.
///
/// A `Database` is a container for multiple [`Collection`] instances,
/// each representing a logical group of documents. This structure does not
/// enforce any schema-level validation at the database level but delegates
/// it to each individual collection.
///
/// # Fields
///
/// - `collections`: A vector of [`Collection`] items that belong to this database.
///
/// # Example
///
/// ```rust
/// use nosqlite_rust::engine::models::Database;
///
/// let db = Database {
///     collections: vec![],
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Database {
    pub collections: Vec<Collection>,
}
