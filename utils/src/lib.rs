use std::time::{SystemTime, UNIX_EPOCH};

pub mod error;

pub use error::{Error, Result};

pub fn current_timestamp_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}
