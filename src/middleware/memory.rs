// ⚠️ DEPRECADO: Este middleware NO mejora el rendimiento.
//
// Problemas del código original:
// 1. refresh_memory() NO libera memoria, solo lee estadísticas del SO
// 2. Añade latencia innecesaria a cada request (~5-10ms)
// 3. Rust maneja memoria automáticamente con ownership/borrowing
// 4. spawn_blocking() crea overhead sin beneficio
//
// Para monitoreo de memoria real, usar:
// - Prometheus metrics con tower-http
// - Tokio console para debugging
// - jemalloc como allocator global (ver Cargo.toml)

use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};

/// Middleware de métricas de rendimiento (sin overhead significativo)
pub async fn performance_metrics(req: Request, next: Next) -> Response {
    let start = std::time::Instant::now();
    let method = req.method().clone();
    let uri = req.uri().clone();
    
    let response = next.run(req).await;
    
    let elapsed = start.elapsed();
    
    // Solo loguear requests lentos (>100ms)
    if elapsed.as_millis() > 100 {
        tracing::warn!(
            method = %method,
            uri = %uri,
            duration_ms = elapsed.as_millis(),
            "Slow request detected"
        );
    }
    
    response
}
