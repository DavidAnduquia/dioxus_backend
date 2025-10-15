use axum::{
    extract::{FromRef, Path, State},
    Json,
};

use crate::{
    models::{usuario as usuario_models, AppState},
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
    let service = UsuarioService::from_ref(&state);
    let usuario = service
        .login_usuario(
        &payload.identificador,
        &payload.contrasena,
    )
        .await?;

    Ok(Json(usuario))
}

// POST /api/usuarios/logout/:id
pub async fn logout_usuario(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<usuario_models::Model>, AppError> {
    let service = UsuarioService::from_ref(&state);
    let usuario = service.logout_usuario(id).await?;
    Ok(Json(usuario))
}

// GET /api/usuarios
pub async fn listar_usuarios(
    State(state): State<AppState>,
) -> Result<Json<Vec<usuario_models::UsuarioConRol>>, AppError> {
    let service = UsuarioService::from_ref(&state);
    let usuarios = service.obtener_usuarios().await?;
    Ok(Json(usuarios))
}

// POST /api/usuarios
pub async fn crear_usuario(
    State(state): State<AppState>,
    Json(payload): Json<usuario_models::NewUsuario>,
) -> Result<Json<usuario_models::Model>, AppError> {
    let service = UsuarioService::from_ref(&state);
    let usuario = service.crear_usuario(payload).await?;
    Ok(Json(usuario))
}

// PUT /api/usuarios/:id
pub async fn actualizar_usuario(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<usuario_models::UpdateUsuario>,
) -> Result<Json<usuario_models::Model>, AppError> {
    let service = UsuarioService::from_ref(&state);
    let usuario = service.editar_usuario(id, payload).await?;
    Ok(Json(usuario))
}

// GET /api/usuarios/:id
pub async fn obtener_usuario_por_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Option<usuario_models::Model>>, AppError> {
    let service = UsuarioService::from_ref(&state);
    let usuario = service.obtener_usuario_por_id(id).await?;
    Ok(Json(usuario))
}
