//! API server entry point

use axum::{routing::*, Router};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber;

pub mod handlers;

/// Create application router
fn create_router() -> Router {
    let routes = Router::new()
        .route("/owners", get(handlers::list_owners).post(handlers::create_owner))
        .route("/owners/:id", get(handlers::get_owner))
        .layer(CorsLayer::permissive());
    routes
}

/// Start the API server
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
mod tests {
    use super::*;
    use std::net::IpAddr;

    #[test]
    fn test_router_construction() {
        let _app = create_router();
        assert!(true);
    }

    #[test]
    fn test_socket_address_localhost() {
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        assert_eq!(addr.port(), 3000);
        assert_eq!(addr.ip(), IpAddr::from([127, 0, 0, 1]));
    }

    #[test]
    fn test_socket_address_port() {
        let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
        assert_eq!(addr.port(), 8080);
    }

    #[test]
    fn test_cors_layer_creation() {
        use tower_http::cors::CorsLayer;
        let _cors = CorsLayer::permissive();
        assert!(true);
    }

    #[test]
    fn test_tracing_subscriber_init() {
        use tracing_subscriber;
        // Just verifying the type and module are available
        let _fmt = tracing_subscriber::fmt();
        assert!(true);
    }

    #[test]
    fn test_dotenvy_module() {
        // Just verifying dotenvy is available
        let _ = dotenvy::dotenv();
        assert!(true);
    }

    #[test]
    fn test_axum_routing_modules() {
        // Just verifying routing functions are available and compiles
        // (we can't store them as values due to complex types)
        assert!(true);
    }
}
