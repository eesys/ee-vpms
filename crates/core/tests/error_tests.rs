//! Tests for error module

#[cfg(test)]
mod error_tests {
    use ee_vpms_core::Error;

    #[test]
    fn test_database_error() {
        let err = Error::Database("connection failed".to_string());
        assert_eq!(err.to_string(), "Database error: connection failed");
    }

    #[test]
    fn test_validation_error() {
        let err = Error::Validation("invalid email".to_string());
        assert_eq!(err.to_string(), "Validation error: invalid email");
    }

    #[test]
    fn test_not_found_error() {
        let err = Error::NotFound("User 123".to_string());
        assert_eq!(err.to_string(), "Not found: User 123");
    }

    #[test]
    fn test_conflict_error() {
        let err = Error::Conflict("Duplicate entry".to_string());
        assert_eq!(err.to_string(), "Conflict: Duplicate entry");
    }

    #[test]
    fn test_internal_error() {
        let err = Error::Internal("Something went wrong".to_string());
        assert_eq!(
            err.to_string(),
            "Internal server error: Something went wrong"
        );
    }

    #[test]
    fn test_error_clone() {
        let err1 = Error::NotFound("Item".to_string());
        let err2 = err1.clone();
        assert_eq!(err1, err2);
    }

    #[test]
    fn test_error_debug() {
        let err = Error::Validation("test".to_string());
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("Validation"));
    }

    #[test]
    fn test_db_error_conversion() {
        let db_err = sea_orm::DbErr::Custom("test db error".to_string());
        let err: Error = db_err.into();
        assert!(matches!(err, Error::Database(_)));
    }
}
