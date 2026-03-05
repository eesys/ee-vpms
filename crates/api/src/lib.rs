//! ee-vpms REST API server
//!
//! This module provides the HTTP server implementation for the ee-vpms application.
//! It sets up routing, middleware, and the necessary handlers for API endpoints.

use axum::{Router, routing::*};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber;

pub mod handlers;

/// Create the main application router
///
/// Configures all routes and middleware for the API server:
/// - GET  /owners      - List all owners
/// - POST /owners      - Create a new owner
/// - GET  /owners/{id} - Get a specific owner
///
/// Enables CORS with permissive settings for development.
fn create_router() -> Router {
    let routes = Router::new()
        .route(
            "/owners",
            get(handlers::list_owners).post(handlers::create_owner),
        )
        .route("/owners/{id}", get(handlers::get_owner))
        .layer(CorsLayer::permissive());
    routes
}

/// Start the API server
///
/// Initializes the environment, sets up logging, and starts listening for requests.
/// The server binds to 127.0.0.1:3000 by default.
///
/// # Returns
/// - `Ok(())` if server runs successfully
/// - `Err(anyhow::Error)` if server startup fails
pub async fn run() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let app = create_router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}

#[cfg(test)]
mod test;
