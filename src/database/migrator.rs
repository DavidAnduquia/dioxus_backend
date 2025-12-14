use sea_orm::sea_query::TableCreateStatement;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, EntityTrait, Schema, Statement};

/// Trait genérico para cualquier entidad que quiera auto-migrarse
pub trait AutoMigrate: EntityTrait {
    /// Nombre legible de la entidad
    fn entity_name() -> &'static str;

    /// SQL para datos iniciales (opcional)
    fn seed_data() -> Option<Vec<String>> {
        None
    }
}

/// Motor genérico de migración para cualquier entidad
pub async fn migrate_entity<E>(db: &DatabaseConnection, entity: E) -> Result<(), sqlx::Error>
where
    E: AutoMigrate,
{
    let schema = Schema::new(DbBackend::Postgres);

    // Generar CREATE TABLE desde el modelo
    let create_table_stmt: TableCreateStatement = schema
        .create_table_from_entity(entity)
        .if_not_exists()
        .to_owned();

    // Ejecutar la creación
    let sql = db.get_database_backend().build(&create_table_stmt);
    db.execute(Statement::from_string(
        db.get_database_backend(),
        sql.to_string(),
    ))
    .await
    .map_err(|e| sqlx::Error::Configuration(e.to_string().into()))?;

    tracing::info!("✓ Table '{}' migrated from model", E::entity_name());

    // Ejecutar seeds si existen
    if let Some(seeds) = E::seed_data() {
        for seed_sql in seeds {
            db.execute(Statement::from_string(db.get_database_backend(), seed_sql))
                .await
                .map_err(|e| sqlx::Error::Configuration(e.to_string().into()))?;
        }
        tracing::info!("✓ '{}' seeded with default data", E::entity_name());
    }

    Ok(())
}

/// Macro para simplificar el registro de migraciones
#[macro_export]
macro_rules! register_migrations {
    ($db:expr, $($entity:ty),+ $(,)?) => {
        {
            $(
                $crate::database::auto_migrate::migrate_entity::<$entity>($db).await?;
            )+
        }
    };
}
