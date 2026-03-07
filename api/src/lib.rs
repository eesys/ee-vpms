use axum::{Router, routing::*};
use ee_vpms_owner::pb::owner::owner_service_client::OwnerServiceClient;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tonic::transport::Channel;
use tower_http::cors::CorsLayer;

pub mod handlers;

#[derive(Clone)]
pub struct AppState {
    pub owner_client: Arc<OwnerServiceClient<Channel>>,
}

fn create_router(state: AppState) -> Router {
    Router::new()
        .route(
            "/owners",
            get(handlers::list_owners).post(handlers::create_owner),
        )
        .route(
            "/owners/{id}",
            get(handlers::get_owner)
                .patch(handlers::update_owner)
                .delete(handlers::delete_owner),
        )
        .with_state(state)
        .layer(CorsLayer::permissive())
}

pub async fn run() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let channel = Channel::from_static("http://localhost:8080")
        .connect()
        .await?;
    let owner_client = OwnerServiceClient::new(channel);
    let state = AppState {
        owner_client: Arc::new(owner_client),
    };
    let app = create_router(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}
