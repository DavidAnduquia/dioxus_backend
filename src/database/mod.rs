use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)  // Reducido de 20 a 5 para ahorrar memoria
        .acquire_timeout(Duration::from_secs(5))  // Aumentado timeout para conexiones más estables
        .idle_timeout(Duration::from_secs(300))   // Cerrar conexiones idle después de 5 minutos
        .max_lifetime(Duration::from_secs(3600))  // Máxima vida de conexión 1 hora
        .connect(database_url)
        .await
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Create schema if it doesn't exist
    sqlx::query("CREATE SCHEMA IF NOT EXISTS rustdema")
        .execute(pool)
        .await?;

    // Set search_path to use the schema
    sqlx::query("SET search_path TO rustdema, public")
        .execute(pool)
        .await?;

    // Create users table in rustdema schema
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS rustdema.users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            email VARCHAR(255) UNIQUE NOT NULL,
            password_hash VARCHAR(255) NOT NULL,
            name VARCHAR(255) NOT NULL,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create posts table (example entity) in rustdema schema
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS rustdema.posts (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            title VARCHAR(255) NOT NULL,
            content TEXT NOT NULL,
            author_id UUID NOT NULL REFERENCES rustdema.users(id) ON DELETE CASCADE,
            published BOOLEAN DEFAULT FALSE,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await?;

    tracing::info!("Database migrations completed successfully in rustdema schema");
    Ok(())
}
