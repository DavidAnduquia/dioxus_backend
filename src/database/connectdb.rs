use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(2)  // Reducir a 2 conexiones máximo para menor memoria
        .min_connections(0)  // No mantener conexiones mínimas idle
        .acquire_timeout(Duration::from_secs(3))  // Reducir tiempo de adquisición
        .idle_timeout(Duration::from_secs(60))   // Cerrar conexiones idle más rápido (1 minuto)
        .max_lifetime(Duration::from_secs(600))  // Vida máxima más corta (10 minutos)
        .connect(database_url)
        .await
}

#[allow(dead_code)]
pub async fn init_schema(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Create schema if it doesn't exist
    sqlx::query("CREATE SCHEMA IF NOT EXISTS rustdema")
        .execute(pool)
        .await?;

    // Set search_path to use the schema
    sqlx::query("SET search_path TO rustdema, public")
        .execute(pool)
        .await?;

    tracing::info!("Schema rustdema initialized");
    Ok(())
}
