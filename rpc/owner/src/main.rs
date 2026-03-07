use ee_vpms_owner::grpc::OwnerGrpcService;
use ee_vpms_owner::pb::owner::owner_service_server::OwnerServiceServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let db = ee_vpms_owner::db::init().await?;
    let owner_service = OwnerGrpcService { db };

    let listen_addr = "localhost:8080";

    let addr = listen_addr.parse()?;
    tracing::info!("Starting microservice: owner on {}", addr);

    Server::builder()
        .add_service(OwnerServiceServer::new(owner_service))
        .serve(addr)
        .await?;

    Ok(())
}
