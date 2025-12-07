use axum::{
    extract::{FromRef, Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    middleware::auth::AuthUser,
    models::{unidad::Model as UnidadModel, AppState},
    services::unidad_service::{ActualizarUnidad, NuevaUnidad, UnidadService},
    utils::errors::AppError,
};

pub async fn crear_unidad(
    _auth_user: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<NuevaUnidad>,
) -> Result<(StatusCode, Json<UnidadModel>), AppError> {
    let service = UnidadService::from_ref(&state);
    let unidad = service.crear_unidad(payload).await?;
    Ok((StatusCode::CREATED, Json(unidad)))
}

pub async fn listar_unidades_por_tema(
    _auth_user: AuthUser,
    State(state): State<AppState>,
    Path(tema_id): Path<i32>,
) -> Result<Json<Vec<UnidadModel>>, AppError> {
    let service = UnidadService::from_ref(&state);
    let unidades = service
        .obtener_unidades_por_tema(tema_id)
        .await
        .map_err(AppError::from)?;
    Ok(Json(unidades))
}

pub async fn obtener_unidad(
    _auth_user: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<UnidadModel>, AppError> {
    let service = UnidadService::from_ref(&state);
    match service
        .obtener_unidad_por_id(id)
        .await
        .map_err(AppError::from)?
    {
        Some(unidad) => Ok(Json(unidad)),
        None => Err(AppError::NotFound(format!("Unidad {} no encontrada", id).into())),
    }
}

pub async fn actualizar_unidad(
    _auth_user: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<ActualizarUnidad>,
) -> Result<Json<UnidadModel>, AppError> {
    let service = UnidadService::from_ref(&state);
    let unidad = service.actualizar_unidad(id, payload).await?;
    Ok(Json(unidad))
}

pub async fn eliminar_unidad(
    _auth_user: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let service = UnidadService::from_ref(&state);
    service.eliminar_unidad(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
