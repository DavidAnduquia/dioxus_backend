use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    middleware::Next,
    response::Response,
    body::Body,
};
use jsonwebtoken::{decode, Validation};

use crate::{
    models::{AppState, Claims},
    utils::errors::AppError,
};

#[derive(Debug, Clone)] 
pub struct AuthUser {
    pub user_id: i32,  // Cambiado de Uuid a i32
    pub email: String,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        tracing::debug!("AuthUser extractor called");
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .ok_or_else(|| {
                tracing::debug!("Missing Authorization header");
                AppError::Unauthorized("Missing Authorization header".into())
            })?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::Unauthorized("Invalid Authorization header format".into()))?;

        let claims = decode::<Claims>(
            token,
            state.jwt_decoding_key.as_ref(),
            &Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized("Invalid token".into()))?
        .claims;

        let user_id = claims.sub.parse::<i32>()
            .map_err(|_| AppError::Unauthorized("Invalid user ID in token".into()))?;

        let email = claims.email;
        if email.trim().is_empty() {
            return Err(AppError::Unauthorized("Email claim missing in token".into()));
        }

        let auth_user = AuthUser {
            user_id,
            email,
        };

        // Registrar el correo para evitar advertencias y facilitar el debugging
        tracing::trace!("Authenticated user email: {}", auth_user.email);

        Ok(auth_user)
    }
}

// Middleware eficiente que valida presencia básica del token JWT
// Sin buffers innecesarios - validación completa se hace en handlers con AuthUser
pub async fn jwt_auth_middleware(
    req: axum::http::Request<Body>,
    next: Next,
) -> Response {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok());

    // Validación básica: header presente y con formato Bearer
    let is_valid_format = match auth_header {
        Some(header) if header.starts_with("Bearer ") => {
            let token = &header[7..];
            !token.is_empty() && token.len() > 10 // Token básico presente
        }
        _ => false,
    };

    if !is_valid_format {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("content-type", "application/json")
            .body(Body::from(r#"{"error":"Unauthorized","message":"Valid JWT token required"}"#))
            .unwrap();
    }

    next.run(req).await
}
