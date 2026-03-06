use super::*;

#[test]
fn test_service_config_from_env() {
    let config = ServiceConfig::from_env();
    assert!(config.services.contains_key("owner"));
}

#[test]
fn test_discover_service() {
    let config = ServiceConfig::from_env();
    let addr = config.discover("owner");
    assert!(addr.is_some());
    assert!(addr.unwrap().contains("50051"));
}

#[test]
fn test_discover_missing_service() {
    let config = ServiceConfig::from_env();
    let addr = config.discover("nonexistent");
    assert!(addr.is_none());
}
