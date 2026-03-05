//! Tests for owner entity module

use super::*;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::{NoContext, Timestamp, Uuid};

fn current_timestamp_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

#[test]
fn test_owner_creation() {
    let now = current_timestamp_millis();
    let owner = Model {
        id: "test-id".to_string(),
        name: "Test Owner".to_string(),
        email: Some("test@example.com".to_string()),
        phone: None,
        address: None,
        city: None,
        state: None,
        postal_code: None,
        country: None,
        created_at: now,
        updated_at: now,
    };
    assert_eq!(owner.name, "Test Owner");
}

#[test]
fn test_owner_with_full_details() {
    let now = current_timestamp_millis();
    let ts = Timestamp::now(NoContext);
    let owner = Model {
        id: Uuid::new_v7(ts).to_string(),
        name: "Full Owner".to_string(),
        email: Some("full@example.com".to_string()),
        phone: Some("+1234567890".to_string()),
        address: Some("123 Main St".to_string()),
        city: Some("Test City".to_string()),
        state: Some("TC".to_string()),
        postal_code: Some("12345".to_string()),
        country: Some("Test Country".to_string()),
        created_at: now,
        updated_at: now,
    };
    assert_eq!(owner.phone, Some("+1234567890".to_string()));
    assert_eq!(owner.address, Some("123 Main St".to_string()));
}

#[test]
fn test_owner_clone() {
    let now = current_timestamp_millis();
    let owner1 = Model {
        id: "id1".to_string(),
        name: "Clone Test".to_string(),
        email: None,
        phone: None,
        address: None,
        city: None,
        state: None,
        postal_code: None,
        country: None,
        created_at: now,
        updated_at: now,
    };
    let owner2 = owner1.clone();
    assert_eq!(owner1, owner2);
}

#[test]
fn test_owner_optional_fields() {
    let now = current_timestamp_millis();
    let owner = Model {
        id: "opt-id".to_string(),
        name: "Optional".to_string(),
        email: None,
        phone: None,
        address: Some("Some Address".to_string()),
        city: None,
        state: None,
        postal_code: None,
        country: None,
        created_at: now,
        updated_at: now,
    };
    assert!(owner.email.is_none());
    assert!(owner.address.is_some());
}
