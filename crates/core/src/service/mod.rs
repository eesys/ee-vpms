//! Service layer for business logic operations
//!
//! This module contains all service implementations that provide the business logic layer
//! between API handlers and database operations.

use std::time::{SystemTime, UNIX_EPOCH};

pub mod owner;

/// Get current timestamp in milliseconds since Unix epoch
///
/// This utility function is used across all service modules to generate consistent timestamps.
/// Returns the number of milliseconds elapsed since the Unix epoch (January 1, 1970 UTC).
pub fn current_timestamp_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}
