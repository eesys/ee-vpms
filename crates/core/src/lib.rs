//! Core module for ee-vpms
//!
//! This module provides essential services, data models, and error handling for the ee-vpms application.
//! It includes:
//! - Database connection and initialization
//! - Error types for the application
//! - Entity definitions (currently Owner entity)
//! - Business logic services (service layer with Owner, Pet, etc.)

pub mod db;
pub mod error;
pub mod entity;
pub mod service;

pub use error::{Error, Result};
