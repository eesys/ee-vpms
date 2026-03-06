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
