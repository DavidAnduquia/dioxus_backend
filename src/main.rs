use std::net::SocketAddr;
use std::sync::Arc;

use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer, 
    trace::TraceLayer,
    compression::CompressionLayer,
    limit::RequestBodyLimitLayer,
};
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
use database::DbExecutor;
use routes::create_app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inicializar logger con archivo en `logs/`
    utils::logger::init_logger("logs", "rust-api-backend")?;

    // Limpiar logs antiguos (más de 30 días)
    if let Err(e) = utils::logger::cleanup_old_logs("logs", 30) {
        tracing::warn!("⚠️  No se pudieron limpiar los logs antiguos: {}", e);
    }

    // Load configuration
    let config = Arc::new(Config::from_env()?);
    let port = config.port;

    // Intentar conectar a la base de datos, pero no fallar si no se puede
    let db_pool = match database::create_pool(&config.database_url).await {
        Ok(pool) => {
            tracing::info!("✅ Base de datos conectada correctamente");
            Some(pool)
        }
        Err(e) => {
            tracing::error!("❌ No se pudo conectar a la base de datos: {}", e);
            tracing::warn!("⚠️  El servidor iniciará sin conexión a la base de datos");
            tracing::warn!("⚠️  Las peticiones que requieran DB retornarán 503 Service Unavailable");
            None
        }
    };

    // Run migrations
    //database::run_migrations(&db_pool).await?;
    // Create application state
    let db_executor = db_pool.map(DbExecutor::from_pool);

    let app_state = models::AppState {
        db: db_executor,
        config: Arc::clone(&config),
        jwt_encoding_key: jsonwebtoken::EncodingKey::from_secret(config.jwt_secret.as_ref()),
        jwt_decoding_key: jsonwebtoken::DecodingKey::from_secret(config.jwt_secret.as_ref()),
    };

    // Build our application with routes and middleware
    let app = create_app()
        .layer(
            ServiceBuilder::new()
                // Límite de body para reducir buffers (2MB para uploads razonables)
                .layer(RequestBodyLimitLayer::new(2 * 1024 * 1024))
                // Compresión automática de respuestas (gzip)
                // Reduce tamaño de respuestas JSON ~70-80%
                .layer(CompressionLayer::new())
                // Logging de requests HTTP
                .layer(TraceLayer::new_for_http())
                // CORS permisivo (ajustar en producción)
                .layer(CorsLayer::permissive())
                // Autenticación JWT
                .layer(middleware::auth::auth_layer())
        )
        // Métricas de performance (solo requests lentos) - fuera del ServiceBuilder
        .layer(axum::middleware::from_fn(middleware::memory::performance_metrics))
        .with_state(app_state);

    // Run the server with graceful shutdown
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("🚀 Server started on http://{}", addr);
    tracing::info!("📚 Swagger UI available at http://{}/swagger-ui", addr);
    tracing::info!("🔌 WebSocket endpoint available at ws://{}/ws", addr);
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