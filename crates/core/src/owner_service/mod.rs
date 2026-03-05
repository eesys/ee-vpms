//! Owner service for database operations
//!
//! This module provides the business logic layer for Owner entity operations.
//! It encapsulates all database operations related to owners.

use crate::entity::owner;
use crate::{Error, Result};
use sea_orm::{entity::*, DbConn};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::{NoContext, Timestamp, Uuid};

/// Owner service providing CRUD operations
///
/// This service provides methods to create, read, and manage owner records in the database.
/// It serves as the business logic layer between the API handlers and the database.
pub struct OwnerService;

impl OwnerService {
    /// Create a new owner record
    ///
    /// Creates a new owner with the provided name and email. The ID is generated
    /// as a UUID v7 string, and timestamps are set to the current time in milliseconds.
    ///
    /// # Arguments
    /// * `db` - Database connection
    /// * `name` - Owner's full name (required)
    /// * `email` - Owner's email address (optional)
    ///
    /// # Returns
    /// - `Ok(owner::Model)` - The newly created owner
    /// - `Err(Error::Database)` - If database operation fails
    pub async fn create(
        db: &DbConn,
        name: String,
        email: Option<String>,
    ) -> Result<owner::Model> {
        let ts = Timestamp::now(NoContext);
        let now_millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;
        let owner = owner::ActiveModel {
            id: Set(Uuid::new_v7(ts).to_string()),
            name: Set(name),
            email: Set(email),
            created_at: Set(now_millis),
            updated_at: Set(now_millis),
            ..Default::default()
        };

        owner.insert(db).await.map_err(|e| Error::Database(e.to_string()))
    }

    /// Find owner by ID
    ///
    /// Retrieves an owner record by their unique ID.
    ///
    /// # Arguments
    /// * `db` - Database connection
    /// * `id` - Owner's ID (UUID string)
    ///
    /// # Returns
    /// - `Ok(Some(owner::Model))` - Owner found
    /// - `Ok(None)` - Owner not found
    /// - `Err(Error::Database)` - If database operation fails
    pub async fn find_by_id(db: &DbConn, id: &str) -> Result<Option<owner::Model>> {
        owner::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| Error::Database(e.to_string()))
    }
}

#[cfg(test)]
mod test;
