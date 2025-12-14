use axum::{
    routing::{delete, post},
    Router,
};

use crate::{handlers::storage, models::AppState};

pub fn storage_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/storage/upload-url",
            post(storage::generate_upload_url),
        )
        .route("/api/storage/upload", post(storage::upload_file_direct))
        .route(
            "/api/storage/files/{file_key}",
            delete(storage::delete_file),
        )
}
