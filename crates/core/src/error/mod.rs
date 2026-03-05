//! Error types and result aliases for ee-vpms
//!
//! This module defines the core error type used throughout the application and implements
//! conversions from external crate errors to our error type.

use thiserror::Error;

/// Application error type
///
/// This enum represents all possible errors that can occur in the ee-vpms application.
/// It supports serialization for API responses.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum Error {
    /// Database operation failed
    #[error("Database error: {0}")]
    Database(String),

    /// Input validation failed
    #[error("Validation error: {0}")]
    Validation(String),

    /// Requested resource was not found
    #[error("Not found: {0}")]
    NotFound(String),

    /// Resource already exists (conflict)
    #[error("Conflict: {0}")]
    Conflict(String),

    /// Unexpected internal error
    #[error("Internal server error: {0}")]
    Internal(String),
}

/// Result type alias for ee-vpms operations
///
/// Most operations in ee-vpms return this type, making error handling consistent.
pub type Result<T> = std::result::Result<T, Error>;

/// Implement conversion from sea-orm database errors
impl From<sea_orm::DbErr> for Error {
    fn from(err: sea_orm::DbErr) -> Self {
        Error::Database(err.to_string())
    }
}

#[cfg(test)]
mod test;
