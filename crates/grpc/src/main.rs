use ee_vpms_grpc::{OwnerServiceServer, owner::OwnerGrpcService};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let db = ee_vpms_core::db::init().await?;
    let owner_service = OwnerGrpcService { db };

    let addr = "[::1]:50051".parse()?;
    tracing::info!("Starting gRPC server on {}", addr);

    Server::builder()
        .add_service(OwnerServiceServer::new(owner_service))
        .serve(addr)
        .await?;

    Ok(())
}
