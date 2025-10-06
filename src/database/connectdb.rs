use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(3)  // Máximo 3 conexiones (balance entre memoria y concurrencia)
        .min_connections(1)  // Mantener 1 conexión mínima
        .acquire_timeout(Duration::from_secs(5))  // Tiempo razonable para adquirir conexión
        .idle_timeout(Duration::from_secs(300))   // Cerrar conexiones idle después de 5 minutos
        .max_lifetime(Duration::from_secs(1800))  // Vida máxima de 30 minutos (balance)
        .connect(database_url)
        .await
}

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
