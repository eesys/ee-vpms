use super::*;

#[test]
fn test_database_error() {
    let err = Error::Database("connection failed".to_string());
    assert_eq!(err.to_string(), "Database error: connection failed");
}

#[test]
fn test_validation_error() {
    let err = Error::Validation("invalid input".to_string());
    assert_eq!(err.to_string(), "Validation error: invalid input");
}

#[test]
fn test_not_found_error() {
    let err = Error::NotFound("Owner not found".to_string());
    assert_eq!(err.to_string(), "Not found: Owner not found");
}

#[test]
fn test_conflict_error() {
    let err = Error::Conflict("duplicate key".to_string());
    assert_eq!(err.to_string(), "Conflict: duplicate key");
}

#[test]
fn test_internal_error() {
    let err = Error::Internal("unexpected state".to_string());
    assert_eq!(err.to_string(), "Internal server error: unexpected state");
}
