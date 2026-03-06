pub mod config;
pub mod error;
pub mod service;

pub use config::{
    DirectResolver, ResolverFactory, ResolverType, ServiceDiscovery,
    ServiceRegistryConfig, DirectResolverConfig, EtcdResolverConfig, ServiceDescriptor,
    get_service_listen_address,
};
pub use error::{Error, Result};
pub use service::current_timestamp_millis;
