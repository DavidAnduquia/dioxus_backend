use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use thiserror::Error;

use crate::models::ApiResponse;

#[derive(Error, Debug)] 
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Validation error: {0}")]
    Validation(#[from] validator::ValidationErrors),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("BCrypt error: {0}")]
    BCrypt(#[from] bcrypt::BcryptError),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    #[allow(dead_code)]
    Forbidden(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Database connection timeout: {0}")]
    #[allow(dead_code)]
    DatabaseTimeout(String),

    #[error("Database connection failed: {0}")]
    #[allow(dead_code)]
    DatabaseConnectionFailed(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("SeaORM database error: {0}")]
    SeaOrm(#[from] sea_orm::DbErr),
}

// Implementaciones From<&str> para optimizar strings sin .to_string()
impl From<&str> for AppError {
    fn from(msg: &str) -> Self {
        Self::BadRequest(msg.to_string())
    }
}

// Específicas para cada variante
impl AppError {
    // pub fn bad_request(msg: impl Into<String>) -> Self {
    //     Self::BadRequest(msg.into())
    // }

    // pub fn unauthorized(msg: impl Into<String>) -> Self {
    //     Self::Unauthorized(msg.into())
    // }

    // pub fn forbidden(msg: impl Into<String>) -> Self {
    //     Self::Forbidden(msg.into())
    // }

    // pub fn not_found(msg: impl Into<String>) -> Self {
    //     Self::NotFound(msg.into())
    // }

    // pub fn conflict(msg: impl Into<String>) -> Self {
    //     Self::Conflict(msg.into())
    // }

    // pub fn internal_server_error(msg: impl Into<String>) -> Self {
    //     Self::InternalServerError(msg.into())
    // }

    // pub fn database_timeout(msg: impl Into<String>) -> Self {
    //     Self::DatabaseTimeout(msg.into())
    // }

    // pub fn database_connection_failed(msg: impl Into<String>) -> Self {
    //     Self::DatabaseConnectionFailed(msg.into())
    // }

    // pub fn service_unavailable(msg: impl Into<String>) -> Self {
    //     Self::ServiceUnavailable(msg.into())
    // }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(ref e) => {
                tracing::error!("Database error: {:?}", e);
                
                // Detectar errores específicos de conexión
                let error_msg = e.to_string();
                if error_msg.contains("PoolTimedOut") || error_msg.contains("timed out") {
                    (
                        StatusCode::SERVICE_UNAVAILABLE,
                        "Database connection timeout. Please try again later.".to_string()
                    )
                } else if error_msg.contains("Connection refused") || error_msg.contains("could not connect") {
                    (
                        StatusCode::SERVICE_UNAVAILABLE,
                        "Database is currently unavailable. Please try again later.".to_string()
                    )
                } else {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
                }
            }
            AppError::Validation(ref e) => {
                let mut message = String::with_capacity(50); // Pre-allocate capacity
                message.push_str("Validation error: ");
                message.push_str(&e.to_string());
                (StatusCode::BAD_REQUEST, message)
            }
            AppError::Jwt(_) => (StatusCode::UNAUTHORIZED, "Invalid token".to_string()),
            AppError::BCrypt(ref e) => {
                tracing::error!("BCrypt error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
            AppError::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            AppError::Unauthorized(message) => (StatusCode::UNAUTHORIZED, message),
            AppError::Forbidden(message) => (StatusCode::FORBIDDEN, message),
            AppError::NotFound(message) => (StatusCode::NOT_FOUND, message),
            AppError::Conflict(message) => (StatusCode::CONFLICT, message),
            AppError::InternalServerError(message) => {
                tracing::error!("Internal server error: {}", message);
                (StatusCode::INTERNAL_SERVER_ERROR, message)
            }
            AppError::DatabaseTimeout(message) => {
                tracing::error!("Database timeout: {}", message);
                (
                    StatusCode::SERVICE_UNAVAILABLE,
                    format!("Database connection timeout: {}", message)
                )
            }
            AppError::DatabaseConnectionFailed(message) => {
                tracing::error!("Database connection failed: {}", message);
                (
                    StatusCode::SERVICE_UNAVAILABLE,
                    format!("Database connection failed: {}", message)
                )
            }
            AppError::ServiceUnavailable(message) => {
                tracing::error!("Service unavailable: {}", message);
                (StatusCode::SERVICE_UNAVAILABLE, message)
            }
            AppError::SeaOrm(ref e) => {
                tracing::error!("SeaORM database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
            }
        };

        let body = Json(ApiResponse::<()>::error(message));
        (status, body).into_response()
    }
}
