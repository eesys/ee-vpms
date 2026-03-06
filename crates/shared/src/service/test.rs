use super::*;

#[test]
fn test_service_registry_register() {
    let mut registry = ServiceRegistry::new();
    registry.register("owner-service", "[::1]:50051");
    assert_eq!(
        registry.discover("owner-service"),
        Some("[::1]:50051".to_string())
    );
}

#[test]
fn test_service_registry_list() {
    let mut registry = ServiceRegistry::new();
    registry.register("owner-service", "[::1]:50051");
    registry.register("pet-service", "[::1]:50052");
    let services = registry.list();
    assert_eq!(services.len(), 2);
}

#[test]
fn test_service_registry_discover_missing() {
    let registry = ServiceRegistry::new();
    assert_eq!(registry.discover("missing-service"), None);
}
