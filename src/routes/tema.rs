use axum::{routing::{get, post}, Router};

use crate::{handlers::tema, models::AppState};

pub fn tema_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/temas",
            post(tema::crear_tema),
        )
        .route(
            "/api/modulos/{modulo_id}/temas",
            get(tema::listar_temas_por_modulo),
        )
        .route(
            "/api/temas/{id}",
            get(tema::obtener_tema)
                .put(tema::actualizar_tema)
                .delete(tema::eliminar_tema),
        )
}
