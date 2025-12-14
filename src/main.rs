use std::net::SocketAddr;
use std::sync::Arc;

use axum::http::{HeaderName, Method};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::{limit::RequestBodyLimitLayer, trace::TraceLayer};
mod config;
mod database;
mod handlers;
mod middleware;
mod models;
mod routes;
mod services;
mod utils;

use config::Config;
use database::{seeder, DbExecutor, init_schema};
use routes::create_app;

#[tokio::main(worker_threads = 2)]
//use std::io;
//#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inicializar logger con persistencia en archivos (zona horaria Bogotá UTC-5)
    // Para máxima optimización sin archivos, usar: utils::logger::init_logger_console_only()?;
    utils::logger::init_logger("logs", "rust-api-backend")?;

    // Load configuration
    let config = Arc::new(Config::from_env()?);
    let port = config.port;

    // Intentar conectar a la base de datos, pero no fallar si no se puede
    let db_pool = match database::create_pool(&config.database_url).await {
        Ok(pool) => {
            tracing::info!("✅ Base de datos conectada correctamente");

            // Inicializar schema rustdema2 (search_path)
            if let Err(e) = init_schema(&pool).await {
                tracing::error!("❌ Error inicializando schema: {}", e);
                return Err(e.into());
            }

            // Ejecutar migraciones iniciales para preparar tablas requeridas
            if let Err(e) = seeder::run_migrations(&pool, &config.database_url).await {
                tracing::error!("❌ Migraciones fallaron: {}", e);
                return Err(e.into());
            }

            Some(pool)
        }
        Err(e) => {
            tracing::error!("❌ No se pudo conectar a la base de datos: {}", e);
            tracing::warn!("⚠️  El servidor iniciará sin conexión a la base de datos");
            tracing::warn!(
                "⚠️  Las peticiones que requieran DB retornarán 503 Service Unavailable"
            );
            None
        }
    };

    // Create application state
    let db_executor = db_pool.map(DbExecutor::from_pool);

    let jwt_secret = config.jwt_secret.as_bytes();
    let jwt_encoding_key = Arc::new(jsonwebtoken::EncodingKey::from_secret(jwt_secret));
    let jwt_decoding_key = Arc::new(jsonwebtoken::DecodingKey::from_secret(jwt_secret));

    let app_state = models::AppState {
        db: db_executor,
        config: Arc::clone(&config),
        jwt_encoding_key,
        jwt_decoding_key,
    };

    // Medición definitiva de Arc
    let arc_size = std::mem::size_of::<Arc<Config>>();
    let inner_size = std::mem::size_of_val(app_state.config.as_ref());
    tracing::info!(
        "🔍 Medición definitiva: Arc: {} bytes, Datos: {} bytes",
        arc_size,
        inner_size
    );

    // Build our application with routes and middleware
    let app = create_app()
        .layer(
            ServiceBuilder::new()
                // Límite de body reducido a 512KB (optimización memoria)
                .layer(RequestBodyLimitLayer::new(512 * 1024))
                // Logging de requests HTTP
                .layer(TraceLayer::new_for_http())
                // CORS configurado para desarrollo (permite localhost:8080 del frontend)
                .layer(
                    CorsLayer::new()
                        .allow_origin([
                            "http://localhost:8080".parse().unwrap(), // Frontend Dioxus
                            "http://127.0.0.1:8080".parse().unwrap(), // Frontend alternativo
                        ]) // Orígenes específicos para desarrollo
                        .allow_methods([
                            Method::GET,
                            Method::POST,
                            Method::PUT,
                            Method::DELETE,
                            Method::OPTIONS,
                        ]) // Métodos HTTP específicos
                        .allow_headers([
                            HeaderName::from_static("authorization"),
                            HeaderName::from_static("content-type"),
                            HeaderName::from_static("accept"),
                            HeaderName::from_static("cache-control"),
                        ]) // Headers específicos para JWT
                        .allow_credentials(true), // Permite credenciales (cookies, auth headers)
                ),
        )
        // Métricas de performance (solo requests lentos)
        .layer(axum::middleware::from_fn(
            middleware::memory::performance_metrics,
        ))
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
            perform_graceful_cleanup().await;
        },
        _ = terminate => {
            tracing::info!("🛑 Received SIGTERM signal, shutting down gracefully...");
            perform_graceful_cleanup().await;
        },
    }
}

/// /* Cambio nuevo */ Realiza limpieza ordenada de todos los recursos
async fn perform_graceful_cleanup() {
    tracing::info!("🧹 Iniciando limpieza ordenada de recursos...");

    // Optimizar memoria de SocketService antes del cierre
    let socket_service = services::socket_service::get_socket_service();
    let optimized = socket_service.optimize_memory().await;
    if optimized > 0 {
        tracing::info!(
            "🔧 Optimizados {} usuarios en SocketService durante shutdown",
            optimized
        );
    }

    // Mostrar métricas finales
    let socket_metrics = socket_service.get_memory_metrics().await;
    tracing::info!(
        "📊 Métricas finales SocketService: {} usuarios, {} conexiones, {} overhead",
        socket_metrics.total_users,
        socket_metrics.total_connections,
        socket_metrics.memory_overhead
    );

    // /* Cambio nuevo */ Limpiar cron jobs activos
    let jobs_cleaned = services::cron_service::cleanup_all_jobs();
    if jobs_cleaned > 0 {
        tracing::info!("🛑 {} cron jobs limpiados durante shutdown", jobs_cleaned);
    }

    // Flush final de logs antes de cerrar
    if let Err(e) = utils::logger::flush_logs() {
        eprintln!("⚠️  Error al hacer flush de logs: {}", e);
    }

    tracing::info!("✅ Limpieza ordenada completada");
}
