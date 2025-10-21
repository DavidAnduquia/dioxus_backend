use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};
use jsonwebtoken::{decode, Validation};
use uuid::Uuid;

use crate::{
    models::{AppState, Claims},
    utils::errors::AppError,
};

#[derive(Debug, Clone)] 
pub struct AuthUser {
    pub user_id: Uuid,
    pub email: String,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".to_string()))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::Unauthorized("Invalid Authorization header format".to_string()))?;

        let claims = decode::<Claims>(
            token,
            state.jwt_decoding_key.as_ref(),
            &Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))?
        .claims;

        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))?;

        let email = claims.email;
        if email.trim().is_empty() {
            return Err(AppError::Unauthorized("Email claim missing in token".to_string()));
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

// Simplified auth layer - just returns identity middleware for now
pub fn auth_layer() -> tower::layer::util::Identity {
    tower::layer::util::Identity::new()
}
