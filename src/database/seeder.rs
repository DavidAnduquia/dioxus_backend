use sea_orm::{Database, DatabaseConnection};
use sqlx::PgPool;

/// Ejecuta todas las migraciones basadas en los modelos
pub async fn run_migrations(
    pool: &PgPool,
    database_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Running migrations from SeaORM models...");

    // Convertir SQLx pool a SeaORM connection
    let db: DatabaseConnection = Database::connect(database_url)
        .await
        .map_err(|e| sqlx::Error::Configuration(e.to_string().into()))?;

    // Migrar tabla users (legacy)
    migrate_users(pool).await?;

    // Migrar tabla usuarios usando SeaORM
    migrate_usuarios_with_seaorm(&db).await?;

    // Migrar tabla roles usando SeaORM
    migrate_roles_with_seaorm(&db).await?;

    tracing::info!("✅ All migrations completed successfully");
    Ok(())
}

/// Migración para users
async fn migrate_users(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
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

/// Migración para usuarios usando el sistema genérico
async fn migrate_usuarios_with_seaorm(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    use crate::database::migrator::migrate_entity;
    use crate::models::usuario::Entity as Usuario;

    // Pasar la entidad como parámetro
    migrate_entity(db, Usuario).await?;

    Ok(())
}

/// Migración para roles usando el sistema genérico
async fn migrate_roles_with_seaorm(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    use crate::database::migrator::migrate_entity;
    use crate::models::rol::Entity as Rol;

    // Pasar la entidad como parámetro
    migrate_entity(db, Rol).await?;

    Ok(())
}
