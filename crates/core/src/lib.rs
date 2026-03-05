//! Core module for ee-vpms
//! Provides database, configuration, and error handling

pub mod db;
pub mod error;
pub mod entity;
pub mod owner_service;

pub use error::{Error, Result};
