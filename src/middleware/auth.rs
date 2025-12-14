use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};
use jsonwebtoken::{decode, Validation};

use crate::{
    models::{AppState, Claims},
    utils::errors::AppError,
};

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: i32, // Cambiado de Uuid a i32
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

        let user_id = claims
            .sub
            .parse::<i32>()
            .map_err(|_| AppError::Unauthorized("Invalid user ID in token".into()))?;

        let email = claims.email;
        if email.trim().is_empty() {
            return Err(AppError::Unauthorized(
                "Email claim missing in token".into(),
            ));
        }

        let auth_user = AuthUser { user_id, email };

        // Registrar el usuario autenticado para trazas y futuras autorizaciones
        tracing::trace!(
            "Authenticated user {} ({})",
            auth_user.user_id,
            auth_user.email
        );

        Ok(auth_user)
    }
}
