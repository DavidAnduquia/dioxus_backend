pub mod rol_service;

use sqlx::PgPool;
use crate::{models, utils::errors::AppError};

#[derive(Clone)]
pub struct DatabaseService {
    pool: PgPool,
}

impl DatabaseService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

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
