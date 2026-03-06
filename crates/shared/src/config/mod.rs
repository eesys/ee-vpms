use serde::{Deserialize, Serialize};
use std::env;

pub trait ServiceDiscovery: Send + Sync {
    fn discover(&self, service_name: &str) -> Option<String>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub services: std::collections::HashMap<String, ServiceEndpoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    pub address: String,
}

impl ServiceConfig {
    pub fn from_env() -> Self {
        let mut services = std::collections::HashMap::new();

        let owner_service_addr =
            env::var("OWNER_SERVICE_ADDR").unwrap_or_else(|_| "http://[::1]:50051".to_string());
        services.insert(
            "owner".to_string(),
            ServiceEndpoint {
                address: owner_service_addr,
            },
        );

        Self { services }
    }
}

impl ServiceDiscovery for ServiceConfig {
    fn discover(&self, service_name: &str) -> Option<String> {
        self.services
            .get(service_name)
            .map(|endpoint| endpoint.address.clone())
    }
}

#[cfg(test)]
mod test;
