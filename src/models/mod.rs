use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;
use jsonwebtoken::{EncodingKey, DecodingKey};
 
use crate::config::Config;

pub mod rol;
pub mod actividad;
pub mod area_conocimiento;
pub mod calificacion;
pub mod contenido_plantilla;
pub mod contenido_transversal;
pub mod curso;
pub mod evaluacion_calificacion;
pub mod evaluacion_sesion;
pub mod evento_programado;
pub mod examen;
pub mod historial_curso_actividad;
pub mod historial_curso_estudiante;
pub mod modulo;
pub mod modulo_archivo;
pub mod notificacion;
pub mod plantilla_curso;
pub mod portafolio;
pub mod portafolio_contenido;
pub mod pregunta_examen;
pub mod profesor_curso;
pub mod usuario;

// Re-export models for easier access

// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Option<PgPool>>, 
    pub config: Config,
    pub jwt_encoding_key: EncodingKey,
    pub jwt_decoding_key: DecodingKey,
}

impl AppState {
    /// Verifica si la conexión a la base de datos está disponible
    pub fn is_db_available(&self) -> bool {
        self.db.is_some()
    }
    
    /// Obtiene el pool de base de datos o retorna un error
    pub fn get_db(&self) -> Result<&PgPool, crate::utils::errors::AppError> {
        self.db.as_ref().as_ref().ok_or_else(|| {
            crate::utils::errors::AppError::ServiceUnavailable(
                format!(
                    "Database connection is not available (environment: {:?})",
                    self.config.environment
                )
            )
        })
    }
}

// User models
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
    #[validate(length(min = 1))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    pub email: String,
    pub name: String,
    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            name: user.name,
            created_at: user.created_at,
        }
    }
}
  
// JWT Claims
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Claims {
    pub sub: String, // user id
    pub email: String,
    pub exp: usize,
    pub iat: usize,
}

// API Response wrapper
#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
        }
    }

    pub fn error(message: String) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            message: Some(message),
        }
    }
}
