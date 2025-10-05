use axum::{
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde_json::{json, Value};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{handlers, models::AppState};

pub fn create_routes() -> Router<AppState> {
    Router::new()
        // Health check
        .route("/health", get(health_check))
        // Auth routes
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/login", post(handlers::auth::login))
        // User routes
        .route("/users/me", get(handlers::users::get_current_user))
        // Post routes
        .route("/posts", get(handlers::posts::get_posts))
        .route("/posts", post(handlers::posts::create_post))
        .route("/posts/:id", get(handlers::posts::get_post))
        .route("/posts/:id", put(handlers::posts::update_post))
        .route("/posts/:id", delete(handlers::posts::delete_post))
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Rust API Backend",
        description = "API backend con autenticación JWT y documentación automática",
        version = "1.0.0"
    ),
    paths(
        handlers::auth::register,
        handlers::auth::login,
        handlers::users::get_current_user,
        health_check
    ),
    components(
        schemas(
            crate::models::CreateUserRequest,
            crate::models::LoginRequest,
            crate::models::AuthResponse,
            crate::models::UserResponse,
            crate::models::ApiResponse<crate::models::AuthResponse>,
            crate::models::ApiResponse<crate::models::UserResponse>
        )
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub struct ApiDoc;

pub fn create_app() -> Router<AppState> {
    Router::new()
        .merge(create_routes())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Estado del servidor", body = Value)
    )
)]
async fn health_check() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION")
    })))
}
