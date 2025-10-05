use axum::{
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde_json::{json, Value};

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

async fn health_check() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION")
    })))
}
