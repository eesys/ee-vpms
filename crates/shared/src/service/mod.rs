use crate::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_timestamp_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

#[async_trait]
pub trait Service: Send + Sync {
    fn name(&self) -> &'static str;
    fn address(&self) -> &str;
    async fn health_check(&self) -> Result<()>;
}

pub struct ServiceRegistry {
    services: HashMap<String, String>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, address: &str) {
        self.services.insert(name.to_string(), address.to_string());
    }

    pub fn discover(&self, name: &str) -> Option<String> {
        self.services.get(name).cloned()
    }

    pub fn list(&self) -> Vec<String> {
        self.services.keys().cloned().collect()
    }
}

impl Default for ServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test;
