use axum::{routing::{get, post}, Router};

use crate::{handlers::modulo, models::AppState};

pub fn modulo_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/modulos",
            post(modulo::crear_modulo),
        )
        .route(
            "/api/cursos/{curso_id}/modulos",
            get(modulo::listar_modulos_por_curso),
        )
        .route(
            "/api/modulos/{id}",
            get(modulo::obtener_modulo)
                .put(modulo::actualizar_modulo)
                .delete(modulo::eliminar_modulo),
        )
}
