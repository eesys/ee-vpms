use ee_vpms_owner::grpc::OwnerGrpcService;
use ee_vpms_owner::pb::owner::owner_service_server::OwnerServiceServer;
use ee_vpms_shared::get_service_listen_address;
use ee_vpms_shared::service::ServiceRegistry;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let db = ee_vpms_owner::db::init().await?;
    let owner_service = OwnerGrpcService { db };

    let listen_addr = get_service_listen_address("owner").ok_or_else(|| {
        Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Owner service listen address not configured",
        )) as Box<dyn std::error::Error>
    })?;

    let mut registry = ServiceRegistry::new();
    registry.register("owner", &listen_addr);

    let addr = listen_addr.parse()?;
    tracing::info!("Starting microservice: owner on {}", addr);

    Server::builder()
        .add_service(OwnerServiceServer::new(owner_service))
        .serve(addr)
        .await?;

    Ok(())
}
