//! API request handlers
//!
//! This module contains all HTTP request handlers for the API endpoints.
//! Each resource type (e.g., Owner) has its own submodule.

pub mod owner;

pub use owner::*;
