//! Owner entity definition

use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "owners")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn test_owner_creation() {
        let now = Utc::now();
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
        let now = Utc::now();
        let owner = Model {
            id: Uuid::new_v4().to_string(),
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
        let now = Utc::now();
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
        let now = Utc::now();
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
}
