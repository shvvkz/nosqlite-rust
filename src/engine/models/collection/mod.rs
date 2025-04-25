//! # Collection Module
//!
//! High-level module for defining and managing document collections in NoSQLite.
//! This module re-exports the `Collection` struct and its full implementation.

pub mod model;
pub mod implem;

// Re-export the core struct
pub use model::Collection;
