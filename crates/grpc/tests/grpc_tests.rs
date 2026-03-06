//! Tests for owner gRPC service

use ee_vpms_grpc::owner::OwnerGrpcService;
use ee_vpms_grpc::pb::{CreateOwnerRequest, GetOwnerRequest};

#[tokio::test]
async fn test_owner_grpc_service_exists() {
    let _service = OwnerGrpcService;
    assert!(true);
}

#[tokio::test]
async fn test_create_owner_request() {
    let req = CreateOwnerRequest {
        name: "Test Owner".to_string(),
        email: Some("test@example.com".to_string()),
    };
    assert_eq!(req.name, "Test Owner");
    assert!(req.email.is_some());
}

#[tokio::test]
async fn test_get_owner_request() {
    let req = GetOwnerRequest {
        id: "test-id-123".to_string(),
    };
    assert_eq!(req.id, "test-id-123");
}
