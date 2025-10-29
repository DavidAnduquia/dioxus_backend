use axum::{
    extract::{FromRef, Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    middleware::auth::AuthUser,
    models::{
        actividad::{Model as ActividadModel, NewActividad, UpdateActividad},
        AppState,
    },
    services::actividad_service::ActividadService,
    utils::errors::AppError,
};

pub async fn listar_actividades(
    _auth_user: AuthUser,  // Validar JWT automáticamente
    State(state): State<AppState>,
) -> Result<Json<Vec<ActividadModel>>, AppError> {
    let service = ActividadService::from_ref(&state);
    let actividades = service.obtener_actividades().await?;
    Ok(Json(actividades))
}

pub async fn listar_actividades_por_curso(
    _auth_user: AuthUser,  // Validar JWT automáticamente
    State(state): State<AppState>,
    Path(curso_id): Path<i32>,
) -> Result<Json<Vec<ActividadModel>>, AppError> {
    let service = ActividadService::from_ref(&state);
    let actividades = service.obtener_actividades_por_curso(curso_id).await?;
    Ok(Json(actividades))
}

pub async fn obtener_actividad(
    _auth_user: AuthUser,  // Validar JWT automáticamente
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ActividadModel>, AppError> {
    let service = ActividadService::from_ref(&state);
    match service.obtener_actividad_por_id(id).await? {
        Some(actividad) => Ok(Json(actividad)),
        None => Err(AppError::NotFound(std::borrow::Cow::Owned(format!("Actividad {} no encontrada", id)))),
    }
}

pub async fn crear_actividad(
    _auth_user: AuthUser,  // Validar JWT automáticamente
    State(state): State<AppState>,
    Json(payload): Json<NewActividad>,
) -> Result<(StatusCode, Json<ActividadModel>), AppError> {
    let service = ActividadService::from_ref(&state);
    let actividad = service.crear_actividad(payload).await?;
    Ok((StatusCode::CREATED, Json(actividad)))
}

pub async fn actualizar_actividad(
    _auth_user: AuthUser,  // Validar JWT automáticamente
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateActividad>,
) -> Result<Json<ActividadModel>, AppError> {
    let service = ActividadService::from_ref(&state);
    let actividad = service.actualizar_actividad(id, payload).await?;
    Ok(Json(actividad))
}

pub async fn eliminar_actividad(
    _auth_user: AuthUser,  // Validar JWT automáticamente
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let service = ActividadService::from_ref(&state);
    service.eliminar_actividad(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
