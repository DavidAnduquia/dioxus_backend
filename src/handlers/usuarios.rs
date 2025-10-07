use axum::{
    extract::{Path, State},
    Json,
};

use sea_orm::Database;

use crate::{
    models::{self, usuario as usuario_models, AppState},
    services::usuario_service::UsuarioService,
    utils::errors::AppError,
};

// POST /api/usuarios/login
#[derive(serde::Deserialize)]
pub struct LoginPayload {
    pub identificador: String, // correo o documento_nit
    pub contrasena: String,
}

pub async fn login_usuario(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<usuario_models::Model>, AppError> {
    let db_url = state.config.database_url.clone();
    let conn = Database::connect(db_url).await.map_err(|e| {
        AppError::ServiceUnavailable(format!("No se pudo conectar a la base de datos: {}", e))
    })?;

    let service = UsuarioService::new(conn);
    let usuario = service
        .login_usuario(&payload.identificador, &payload.contrasena)
        .await?;

    Ok(Json(usuario))
}

// POST /api/usuarios/logout/:id
pub async fn logout_usuario(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<usuario_models::Model>, AppError> {
    let db_url = state.config.database_url.clone();
    let conn = Database::connect(db_url).await.map_err(|e| {
        AppError::ServiceUnavailable(format!("No se pudo conectar a la base de datos: {}", e))
    })?;

    let service = UsuarioService::new(conn);
    let usuario = service.logout_usuario(id).await?;
    Ok(Json(usuario))
}

// GET /api/usuarios
pub async fn listar_usuarios(
    State(state): State<AppState>,
) -> Result<Json<Vec<usuario_models::UsuarioConRol>>, AppError> {
    let db_url = state.config.database_url.clone();
    let conn = Database::connect(db_url).await.map_err(|e| {
        AppError::ServiceUnavailable(format!("No se pudo conectar a la base de datos: {}", e))
    })?;

    let service = UsuarioService::new(conn);
    let usuarios = service.obtener_usuarios().await.map_err(AppError::from)?;
    Ok(Json(usuarios))
}

// POST /api/usuarios
pub async fn crear_usuario(
    State(state): State<AppState>,
    Json(payload): Json<usuario_models::NewUsuario>,
) -> Result<Json<usuario_models::Model>, AppError> {
    let db_url = state.config.database_url.clone();
    let conn = Database::connect(db_url).await.map_err(|e| {
        AppError::ServiceUnavailable(format!("No se pudo conectar a la base de datos: {}", e))
    })?;

    let service = UsuarioService::new(conn);
    let usuario = service.crear_usuario(payload).await?;
    Ok(Json(usuario))
}

// PUT /api/usuarios/:id
pub async fn actualizar_usuario(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<usuario_models::UpdateUsuario>,
) -> Result<Json<usuario_models::Model>, AppError> {
    let db_url = state.config.database_url.clone();
    let conn = Database::connect(db_url).await.map_err(|e| {
        AppError::ServiceUnavailable(format!("No se pudo conectar a la base de datos: {}", e))
    })?;

    let service = UsuarioService::new(conn);
    let usuario = service.editar_usuario(id, payload).await?;
    Ok(Json(usuario))
}

// GET /api/usuarios/:id
pub async fn obtener_usuario_por_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Option<usuario_models::Model>>, AppError> {
    let db_url = state.config.database_url.clone();
    let conn = Database::connect(db_url).await.map_err(|e| {
        AppError::ServiceUnavailable(format!("No se pudo conectar a la base de datos: {}", e))
    })?;

    let service = UsuarioService::new(conn);
    let usuario = service
        .obtener_usuario_por_id(id)
        .await
        .map_err(AppError::from)?;
    Ok(Json(usuario))
}
