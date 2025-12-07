use axum::{routing::{get, post}, Router};

use crate::{handlers::unidad, models::AppState};

pub fn unidad_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/unidades",
            post(unidad::crear_unidad),
        )
        .route(
            "/api/temas/{tema_id}/unidades",
            get(unidad::listar_unidades_por_tema),
        )
        .route(
            "/api/unidades/{id}",
            get(unidad::obtener_unidad)
                .put(unidad::actualizar_unidad)
                .delete(unidad::eliminar_unidad),
        )
}
