mod config;
mod middleware;
mod response;
mod router;

use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::ConfigManager;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "mockserver=info,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config_path = std::env::var("CONFIG_PATH")
        .unwrap_or_else(|_| "config/endpoints.yaml".to_string());
    
    let config_manager = Arc::new(ConfigManager::new(&config_path).await
        .expect("Failed to load configuration"));

    let server_config = config_manager.get_config().server.clone();

    // Build the router
    let app = Router::new()
        .fallback(crate::router::handle_dynamic_request)
        .layer(CorsLayer::permissive())
        .with_state(config_manager);

    let addr = SocketAddr::new(
        server_config.host.parse().expect("Invalid host"),
        server_config.port,
    );

    tracing::info!("MockServer running on http://{}", addr);
    tracing::info!("Config loaded from: {}", config_path);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
