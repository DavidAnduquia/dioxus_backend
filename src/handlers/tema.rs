use axum::{
    extract::{FromRef, Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    middleware::auth::AuthUser,
    models::{tema::Model as TemaModel, AppState},
    services::tema_service::{ActualizarTema, NuevoTema, TemaService},
    utils::errors::AppError,
};

pub async fn crear_tema(
    _auth_user: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<NuevoTema>,
) -> Result<(StatusCode, Json<TemaModel>), AppError> {
    let service = TemaService::from_ref(&state);
    let tema = service.crear_tema(payload).await?;
    Ok((StatusCode::CREATED, Json(tema)))
}

pub async fn listar_temas_por_modulo(
    _auth_user: AuthUser,
    State(state): State<AppState>,
    Path(modulo_id): Path<i32>,
) -> Result<Json<Vec<TemaModel>>, AppError> {
    let service = TemaService::from_ref(&state);
    let temas = service
        .obtener_temas_por_modulo(modulo_id)
        .await
        .map_err(AppError::from)?;
    Ok(Json(temas))
}

pub async fn obtener_tema(
    _auth_user: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<TemaModel>, AppError> {
    let service = TemaService::from_ref(&state);
    match service.obtener_tema_por_id(id).await.map_err(AppError::from)? {
        Some(tema) => Ok(Json(tema)),
        None => Err(AppError::NotFound(format!("Tema {} no encontrado", id).into())),
    }
}

pub async fn actualizar_tema(
    _auth_user: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<ActualizarTema>,
) -> Result<Json<TemaModel>, AppError> {
    let service = TemaService::from_ref(&state);
    let tema = service.actualizar_tema(id, payload).await?;
    Ok(Json(tema))
}

pub async fn eliminar_tema(
    _auth_user: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let service = TemaService::from_ref(&state);
    service.eliminar_tema(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
