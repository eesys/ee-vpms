use clap::Parser;
use config::Config;
use ee_vpms_owner::grpc::OwnerGrpcService;
use ee_vpms_owner::pb::owner::owner_service_server::OwnerServiceServer;
use tonic::transport::Server;

mod conf;
use conf::RpcConfig;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short = 'f', default_value = "config.yaml")]
    config_file: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    let c = Config::builder()
        .add_source(config::File::with_name(&args.config_file))
        .build()?;
    let c: RpcConfig = c.try_deserialize()?;

    let db = ee_vpms_owner::db::init(c.data_source).await?;
    let service = OwnerGrpcService { db };

    let listen_addr = "localhost:8080";

    let addr = listen_addr.parse()?;
    tracing::info!("Starting microservice: owner on {}", addr);

    Server::builder()
        .add_service(OwnerServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
