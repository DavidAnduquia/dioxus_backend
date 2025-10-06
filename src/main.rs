use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod database;
mod entities;
mod handlers;
mod middleware;
mod models;
mod routes;
mod services;
mod utils;

use config::Config;
use routes::create_app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_api_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    let port = config.port;

    let db_pool = database::create_pool(&config.database_url).await?;

    // Run migrations
    //database::run_migrations(&db_pool).await?;

    // Create application state
    let app_state = models::AppState {
        db: db_pool,
        config: config.clone(),
        jwt_encoding_key: jsonwebtoken::EncodingKey::from_secret(config.jwt_secret.as_ref()),
        jwt_decoding_key: jsonwebtoken::DecodingKey::from_secret(config.jwt_secret.as_ref()),
    };

    // Build our application with routes and middleware
    let app = create_app()
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
                .layer(middleware::auth::auth_layer()),
        )
        .with_state(app_state);

    // Run the server with graceful shutdown
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("🚀 Server started on http://{}", addr);
    tracing::info!("📚 Swagger UI available at http://{}/swagger-ui", addr);
    tracing::info!("Press Ctrl+C to shutdown gracefully");

    // Serve with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("✅ Server shutdown complete");
    Ok(())
}

/// Handle shutdown signals (Ctrl+C) gracefully
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("🛑 Received Ctrl+C signal, shutting down gracefully...");
        },
        _ = terminate => {
            tracing::info!("🛑 Received SIGTERM signal, shutting down gracefully...");
        },
    }
}