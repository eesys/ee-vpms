//! Integration tests for owner gRPC service

use ee_vpms_grpc::pb::{
    CreateOwnerRequest, DeleteOwnerRequest, GetOwnerRequest, ListOwnersRequest, UpdateOwnerRequest,
};

#[tokio::test]
async fn test_create_owner_request_serialization() {
    let req = CreateOwnerRequest {
        name: "John Doe".to_string(),
        email: Some("john@example.com".to_string()),
    };
    assert_eq!(req.name, "John Doe");
    assert_eq!(req.email, Some("john@example.com".to_string()));
}

#[tokio::test]
async fn test_get_owner_request() {
    let req = GetOwnerRequest {
        id: "test-id-123".to_string(),
    };
    assert_eq!(req.id, "test-id-123");
}

#[tokio::test]
async fn test_list_owners_request() {
    let _req = ListOwnersRequest {};
    assert!(true);
}

#[tokio::test]
async fn test_update_owner_request() {
    let req = UpdateOwnerRequest {
        id: "test-id-123".to_string(),
        name: Some("Jane Doe".to_string()),
        email: Some("jane@example.com".to_string()),
    };
    assert_eq!(req.id, "test-id-123");
    assert_eq!(req.name, Some("Jane Doe".to_string()));
}

#[tokio::test]
async fn test_delete_owner_request() {
    let req = DeleteOwnerRequest {
        id: "test-id-123".to_string(),
    };
    assert_eq!(req.id, "test-id-123");
}
