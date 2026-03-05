//! Integration tests for API handlers

#[cfg(test)]
mod api_handler_tests {
    use ee_vpms_api::handlers::{CreateOwnerRequest, OwnerResponse};
    use serde_json::json;

    #[test]
    fn test_create_owner_request_new() {
        let req = CreateOwnerRequest {
            name: "Alice".to_string(),
            email: Some("alice@example.com".to_string()),
        };
        assert_eq!(req.name, "Alice");
        assert_eq!(req.email, Some("alice@example.com".to_string()));
    }

    #[test]
    fn test_create_owner_request_no_email() {
        let req = CreateOwnerRequest {
            name: "Bob".to_string(),
            email: None,
        };
        assert_eq!(req.name, "Bob");
        assert_eq!(req.email, None);
    }

    #[test]
    fn test_owner_response_creation() {
        let resp = OwnerResponse {
            id: "123".to_string(),
            name: "Charlie".to_string(),
            email: Some("charlie@example.com".to_string()),
        };
        assert_eq!(resp.id, "123");
        assert_eq!(resp.name, "Charlie");
    }

    #[test]
    fn test_create_owner_request_serialization() {
        let req = CreateOwnerRequest {
            name: "David".to_string(),
            email: Some("david@example.com".to_string()),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["name"], "David");
        assert_eq!(json["email"], "david@example.com");
    }

    #[test]
    fn test_create_owner_request_deserialization() {
        let json = json!({
            "name": "Eve",
            "email": "eve@example.com"
        });
        let req: CreateOwnerRequest = serde_json::from_value(json).unwrap();
        assert_eq!(req.name, "Eve");
        assert_eq!(req.email, Some("eve@example.com".to_string()));
    }

    #[test]
    fn test_owner_response_serialization() {
        let resp = OwnerResponse {
            id: "456".to_string(),
            name: "Frank".to_string(),
            email: None,
        };
        let json = serde_json::to_value(&resp).unwrap();
        assert_eq!(json["id"], "456");
        assert_eq!(json["name"], "Frank");
        assert_eq!(json["email"], json!(null));
    }

    #[test]
    fn test_owner_response_deserialization() {
        let json = json!({
            "id": "789",
            "name": "Grace",
            "email": null
        });
        let resp: OwnerResponse = serde_json::from_value(json).unwrap();
        assert_eq!(resp.id, "789");
        assert_eq!(resp.name, "Grace");
        assert_eq!(resp.email, None);
    }

    #[test]
    fn test_request_with_empty_name() {
        let req = CreateOwnerRequest {
            name: "".to_string(),
            email: None,
        };
        assert_eq!(req.name.len(), 0);
    }

    #[test]
    fn test_multiple_owners() {
        let owners = vec![
            CreateOwnerRequest {
                name: "Owner1".to_string(),
                email: None,
            },
            CreateOwnerRequest {
                name: "Owner2".to_string(),
                email: Some("owner2@example.com".to_string()),
            },
        ];
        assert_eq!(owners.len(), 2);
        assert_eq!(owners[0].name, "Owner1");
        assert_eq!(owners[1].name, "Owner2");
    }
}
