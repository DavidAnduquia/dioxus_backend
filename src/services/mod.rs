pub mod rol_service;
pub mod socket_service;
pub mod usuario_service;

use sqlx::PgPool;
use crate::{models, utils::errors::AppError};

#[derive(Clone)]
#[allow(dead_code)]
pub struct DatabaseService {
    pool: PgPool,
}

impl DatabaseService {
    #[allow(dead_code)]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    #[allow(dead_code)]
    pub async fn get_user(&self, id: uuid::Uuid) -> Result<models::User, AppError> {
        sqlx::query_as::<_, models::User>(
            "SELECT id, email, password_hash, name, created_at, updated_at FROM rustdema.users WHERE id = $1"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(Into::into)
    }
}
