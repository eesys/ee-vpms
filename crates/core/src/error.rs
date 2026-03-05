//! Error types for ee-vpms

use thiserror::Error;

/// Application error type
#[derive(Error, Debug, Clone, PartialEq)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal server error: {0}")]
    Internal(String),
}

/// Result type alias for ee-vpms
pub type Result<T> = std::result::Result<T, Error>;

impl From<sea_orm::DbErr> for Error {
    fn from(err: sea_orm::DbErr) -> Self {
        Error::Database(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_error() {
        let err = Error::Database("test".to_string());
        assert!(err.to_string().contains("Database"));
    }

    #[test]
    fn test_validation_error() {
        let err = Error::Validation("test".to_string());
        assert!(err.to_string().contains("Validation"));
    }

    #[test]
    fn test_not_found_error() {
        let err = Error::NotFound("test".to_string());
        assert!(err.to_string().contains("Not found"));
    }

    #[test]
    fn test_conflict_error() {
        let err = Error::Conflict("test".to_string());
        assert!(err.to_string().contains("Conflict"));
    }

    #[test]
    fn test_internal_error() {
        let err = Error::Internal("test".to_string());
        assert!(err.to_string().contains("Internal"));
    }
}
