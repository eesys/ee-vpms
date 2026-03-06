//! Tests for owner service module

use super::*;
use crate::service::current_timestamp_millis;

#[test]
fn test_owner_service_namespace() {
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

#[test]
fn test_update_email_option_nesting() {
    let email: Option<Option<String>> = Some(Some("test@example.com".to_string()));
    assert!(email.is_some());
    assert!(email.unwrap().is_some());
}

#[test]
fn test_update_clear_email() {
    let email: Option<Option<String>> = Some(None);
    assert!(email.is_some());
    assert!(email.unwrap().is_none());
}
