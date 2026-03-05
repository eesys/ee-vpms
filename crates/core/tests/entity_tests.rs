//! Database entity tests

#[cfg(test)]
mod entity_tests {
    use chrono::Utc;
    use ee_vpms_core::entity::owner;
    use uuid::Uuid;

    #[test]
    fn test_owner_entity_full_creation() {
        let now = Utc::now();
        let id = Uuid::new_v4().to_string();
        let owner = owner::Model {
            id: id.clone(),
            name: "Test Owner".to_string(),
            email: Some("test@example.com".to_string()),
            phone: Some("+1-555-0100".to_string()),
            address: Some("123 Test St".to_string()),
            city: Some("Test City".to_string()),
            state: Some("TC".to_string()),
            postal_code: Some("12345".to_string()),
            country: Some("Test Country".to_string()),
            created_at: now,
            updated_at: now,
        };

        assert_eq!(owner.id, id);
        assert_eq!(owner.name, "Test Owner");
        assert_eq!(owner.email, Some("test@example.com".to_string()));
        assert_eq!(owner.phone, Some("+1-555-0100".to_string()));
        assert_eq!(owner.address, Some("123 Test St".to_string()));
        assert_eq!(owner.city, Some("Test City".to_string()));
        assert_eq!(owner.state, Some("TC".to_string()));
        assert_eq!(owner.postal_code, Some("12345".to_string()));
        assert_eq!(owner.country, Some("Test Country".to_string()));
    }

    #[test]
    fn test_owner_entity_minimal() {
        let now = Utc::now();
        let owner = owner::Model {
            id: "minimal-id".to_string(),
            name: "Minimal Owner".to_string(),
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

        assert_eq!(owner.name, "Minimal Owner");
        assert!(owner.email.is_none());
        assert!(owner.phone.is_none());
    }

    #[test]
    fn test_owner_entity_clone() {
        let now = Utc::now();
        let owner1 = owner::Model {
            id: "id1".to_string(),
            name: "Owner".to_string(),
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
    fn test_owner_entity_serde() {
        let now = Utc::now();
        let owner = owner::Model {
            id: "test-id".to_string(),
            name: "Serde Test".to_string(),
            email: Some("serde@test.com".to_string()),
            phone: None,
            address: None,
            city: None,
            state: None,
            postal_code: None,
            country: None,
            created_at: now,
            updated_at: now,
        };

        let json = serde_json::to_value(&owner).unwrap();
        assert_eq!(json["name"], "Serde Test");
        assert_eq!(json["email"], "serde@test.com");

        let owner2: owner::Model = serde_json::from_value(json).unwrap();
        assert_eq!(owner.id, owner2.id);
        assert_eq!(owner.name, owner2.name);
    }

    #[test]
    fn test_owner_entity_partial_info() {
        let now = Utc::now();
        let owner = owner::Model {
            id: "partial-id".to_string(),
            name: "Partial Owner".to_string(),
            email: Some("partial@example.com".to_string()),
            phone: Some("+1-555-1234".to_string()),
            address: None,
            city: None,
            state: None,
            postal_code: None,
            country: None,
            created_at: now,
            updated_at: now,
        };

        assert!(owner.email.is_some());
        assert!(owner.phone.is_some());
        assert!(owner.address.is_none());
        assert!(owner.city.is_none());
    }

    #[test]
    fn test_owner_entity_timestamp() {
        let now = Utc::now();
        let later = Utc::now();

        let owner = owner::Model {
            id: "time-id".to_string(),
            name: "Time Owner".to_string(),
            email: None,
            phone: None,
            address: None,
            city: None,
            state: None,
            postal_code: None,
            country: None,
            created_at: now,
            updated_at: later,
        };

        assert_eq!(owner.created_at, now);
        assert_eq!(owner.updated_at, later);
        assert!(owner.updated_at >= owner.created_at);
    }

    #[test]
    fn test_owner_entity_id_uniqueness() {
        let now = Utc::now();
        let id1 = Uuid::new_v4().to_string();
        let id2 = Uuid::new_v4().to_string();

        let owner1 = owner::Model {
            id: id1.clone(),
            name: "Owner1".to_string(),
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

        let owner2 = owner::Model {
            id: id2.clone(),
            name: "Owner2".to_string(),
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

        assert_ne!(owner1.id, owner2.id);
    }
}
