use axum::{
    routing::{get, post, delete},
    Router,
};

use crate::{handlers::storage, models::AppState};

pub fn storage_routes() -> Router<AppState> {
    Router::new()
        .route("/api/storage/upload", post(storage::upload_file_direct))
        .route("/api/storage/presigned-url", post(storage::generate_upload_url))
        .route("/api/storage/download/{file_key}", get(storage::download_file))
        .route("/api/storage/{file_key}", delete(storage::delete_file))
}
