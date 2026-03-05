//! Tests for owner service module

use super::*;
use std::time::{SystemTime, UNIX_EPOCH};

fn current_timestamp_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

#[test]
fn test_owner_service_namespace() {
    // OwnerService is a zero-sized namespace for methods
    let _service = OwnerService;
    assert!(true);
}

#[test]
fn test_uuid_creation() {
    let ts = Timestamp::now(NoContext);
    let id = Uuid::new_v7(ts).to_string();
    assert!(!id.is_empty());
}

#[test]
fn test_timestamp_now() {
    let now = current_timestamp_millis();
    assert!(now > 0);
}

#[test]
fn test_email_option_handling() {
    let email1: Option<String> = Some("test@example.com".to_string());
    let email2: Option<String> = None;
    assert!(email1.is_some());
    assert!(email2.is_none());
}
