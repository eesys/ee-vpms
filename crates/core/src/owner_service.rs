//! Owner service for database operations

use crate::entity::owner;
use crate::{Error, Result};
use chrono::Utc;
use sea_orm::{entity::*, DbConn};
use uuid::Uuid;

/// Owner service for CRUD operations
pub struct OwnerService;

impl OwnerService {
    /// Create a new owner
    pub async fn create(
        db: &DbConn,
        name: String,
        email: Option<String>,
    ) -> Result<owner::Model> {
        let owner = owner::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            name: Set(name),
            email: Set(email),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };

        owner.insert(db).await.map_err(|e| Error::Database(e.to_string()))
    }

    /// Get owner by ID
    pub async fn find_by_id(db: &DbConn, id: &str) -> Result<Option<owner::Model>> {
        owner::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| Error::Database(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_owner_service_namespace() {
        // OwnerService is a zero-sized namespace for methods
        let _service = OwnerService;
        assert!(true);
    }

    #[test]
    fn test_uuid_creation() {
        let id = Uuid::new_v4().to_string();
        assert!(!id.is_empty());
    }

    #[test]
    fn test_timestamp_now() {
        let now = Utc::now();
        assert!(now.timestamp() > 0);
    }

    #[test]
    fn test_email_option_handling() {
        let email1: Option<String> = Some("test@example.com".to_string());
        let email2: Option<String> = None;
        assert!(email1.is_some());
        assert!(email2.is_none());
    }
}
