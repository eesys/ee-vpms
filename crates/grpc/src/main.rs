use ee_vpms_grpc::{OwnerServiceServer, owner::OwnerGrpcService};
use ee_vpms_shared::service::ServiceRegistry;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let db = ee_vpms_core::db::init().await?;
    let owner_service = OwnerGrpcService { db };

    let mut registry = ServiceRegistry::new();
    registry.register("owner-service", "[::1]:50051");

    let addr = "[::1]:50051".parse()?;
    tracing::info!(
        "Starting microservice: {} on {}",
        "owner-service",
        addr
    );

    Server::builder()
        .add_service(OwnerServiceServer::new(owner_service))
        .serve(addr)
        .await?;

    Ok(())
}

