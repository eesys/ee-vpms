use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use ee_vpms_owner::entity::owner;
use ee_vpms_owner::service::OwnerService;
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Serialize, Deserialize)]
pub struct CreateOwnerRequest {
    pub name: String,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateOwnerRequest {
    pub name: Option<String>,
    pub email: Option<Option<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct OwnerResponse {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
}

impl From<owner::Model> for OwnerResponse {
    fn from(owner: owner::Model) -> Self {
        Self {
            id: owner.id,
            name: owner.name,
            email: owner.email,
        }
    }
}

pub async fn list_owners(State(state): State<AppState>) -> impl IntoResponse {
    match OwnerService::list(&state.db).await {
        Ok(owners) => {
            let responses: Vec<OwnerResponse> = owners.into_iter().map(Into::into).collect();
            (
                StatusCode::OK,
                Json(serde_json::json!({"owners": responses})),
            )
                .into_response()
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn create_owner(
    State(state): State<AppState>,
    Json(payload): Json<CreateOwnerRequest>,
) -> impl IntoResponse {
    match OwnerService::create(&state.db, payload.name, payload.email).await {
        Ok(owner) => {
            let response: OwnerResponse = owner.into();
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(_) => StatusCode::BAD_REQUEST.into_response(),
    }
}

pub async fn get_owner(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    match OwnerService::find_by_id(&state.db, &id).await {
        Ok(Some(owner)) => {
            let response: OwnerResponse = owner.into();
            (StatusCode::OK, Json(response)).into_response()
        }
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn update_owner(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateOwnerRequest>,
) -> impl IntoResponse {
    match OwnerService::update(&state.db, id, payload.name, payload.email).await {
        Ok(owner) => {
            let response: OwnerResponse = owner.into();
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(ee_vpms_owner::Error::NotFound(_)) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn delete_owner(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match OwnerService::delete(&state.db, &id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[cfg(test)]
mod test;
