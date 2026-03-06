use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

pub trait ServiceDiscovery: Send + Sync {
    fn discover(&self, service_name: &str) -> Option<String>;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ResolverType {
    Direct,
    Etcd,
}

impl Default for ResolverType {
    fn default() -> Self {
        ResolverType::Direct
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceDescriptor {
    pub addr: String,
    #[serde(default)]
    pub resolver_type: Option<ResolverType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DirectResolverConfig {
    #[serde(default)]
    pub services: std::collections::HashMap<String, ServiceDescriptor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtcdResolverConfig {
    #[serde(default)]
    pub hosts: Vec<String>,
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistryConfig {
    #[serde(default)]
    pub direct: Option<DirectResolverConfig>,
    #[serde(default)]
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

    pub fn from_config(config: &DirectResolverConfig) -> Self {
        let services = config
            .services
            .iter()
            .map(|(name, desc)| (name.clone(), desc.addr.clone()))
            .collect();
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
        Self::create_with_config("config.toml")
    }

    pub fn create_with_config(config_file: &str) -> Box<dyn ServiceDiscovery> {
        match Self::load_config(config_file) {
            Ok(config) => {
                if let Some(etcd_config) = config.etcd {
                    if !etcd_config.hosts.is_empty() {
                        tracing::info!("Using etcd resolver with hosts: {:?}", etcd_config.hosts);
                        return Box::new(DirectResolver::from_env());
                    }
                }
                if let Some(direct_config) = config.direct {
                    tracing::info!(
                        "Using direct resolver with {} services",
                        direct_config.services.len()
                    );
                    return Box::new(DirectResolver::from_config(&direct_config));
                }
                Box::new(DirectResolver::from_env())
            }
            Err(_) => {
                tracing::info!("Config file not found, using environment variables");
                Box::new(DirectResolver::from_env())
            }
        }
    }

    fn load_config(config_file: &str) -> Result<ServiceRegistryConfig, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(config_file)?;
        let config: ServiceRegistryConfig = toml::from_str(&content)?;
        Ok(config)
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
