use axum::{routing::{get, post, put}, Router};

use crate::{handlers::usuarios, models::AppState};

pub fn usuarios_routes() -> Router<AppState> {
    Router::new()
        .route("/api/usuarios", get(usuarios::listar_usuarios).post(usuarios::crear_usuario))
        .route(
            "/api/usuarios/:id",
            get(usuarios::obtener_usuario_por_id).put(usuarios::actualizar_usuario),
        )
        .route(
            "/api/usuarios/logout/:id",
            post(usuarios::logout_usuario),
        )
        .route(
            "/api/usuarios/login",
            post(usuarios::login_usuario),
        )
}
