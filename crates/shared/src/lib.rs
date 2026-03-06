pub mod config;
pub mod error;
pub mod service;

pub use config::{
    DirectResolver, ResolverConfig, ResolverFactory, ResolverType, ServiceDiscovery,
    get_service_listen_address,
};
pub use error::{Error, Result};
pub use service::current_timestamp_millis;
