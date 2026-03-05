//! Owner API handlers
//!
//! This module provides HTTP request handlers for owner-related endpoints.
//! It includes request/response models and the handler functions.

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

/// Request payload for creating a new owner
#[derive(Serialize, Deserialize)]
pub struct CreateOwnerRequest {
    /// Owner's full name (required)
    pub name: String,
    
    /// Owner's email address (optional)
    pub email: Option<String>,
}

/// Response payload for owner-related endpoints
#[derive(Serialize, Deserialize)]
pub struct OwnerResponse {
    /// Unique owner identifier
    pub id: String,
    
    /// Owner's full name
    pub name: String,
    
    /// Owner's email address
    pub email: Option<String>,
}

/// List all owners
///
/// Handles GET /owners requests to retrieve all owners.
/// Currently returns an empty list as a placeholder.
///
/// # Returns
/// JSON response with owners array
pub async fn list_owners() -> impl IntoResponse {
    Json(serde_json::json!({"owners": []}))
}

/// Create a new owner
///
/// Handles POST /owners requests to create a new owner record.
/// The request body should contain the owner's name and optional email.
///
/// # Arguments
/// * `payload` - CreateOwnerRequest containing owner details
///
/// # Returns
/// - HTTP 201 Created with the new owner's ID
pub async fn create_owner(
    Json(_payload): Json<CreateOwnerRequest>,
) -> impl IntoResponse {
    (StatusCode::CREATED, Json(serde_json::json!({"id": ""})))
}

/// Get owner by ID
///
/// Handles GET /owners/:id requests to retrieve a specific owner.
/// Currently returns 404 as a placeholder.
///
/// # Arguments
/// * `id` - Owner's unique identifier
///
/// # Returns
/// - HTTP 200 with owner details if found
/// - HTTP 404 if owner not found
pub async fn get_owner(Path(_id): Path<String>) -> impl IntoResponse {
    StatusCode::NOT_FOUND
}

#[cfg(test)]
mod test;
