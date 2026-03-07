use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use ee_vpms_owner::pb::owner::{
    CreateOwnerRequest as ProtoCreateOwnerRequest, DeleteOwnerRequest, GetOwnerRequest,
    ListOwnersRequest, OwnerResponse as ProtoOwnerResponse,
    UpdateOwnerRequest as ProtoUpdateOwnerRequest,
};
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

fn proto_to_response(owner: ProtoOwnerResponse) -> OwnerResponse {
    OwnerResponse {
        id: owner.id,
        name: owner.name,
        email: owner.email,
    }
}

pub async fn list_owners(State(state): State<AppState>) -> impl IntoResponse {
    let mut client = state.owner_client.as_ref().clone();
    match client.list_owners(ListOwnersRequest {}).await {
        Ok(res) => {
            let owners: Vec<OwnerResponse> = res
                .into_inner()
                .owners
                .into_iter()
                .map(proto_to_response)
                .collect();
            (StatusCode::OK, Json(serde_json::json!({"owners": owners}))).into_response()
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn create_owner(
    State(state): State<AppState>,
    Json(payload): Json<CreateOwnerRequest>,
) -> impl IntoResponse {
    let mut client = state.owner_client.as_ref().clone();
    let req = ProtoCreateOwnerRequest {
        name: payload.name,
        email: payload.email,
    };
    match client.create_owner(req).await {
        Ok(res) => {
            let response = proto_to_response(res.into_inner());
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(_) => StatusCode::BAD_REQUEST.into_response(),
    }
}

pub async fn get_owner(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let mut client = state.owner_client.as_ref().clone();
    match client.get_owner(GetOwnerRequest { id }).await {
        Ok(res) => {
            let response = proto_to_response(res.into_inner());
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) if e.code() == tonic::Code::NotFound => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn update_owner(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateOwnerRequest>,
) -> impl IntoResponse {
    let mut client = state.owner_client.as_ref().clone();
    let req = ProtoUpdateOwnerRequest {
        id,
        name: payload.name,
        email: payload.email.flatten(),
    };
    match client.update_owner(req).await {
        Ok(res) => {
            let response = proto_to_response(res.into_inner());
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) if e.code() == tonic::Code::NotFound => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn delete_owner(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let mut client = state.owner_client.as_ref().clone();
    match client.delete_owner(DeleteOwnerRequest { id }).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[cfg(test)]
mod test;
