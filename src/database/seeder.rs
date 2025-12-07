use sqlx::PgPool;
use sea_orm::{Database, DatabaseConnection};

/// Ejecuta todas las migraciones basadas en los modelos
#[allow(dead_code)]
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    tracing::info!("Running migrations from SeaORM models...");

    // Convertir SQLx pool a SeaORM connection
    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let db: DatabaseConnection = Database::connect(&db_url)
        .await
        .map_err(|e| sqlx::Error::Configuration(e.to_string().into()))?;

    // Migrar tabla users (legacy)
    migrate_users(pool).await?;
    
    // Migrar tabla posts (legacy)
    //migrate_posts(pool).await?;
    
    // Migrar tabla roles usando SeaORM
    migrate_roles_with_seaorm(&db).await?;

    tracing::info!("✅ All migrations completed successfully");
    Ok(())
}

/// Migración para users
async fn migrate_users(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS rustdema2.users (
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
    
    tracing::info!("✓ Table 'users' migrated");
    Ok(())
}

/// Migración para posts
async fn migrate_posts(pool: &PgPool) -> Result<(), sqlx::Error> {
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
    
    tracing::info!("✓ Table 'posts' migrated");
    Ok(())
}

/// Migración para roles usando el sistema genérico
async fn migrate_roles_with_seaorm(db: &DatabaseConnection) -> Result<(), sqlx::Error> {
    use crate::models::rol::Entity as Rol;
    use crate::database::migrator::migrate_entity;
    
    // Pasar la entidad como parámetro
    migrate_entity(db, Rol).await?;
    
    Ok(())
}
