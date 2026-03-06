use std::time::{SystemTime, UNIX_EPOCH};

pub mod owner;

pub use owner::OwnerService;

pub fn current_timestamp_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}
