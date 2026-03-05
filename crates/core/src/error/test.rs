//! Tests for error module

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
