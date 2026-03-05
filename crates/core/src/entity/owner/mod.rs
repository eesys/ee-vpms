//! Owner entity definition
//!
//! This module defines the Owner entity which represents a pet owner in the system.
//! It maps to the `owners` table in the PostgreSQL database.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Owner model representing a pet owner in the system
///
/// This struct represents a single owner record in the database. The `id` field
/// is a UUID v7 string that uniquely identifies the owner. All timestamps are
/// stored as millisecond-precision Unix timestamps (i64) in UTC.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "owners")]
pub struct Model {
    /// Unique identifier - UUID v7 string
    #[sea_orm(primary_key)]
    pub id: String,
    
    /// Owner's full name - required field
    pub name: String,
    
    /// Owner's email address - optional
    pub email: Option<String>,
    
    /// Owner's phone number - optional
    pub phone: Option<String>,
    
    /// Street address - optional
    pub address: Option<String>,
    
    /// City - optional
    pub city: Option<String>,
    
    /// State or province - optional
    pub state: Option<String>,
    
    /// Postal code or ZIP code - optional
    pub postal_code: Option<String>,
    
    /// Country - optional
    pub country: Option<String>,
    
    /// Timestamp when the owner was created (milliseconds since epoch) - automatically set
    pub created_at: i64,
    
    /// Timestamp when the owner was last updated (milliseconds since epoch) - automatically set
    pub updated_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[cfg(test)]
mod test;
