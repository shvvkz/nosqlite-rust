//! # Error Handling
//!
//! This module defines all error types and the central error handler.

pub mod error_type;
pub mod handler;

pub use error_type::NosqliteError;
pub use handler::NosqliteErrorHandler;
