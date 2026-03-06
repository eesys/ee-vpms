use axum::{Router, routing::*};
use sea_orm::DbConn;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

pub mod handlers;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DbConn>,
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

    let db = ee_vpms_core::db::init().await?;
    let state = AppState { db: Arc::new(db) };
    let app = create_router(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}
