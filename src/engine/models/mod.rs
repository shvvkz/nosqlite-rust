//! # Models
//!
//! Contains the core data structures used by the engine:
//! - `Collection`
//! - `Database`
//! - `Document`
//! - `File`

pub mod collection;
pub mod database;
pub mod document;
pub mod file;
pub mod utils;

pub use collection::Collection;
pub use database::Database;
pub use document::Document;
pub use file::File;
