//! gRPC service definitions for ee-vpms
//!
//! This crate provides gRPC service implementations for the ee-vpms application.
//! It includes service definitions and implementations using tonic framework.

pub mod owner;
pub mod pb {
    tonic::include_proto!("owner");
}

pub use pb::owner_service_server::OwnerServiceServer;
