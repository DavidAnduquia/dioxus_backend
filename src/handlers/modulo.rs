use axum::{
    extract::{FromRef, Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    middleware::auth::AuthUser,
    models::{modulo::Model as ModuloModel, AppState},
    services::modulo_service::{ActualizarModulo, ModuloService, NuevoModulo},
    utils::errors::AppError,
};

pub async fn crear_modulo(
    _auth_user: AuthUser, // Validar JWT automáticamente
    State(state): State<AppState>,
    Json(payload): Json<NuevoModulo>,
) -> Result<(StatusCode, Json<ModuloModel>), AppError> {
    let service = ModuloService::from_ref(&state);
    let modulo = service.crear_modulo(payload).await?;
    Ok((StatusCode::CREATED, Json(modulo)))
}

pub async fn listar_modulos_por_curso(
    _auth_user: AuthUser, // Validar JWT automáticamente
    State(state): State<AppState>,
    Path(curso_id): Path<i32>,
) -> Result<Json<Vec<ModuloModel>>, AppError> {
    let service = ModuloService::from_ref(&state);
    let modulos = service.obtener_modulos_por_curso(curso_id).await?;
    Ok(Json(modulos))
}

pub async fn obtener_modulo(
    _auth_user: AuthUser, // Validar JWT automáticamente
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ModuloModel>, AppError> {
    let service = ModuloService::from_ref(&state);
    match service
        .obtener_modulo_por_id(id)
        .await
        .map_err(AppError::from)?
    {
        Some(modulo) => Ok(Json(modulo)),
        None => Err(AppError::NotFound(
            format!("Módulo {} no encontrado", id).into(),
        )),
    }
}

pub async fn actualizar_modulo(
    _auth_user: AuthUser, // Validar JWT automáticamente
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<ActualizarModulo>,
) -> Result<Json<ModuloModel>, AppError> {
    let service = ModuloService::from_ref(&state);
    let modulo = service.actualizar_modulo(id, payload).await?;
    Ok(Json(modulo))
}

pub async fn eliminar_modulo(
    _auth_user: AuthUser, // Validar JWT automáticamente
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let service = ModuloService::from_ref(&state);
    service.eliminar_modulo(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
