use axum::{
    extract::{FromRef, Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;

use crate::{
    models::AppState,
    services::matricula_service::MatriculaService,
    utils::errors::AppError,
};

#[derive(Deserialize)]
pub struct MatriculaPayload {
    pub estudiante_id: i32,
    pub curso_id: i32,
}

pub async fn matricular_estudiante(
    State(state): State<AppState>,
    Json(payload): Json<MatriculaPayload>,
) -> Result<(StatusCode, Json<crate::models::historial_curso_estudiante::Model>), AppError> {
    let service = MatriculaService::from_ref(&state);
    let matricula = service
        .matricular_estudiante(payload.estudiante_id, payload.curso_id)
        .await?;
    Ok((StatusCode::CREATED, Json(matricula)))
}

pub async fn desmatricular_estudiante(
    State(state): State<AppState>,
    Path((estudiante_id, curso_id)): Path<(i32, i32)>,
) -> Result<Json<crate::models::historial_curso_estudiante::Model>, AppError> {
    let service = MatriculaService::from_ref(&state);
    let matricula = service
        .desmatricular_estudiante(estudiante_id, curso_id)
        .await?;
    Ok(Json(matricula))
}

pub async fn obtener_matriculas_estudiante(
    State(state): State<AppState>,
    Path(estudiante_id): Path<i32>,
) -> Result<Json<Vec<crate::models::historial_curso_estudiante::Model>>, AppError> {
    let service = MatriculaService::from_ref(&state);
    let matriculas = service
        .obtener_matriculas_estudiante(estudiante_id)
        .await?;
    Ok(Json(matriculas))
}

pub async fn obtener_matriculas_curso(
    State(state): State<AppState>,
    Path(curso_id): Path<i32>,
) -> Result<Json<Vec<crate::models::historial_curso_estudiante::Model>>, AppError> {
    let service = MatriculaService::from_ref(&state);
    let matriculas = service
        .obtener_matriculas_curso(curso_id)
        .await?;
    Ok(Json(matriculas))
}
