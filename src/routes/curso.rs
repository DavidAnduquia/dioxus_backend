use axum::{routing::get, Router};

use crate::{handlers::curso, models::AppState};

pub fn curso_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/cursos",
            get(curso::listar_cursos).post(curso::crear_curso),
        )
        .route(
            "/api/cursos/{id}",
            get(curso::obtener_curso)
                .put(curso::actualizar_curso)
                .delete(curso::eliminar_curso),
        )
        .route(
            "/api/plantillas/{plantilla_id}/cursos",
            get(curso::cursos_por_plantilla),
        )
        .route(
            "/api/areas-conocimiento/{area_id}/cursos",
            get(curso::cursos_por_area_y_periodo),
        )
        .route(
            "/api/cursos/{id}/aula",
            get(curso::aula_por_curso),
        )
}
