use axum::{routing::{delete, get, post, put}, Router};

use crate::{handlers::examen, models::AppState};

pub fn examen_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/examenes",
            post(examen::crear_examen),
        )
        .route(
            "/api/cursos/{curso_id}/examenes",
            get(examen::listar_examenes_por_curso),
        )
        .route(
            "/api/examenes/{id}",
            get(examen::obtener_examen)
                .put(examen::actualizar_examen)
                .delete(examen::eliminar_examen),
        )
}
