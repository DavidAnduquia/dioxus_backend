use axum::{routing::{get, post}, Router};

use crate::{handlers::matricula, models::AppState};

pub fn matricula_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/matriculas",
            post(matricula::matricular_estudiante),
        )
        .route(
            "/api/matriculas/{estudiante_id}/{curso_id}",
            post(matricula::desmatricular_estudiante),
        )
        .route(
            "/api/estudiantes/{estudiante_id}/matriculas",
            get(matricula::obtener_matriculas_estudiante),
        )
        .route(
            "/api/cursos/{curso_id}/matriculas",
            get(matricula::obtener_matriculas_curso),
        )
}
