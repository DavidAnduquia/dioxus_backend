use axum::routing::get;
use axum::Router;

use crate::{handlers::actividad, models::AppState};

pub fn actividad_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/actividades",
            get(actividad::listar_actividades).post(actividad::crear_actividad),
        )
        .route(
            "/api/actividades/{id}",
            get(actividad::obtener_actividad)
                .put(actividad::actualizar_actividad)
                .delete(actividad::eliminar_actividad),
        )
        .route(
            "/api/cursos/{curso_id}/actividades",
            get(actividad::listar_actividades_por_curso),
        )
}
