use axum::{
    extract::{FromRef, Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    models::{examen::Model as ExamenModel, AppState},
    services::examen_service::{ActualizarExamen, ExamenService, NuevoExamen},
    utils::errors::AppError,
};

pub async fn crear_examen(
    State(state): State<AppState>,
    Json(payload): Json<NuevoExamen>,
) -> Result<(StatusCode, Json<ExamenModel>), AppError> {
    let service = ExamenService::from_ref(&state);
    let examen = service.crear_examen(payload).await?;
    Ok((StatusCode::CREATED, Json(examen)))
}

pub async fn listar_examenes_por_curso(
    State(state): State<AppState>,
    Path(curso_id): Path<i32>,
) -> Result<Json<Vec<ExamenModel>>, AppError> {
    let service = ExamenService::from_ref(&state);
    let examenes = service
        .obtener_examenes_por_curso(curso_id)
        .await
        .map_err(AppError::from)?;
    Ok(Json(examenes))
}

pub async fn obtener_examen(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ExamenModel>, AppError> {
    let service = ExamenService::from_ref(&state);
    match service.obtener_examen_por_id(id).await.map_err(AppError::from)? {
        Some(examen) => Ok(Json(examen)),
        None => Err(AppError::NotFound(format!("Examen {} no encontrado", id).into())),
    }
}

pub async fn actualizar_examen(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<ActualizarExamen>,
) -> Result<Json<ExamenModel>, AppError> {
    let service = ExamenService::from_ref(&state);
    let examen = service.actualizar_examen(id, payload).await?;
    Ok(Json(examen))
}

pub async fn eliminar_examen(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let service = ExamenService::from_ref(&state);
    service.eliminar_examen(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
