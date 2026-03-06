use serde::{Deserialize, Serialize};
use std::env;

pub trait ServiceDiscovery: Send + Sync {
    fn discover(&self, service_name: &str) -> Option<String>;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResolverType {
    Direct,
    Etcd,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectResolverConfig {
    pub services: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtcdResolverConfig {
    pub endpoints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolverConfig {
    #[serde(default)]
    pub resolver_type: Option<ResolverType>,
    pub direct: Option<DirectResolverConfig>,
    pub etcd: Option<EtcdResolverConfig>,
}

pub struct DirectResolver {
    services: std::collections::HashMap<String, String>,
}

impl DirectResolver {
    pub fn from_env() -> Self {
        let mut services = std::collections::HashMap::new();
        let owner_addr =
            env::var("OWNER_SERVICE_ADDR").unwrap_or_else(|_| "http://[::1]:50051".to_string());
        services.insert("owner".to_string(), owner_addr);
        Self { services }
    }

    pub fn new(services: std::collections::HashMap<String, String>) -> Self {
        Self { services }
    }
}

impl ServiceDiscovery for DirectResolver {
    fn discover(&self, service_name: &str) -> Option<String> {
        self.services.get(service_name).cloned()
    }
}

pub struct ResolverFactory;

impl ResolverFactory {
    pub fn create() -> Box<dyn ServiceDiscovery> {
        Box::new(DirectResolver::from_env())
    }
}

pub fn get_service_listen_address(service_name: &str) -> Option<String> {
    match service_name {
        "owner" => Some(
            env::var("OWNER_SERVICE_ADDR").unwrap_or_else(|_| "http://[::1]:50051".to_string()),
        ),
        _ => None,
    }
}

#[cfg(test)]
mod test;
