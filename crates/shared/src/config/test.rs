use super::*;

#[test]
fn test_direct_resolver_from_env() {
    let resolver = DirectResolver::from_env();
    assert!(resolver.discover("owner").is_some());
}

#[test]
fn test_discover_service() {
    let resolver = DirectResolver::from_env();
    let addr = resolver.discover("owner");
    assert!(addr.is_some());
    assert!(addr.unwrap().contains("50051"));
}

#[test]
fn test_discover_missing_service() {
    let resolver = DirectResolver::from_env();
    let addr = resolver.discover("nonexistent");
    assert!(addr.is_none());
}

#[test]
fn test_get_service_listen_address() {
    let addr = get_service_listen_address("owner");
    assert!(addr.is_some());
    assert!(addr.unwrap().contains("50051"));
}

#[test]
fn test_get_missing_service_listen_address() {
    let addr = get_service_listen_address("nonexistent");
    assert!(addr.is_none());
}

#[test]
fn test_default_resolver_type() {
    let default_type = ResolverType::default();
    assert_eq!(default_type, ResolverType::Direct);
}

#[test]
fn test_service_descriptor_creation() {
    let desc = ServiceDescriptor {
        addr: "http://localhost:8080".to_string(),
        resolver_type: Some(ResolverType::Direct),
    };
    assert_eq!(desc.addr, "http://localhost:8080");
    assert_eq!(desc.resolver_type, Some(ResolverType::Direct));
}

#[test]
fn test_direct_resolver_from_config() {
    let mut config = DirectResolverConfig::default();
    config.services.insert(
        "test".to_string(),
        ServiceDescriptor {
            addr: "http://test:8080".to_string(),
            resolver_type: Some(ResolverType::Direct),
        },
    );
    let resolver = DirectResolver::from_config(&config);
    let addr = resolver.discover("test");
    assert_eq!(addr, Some("http://test:8080".to_string()));
}
