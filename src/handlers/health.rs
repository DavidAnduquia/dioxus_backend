use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::{ApiResponse, AppState};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub database: String,
    pub message: String,
}

/// Health check endpoint
///
/// Verifica el estado del servidor y la conexión a la base de datos
#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "Server is healthy", body = ApiResponse<HealthResponse>),
        (status = 503, description = "Service unavailable", body = ApiResponse<HealthResponse>)
    )
)]
pub async fn health_check(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let db_status = if state.is_db_available() {
        // Intentar hacer una query simple para verificar la conexión
        match state.get_db() {
            Ok(pool) => {
                match sqlx::query("SELECT 1").fetch_one(pool).await {
                    Ok(_) => "connected",
                    Err(_) => "error",
                }
            }
            Err(_) => "disconnected",
        }
    } else {
        "disconnected"
    };

    let (status_code, status_msg) = if db_status == "connected" {
        (StatusCode::OK, "healthy")
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "degraded")
    };

    let response = HealthResponse {
        status: status_msg.to_string(),
        database: db_status.to_string(),
        message: if db_status == "connected" {
            "All systems operational".to_string()
        } else {
            "Database connection unavailable".to_string()
        },
    };

    (status_code, Json(ApiResponse::success(response)))
}

/// Readiness check endpoint
///
/// Verifica si el servidor está listo para recibir tráfico
#[utoipa::path(
    get,
    path = "/ready",
    tag = "Health",
    responses(
        (status = 200, description = "Server is ready"),
        (status = 503, description = "Server is not ready")
    )
)]
pub async fn readiness_check(
    State(state): State<AppState>,
) -> impl IntoResponse {
    if state.is_db_available() {
        match state.get_db() {
            Ok(pool) => {
                match sqlx::query("SELECT 1").fetch_one(pool).await {
                    Ok(_) => (StatusCode::OK, "Ready"),
                    Err(_) => (StatusCode::SERVICE_UNAVAILABLE, "Database not ready"),
                }
            }
            Err(_) => (StatusCode::SERVICE_UNAVAILABLE, "Database not available"),
        }
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "Database not connected")
    }
}

/// Liveness check endpoint
///
/// Verifica si el servidor está vivo (sin verificar dependencias)
#[utoipa::path(
    get,
    path = "/live",
    tag = "Health",
    responses(
        (status = 200, description = "Server is alive")
    )
)]
pub async fn liveness_check() -> impl IntoResponse {
    (StatusCode::OK, "Alive")
}