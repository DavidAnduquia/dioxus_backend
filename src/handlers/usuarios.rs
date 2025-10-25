use axum::{
    extract::{Path, State},
    Json,
};
use serde::Serialize;
use std::sync::Arc;

use crate::{
    models::{usuario as usuario_models, AppState, Claims},
    services::usuario_service::UsuarioService,
    utils::errors::AppError,
};

// POST /api/usuarios/login
#[derive(serde::Deserialize)]
pub struct LoginPayload {
    pub identificador: String, // correo o documento_nit
    pub contrasena: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    #[serde(flatten)]
    pub usuario: usuario_models::Model,
}

pub async fn login_usuario(
    State(state): State<AppState>,
    State(service): State<Arc<UsuarioService>>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<LoginResponse>, AppError> {
    let usuario = service
        .login_usuario(
        &payload.identificador,
        &payload.contrasena,
    )
        .await?;

    // Generar token JWT
    let token = generate_token(&usuario, state.jwt_encoding_key.as_ref())?;

    Ok(Json(LoginResponse {
        token,
        usuario,
    }))
}

fn generate_token(
    usuario: &usuario_models::Model,
    encoding_key: &jsonwebtoken::EncodingKey,
) -> Result<String, AppError> {
    use chrono::Utc;
    use jsonwebtoken::{encode, Header};

    let now = Utc::now();
    let exp = (now + chrono::Duration::hours(24)).timestamp() as usize;
    let iat = now.timestamp() as usize;

    let claims = Claims {
        sub: usuario.id.to_string(),
        email: usuario.correo.clone(),
        exp,
        iat,
    };

    Ok(encode(&Header::default(), &claims, encoding_key)?)
}

// POST /api/usuarios/logout/:id
pub async fn logout_usuario(
    State(service): State<Arc<UsuarioService>>,
    Path(id): Path<i32>,
) -> Result<Json<usuario_models::Model>, AppError> {
    let usuario = service.logout_usuario(id).await?;
    Ok(Json(usuario))
}

// GET /api/usuarios
pub async fn listar_usuarios(
    State(service): State<Arc<UsuarioService>>,
) -> Result<Json<Vec<usuario_models::UsuarioConRol>>, AppError> {
    let usuarios = service.obtener_usuarios().await?;
    Ok(Json(usuarios))
}

// POST /api/usuarios
pub async fn crear_usuario(
    State(service): State<Arc<UsuarioService>>,
    Json(payload): Json<usuario_models::NewUsuario>,
) -> Result<Json<usuario_models::Model>, AppError> {
    let usuario = service.crear_usuario(payload).await?;
    Ok(Json(usuario))
}

// PUT /api/usuarios/:id
pub async fn actualizar_usuario(
    State(service): State<Arc<UsuarioService>>,
    Path(id): Path<i32>,
    Json(payload): Json<usuario_models::UpdateUsuario>,
) -> Result<Json<usuario_models::Model>, AppError> {
    let usuario = service.editar_usuario(id, payload).await?;
    Ok(Json(usuario))
}

// GET /api/usuarios/:id
pub async fn obtener_usuario_por_id(
    Path(_id): Path<i32>,
    State(_service): State<Arc<UsuarioService>>,
) -> Result<Json<Option<usuario_models::Model>>, String> {
    Ok(Json(None))
}

// GET /api/usuarios/:id
pub async fn get_usuario(
    Path(_id): Path<i32>,
    State(_service): State<Arc<UsuarioService>>,
) -> Result<Json<Option<usuario_models::Model>>, String> {
    Ok(Json(None))
}
