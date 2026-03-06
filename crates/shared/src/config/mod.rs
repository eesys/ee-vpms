use crate::consts::{defaults, services};
use config::{Config, Environment, File, FileFormat};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

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
    pub services: HashMap<String, ServiceDescriptor>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatabaseConfig {
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistryConfig {
    #[serde(default)]
    pub direct: Option<DirectResolverConfig>,
    #[serde(default)]
    pub etcd: Option<EtcdResolverConfig>,
    #[serde(default)]
    pub database: Option<DatabaseConfig>,
}

pub struct DirectResolver {
    services: HashMap<String, String>,
}

impl DirectResolver {
    pub fn from_env() -> Self {
        let mut services = HashMap::new();
        let owner_addr = env::var("OWNER_SERVICE_ADDR")
            .unwrap_or_else(|_| defaults::OWNER_SERVICE_ADDR.to_string());
        services.insert(services::OWNER.to_string(), owner_addr);
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

    pub fn new(services: HashMap<String, String>) -> Self {
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
        Self::create_for_service("", config_file)
    }

    pub fn create_for_service(service_name: &str, config_file: &str) -> Box<dyn ServiceDiscovery> {
        match Self::load_config_for_service(service_name, config_file) {
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

    pub fn load_config(
        config_file: &str,
    ) -> Result<ServiceRegistryConfig, Box<dyn std::error::Error>> {
        Self::load_config_for_service("", config_file)
    }

    pub fn load_config_for_service(
        service_name: &str,
        config_file: &str,
    ) -> Result<ServiceRegistryConfig, Box<dyn std::error::Error>> {
        let mut builder = Config::builder().add_source(config::File::from_str(
            &Self::default_config(),
            FileFormat::Toml,
        ));

        let config_path = if !service_name.is_empty() {
            format!("crates/{}/{}", service_name, config_file)
        } else {
            config_file.to_string()
        };

        builder = builder.add_source(File::new(&config_path, FileFormat::Toml).required(false));

        // Add environment variable overrides: VPMS_{SERVICE}_{KEY}
        builder = builder.add_source(
            Environment::with_prefix("VPMS")
                .try_parsing(true)
                .separator("_"),
        );

        let config = builder.build()?;
        let service_config: ServiceRegistryConfig = config.try_deserialize()?;

        tracing::debug!("Loaded config from: {}", config_path);
        Ok(service_config)
    }

    fn default_config() -> String {
        format!(
            r#"
[direct]
[direct.services]

[direct.services.{}]
addr = "{}"
resolver_type = "direct"

[database]
url = "{}"
"#,
            services::OWNER,
            defaults::OWNER_SERVICE_ADDR,
            defaults::DEFAULT_DATABASE_URL
        )
    }
}

pub fn get_service_listen_address(service_name: &str) -> Option<String> {
    match service_name {
        name if name == services::OWNER => Some(
            env::var("OWNER_SERVICE_ADDR")
                .unwrap_or_else(|_| defaults::OWNER_SERVICE_ADDR.to_string()),
        ),
        _ => None,
    }
}

pub fn get_service_listen_address_with_config(
    service_name: &str,
    config_file: &str,
) -> Option<String> {
    // Priority: ENV > config file > defaults
    if let Ok(addr) = env::var(format!("{}_SERVICE_ADDR", service_name.to_uppercase())) {
        return Some(addr);
    }

    if let Ok(config) = ResolverFactory::load_config_for_service(service_name, config_file) {
        if let Some(direct_config) = config.direct {
            if let Some(descriptor) = direct_config.services.get(service_name) {
                return Some(descriptor.addr.clone());
            }
        }
    }

    get_service_listen_address(service_name)
}

pub fn get_database_url(
    service_name: &str,
    config_file: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Priority: ENV > config file > defaults
    if let Ok(url) = env::var("VPMS_DATABASE_URL") {
        return Ok(url);
    }

    let config = ResolverFactory::load_config_for_service(service_name, config_file)?;

    if let Some(db_config) = config.database {
        if !db_config.url.is_empty() {
            return Ok(db_config.url);
        }
    }

    Err("Database URL not configured".into())
}

#[cfg(test)]
mod test;
