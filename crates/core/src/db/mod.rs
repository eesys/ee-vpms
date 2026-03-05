//! Database connection and initialization
//!
//! This module handles database connection setup using sea-orm ORM.
//! It reads the database URL from environment variables or uses a default
//! PostgreSQL connection string for local development.

use sea_orm::{Database, DbConn};
use std::env;

/// Initialize a database connection
///
/// Attempts to establish a connection to the PostgreSQL database using the `DATABASE_URL`
/// environment variable. If not set, defaults to a local PostgreSQL instance.
///
/// # Returns
/// - `Ok(DbConn)` - A successful database connection
/// - `Err(Error::Database)` - If connection fails
///
/// # Example
/// ```ignore
/// let db = init().await?;
/// ```
pub async fn init() -> crate::Result<DbConn> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| {
            "postgres://postgres:postgres@localhost:5432/ee_vpms".to_string()
        });

    let db = Database::connect(&database_url)
        .await
        .map_err(|e| crate::Error::Database(e.to_string()))?;

    Ok(db)
}

#[cfg(test)]
mod test;
