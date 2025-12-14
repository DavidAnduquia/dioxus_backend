use axum::{extract::State, response::Json};
use serde_json::{json, Value};

use crate::{models::AppState, services::socket_service::get_socket_service};

/// /* Cambio nuevo */ Handler para exponer métricas de memoria de singletons
pub async fn get_memory_metrics(State(_state): State<AppState>) -> Json<Value> {
    // Obtener métricas de SocketService
    let socket_service = get_socket_service();
    let socket_metrics = socket_service.get_memory_metrics().await;
    let connection_info = socket_service.get_connection_info().await;

    // Obtener métricas de CronService usando función estática
    let cron_jobs_count = crate::services::cron_service::get_jobs_count();

    Json(json!({
        "timestamp": chrono::Utc::now(),
        "socket_service": {
            "total_users": socket_metrics.total_users,
            "total_connections": socket_metrics.total_connections,
            "total_capacity": socket_metrics.total_capacity,
            "memory_overhead": socket_metrics.memory_overhead,
            "largest_user_connections": socket_metrics.largest_user_connections,
            "rooms": connection_info.rooms
        },
        "cron_service": {
            "active_jobs": cron_jobs_count,
            "running_jobs": cron_jobs_count, // Simplificado por ahora
            "finished_jobs": 0
        },
        "memory_recommendations": {
            "socket_optimization_needed": socket_metrics.memory_overhead > socket_metrics.total_connections,
            "overhead_percentage": if socket_metrics.total_capacity > 0 {
                (socket_metrics.memory_overhead as f64 / socket_metrics.total_capacity as f64) * 100.0
            } else {
                0.0
            }
        }
    }))
}

/// /* Cambio nuevo */ Handler para optimizar memoria manualmente
pub async fn optimize_memory(State(_state): State<AppState>) -> Json<Value> {
    let socket_service = get_socket_service();

    // Obtener métricas antes de la optimización
    let metrics_before = socket_service.get_memory_metrics().await;

    // Ejecutar optimización
    let optimized_users = socket_service.optimize_memory().await;

    // Obtener métricas después de la optimización
    let metrics_after = socket_service.get_memory_metrics().await;

    let memory_saved = metrics_before
        .memory_overhead
        .saturating_sub(metrics_after.memory_overhead);

    Json(json!({
        "timestamp": chrono::Utc::now(),
        "optimization_result": {
            "users_optimized": optimized_users,
            "memory_saved": memory_saved,
            "before": {
                "total_capacity": metrics_before.total_capacity,
                "memory_overhead": metrics_before.memory_overhead
            },
            "after": {
                "total_capacity": metrics_after.total_capacity,
                "memory_overhead": metrics_after.memory_overhead
            }
        }
    }))
}
