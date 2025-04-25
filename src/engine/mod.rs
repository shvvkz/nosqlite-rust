//! # Engine
//!
//! This module contains the core logic of the NoSQLite engine, including:
//! - Data models (`models`)
//! - Error definitions (`error`)
//! - Public API (`nosqlite`)
//! - Operation services (`services`)

pub mod error;
pub mod models;
pub mod nosqlite;
pub mod services;

pub use nosqlite::Nosqlite;
