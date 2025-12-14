use axum::{
    extract::{FromRef, Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    middleware::auth::AuthUser,
    models::AppState,
    services::notificacion_service::{NotificacionService, NuevaNotificacion},
    utils::errors::AppError,
};

#[derive(Debug, Deserialize)]
pub struct NotificacionesQuery {
    pub leida: Option<bool>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct NotificacionesResponse {
    pub notificaciones: Vec<Value>,
    pub total: u64,
}

/// Función helper para serializar una notificación a JSON
fn serialize_notificacion(n: &crate::models::notificacion::Model) -> Value {
    json!({
        "id": n.id,
        "usuario_id": n.usuario_id,
        "titulo": n.titulo,
        "mensaje": n.mensaje,
        "tipo": n.tipo,
        "leida": n.leida,
        "enlace": n.enlace,
        "datos_adicionales": n.datos_adicionales,
        "created_at": n.created_at,
        "updated_at": n.updated_at,
    })
}

/// Obtener notificaciones de un usuario
pub async fn obtener_notificaciones_usuario(
    _auth_user: AuthUser, // Validar JWT automáticamente
    State(state): State<AppState>,
    Path(usuario_id): Path<i64>,
    Query(params): Query<NotificacionesQuery>,
) -> Result<Json<NotificacionesResponse>, AppError> {
    let service = NotificacionService::from_ref(&state);

    // Convertir usuario_id de i64 (ruta) a i32 (modelo/BD)
    let usuario_id_i32 = i32::try_from(usuario_id)
        .map_err(|_| AppError::BadRequest("usuario_id fuera de rango para i32".into()))?;

    let (notificaciones, total) = service
        .obtener_por_usuario(usuario_id_i32, params.leida, params.limit, params.offset)
        .await?;

    let notificaciones_json: Vec<Value> =
        notificaciones.iter().map(serialize_notificacion).collect();

    Ok(Json(NotificacionesResponse {
        notificaciones: notificaciones_json,
        total,
    }))
}

/// Marcar notificación como leída
pub async fn marcar_notificacion_leida(
    _auth_user: AuthUser, // Validar JWT automáticamente
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, AppError> {
    let service = NotificacionService::from_ref(&state);

    let notificacion = service.marcar_como_leida(id).await?;

    Ok(Json(serialize_notificacion(&notificacion)))
}

/// Crear nueva notificación
pub async fn crear_notificacion(
    _auth_user: AuthUser, // Validar JWT automáticamente
    State(state): State<AppState>,
    Json(payload): Json<NuevaNotificacion>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let service = NotificacionService::from_ref(&state);

    let notificacion = service.crear_notificacion(payload).await?;

    Ok((
        StatusCode::CREATED,
        Json(serialize_notificacion(&notificacion)),
    ))
}

/// Marcar todas las notificaciones de un usuario como leídas
pub async fn marcar_todas_leidas(
    _auth_user: AuthUser, // Validar JWT automáticamente
    State(state): State<AppState>,
    Path(usuario_id): Path<i64>,
) -> Result<Json<Value>, AppError> {
    let service = NotificacionService::from_ref(&state);

    let usuario_id_i32 = i32::try_from(usuario_id)
        .map_err(|_| AppError::BadRequest("usuario_id fuera de rango para i32".into()))?;

    service.marcar_todas_como_leidas(usuario_id_i32).await?;

    Ok(Json(json!({
        "message": "Todas las notificaciones marcadas como leídas",
        "usuario_id": usuario_id
    })))
}
