use axum::{
    extract::{FromRef, Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    middleware::auth::AuthUser,
    models::{area_conocimiento::Model as AreaConocimientoModel, AppState},
    services::area_conocimiento_service::{ActualizarArea, AreaConocimientoService, NuevaArea},
    utils::errors::AppError,
};

#[derive(serde::Deserialize)]
pub struct CambiarEstadoPayload {
    pub estado: bool,
}

pub async fn listar_areas(
    _auth_user: AuthUser, // Validar JWT automáticamente
    State(state): State<AppState>,
) -> Result<Json<Vec<AreaConocimientoModel>>, AppError> {
    let service = AreaConocimientoService::from_ref(&state);
    let areas = service.obtener_areas().await?;
    Ok(Json(areas))
}

pub async fn listar_areas_activas(
    _auth_user: AuthUser, // Validar JWT automáticamente
    State(state): State<AppState>,
) -> Result<Json<Vec<AreaConocimientoModel>>, AppError> {
    let service = AreaConocimientoService::from_ref(&state);
    let areas = service.obtener_areas_activas().await?;
    Ok(Json(areas))
}

pub async fn obtener_area(
    _auth_user: AuthUser, // Validar JWT automáticamente
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<AreaConocimientoModel>, AppError> {
    let service = AreaConocimientoService::from_ref(&state);
    match service.obtener_area_por_id(id).await? {
        Some(area) => Ok(Json(area)),
        None => Err(AppError::NotFound(
            format!("Área de conocimiento con id {} no encontrada", id).into(),
        )),
    }
}

pub async fn crear_area(
    _auth_user: AuthUser, // Validar JWT automáticamente
    State(state): State<AppState>,
    Json(payload): Json<NuevaArea>,
) -> Result<(StatusCode, Json<AreaConocimientoModel>), AppError> {
    let service = AreaConocimientoService::from_ref(&state);
    let area = service.crear_area(payload).await?;
    Ok((StatusCode::CREATED, Json(area)))
}

pub async fn actualizar_area(
    _auth_user: AuthUser, // Validar JWT automáticamente
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<ActualizarArea>,
) -> Result<Json<AreaConocimientoModel>, AppError> {
    let service = AreaConocimientoService::from_ref(&state);
    let area = service.actualizar_area(id, payload).await?;
    Ok(Json(area))
}

pub async fn cambiar_estado(
    _auth_user: AuthUser, // Validar JWT automáticamente
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<CambiarEstadoPayload>,
) -> Result<Json<AreaConocimientoModel>, AppError> {
    let service = AreaConocimientoService::from_ref(&state);
    let area = service.cambiar_estado(id, payload.estado).await?;
    Ok(Json(area))
}

pub async fn eliminar_area(
    _auth_user: AuthUser, // Validar JWT automáticamente
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let service = AreaConocimientoService::from_ref(&state);
    service.eliminar_area(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
