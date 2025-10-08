use crate::models::rol::Model;
use sqlx::PgPool;

pub struct RolService;

impl RolService {
    pub async fn find_by_id(
        pool: &PgPool,
        id: i32
    ) -> Result<Option<Model>, sqlx::Error> {
        sqlx::query_as::<_, Model>(
            "SELECT id, nombre FROM rustdema.roles WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }

 
    pub async fn obtener_roles(
        pool: &PgPool
    ) -> Result<Vec<Model>, sqlx::Error> {
        sqlx::query_as::<_, Model>(
            "SELECT id, nombre FROM rustdema.roles ORDER BY id"
        )
        .fetch_all(pool)
        .await
    }

    pub async fn create(
        pool: &PgPool,
        nombre: String
    ) -> Result<Model, sqlx::Error> {
        sqlx::query_as::<_, Model>(
            "INSERT INTO rustdema.roles (nombre) VALUES ($1) RETURNING id, nombre"
        )
        .bind(nombre)
        .fetch_one(pool)
        .await
    }

    pub async fn update(
        pool: &PgPool,
        id: i32,
        nombre: String
    ) -> Result<Model, sqlx::Error> {
        sqlx::query_as::<_, Model>(
            "UPDATE rustdema.roles SET nombre = $1 WHERE id = $2 RETURNING id, nombre"
        )
        .bind(nombre)
        .bind(id)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(
        pool: &PgPool,
        id: i32
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM rustdema.roles WHERE id = $1"
        )
        .bind(id)
        .execute(pool)
        .await?;
        
        Ok(result.rows_affected())
    }
}