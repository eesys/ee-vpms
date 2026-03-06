pub mod owner;
pub mod pb {
    tonic::include_proto!("owner");
}

pub use owner::OwnerGrpcService;
pub use pb::owner_service_server::OwnerServiceServer;
