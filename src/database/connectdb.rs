use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

/// Crea un pool de conexiones a PostgreSQL con reintentos
/// 
/// # Argumentos
/// * `database_url` - URL de conexión a la base de datos
/// 
/// # Errores
/// Retorna un error si no se puede conectar después de varios intentos
pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    const MAX_RETRIES: u32 = 3;
    const RETRY_DELAY_SECS: u64 = 2;
    
    tracing::info!("🔌 Intentando conectar a la base de datos...");
    
    for attempt in 1..=MAX_RETRIES {
        match try_create_pool(database_url).await {
            Ok(pool) => {
                tracing::info!("✅ Conexión a la base de datos establecida exitosamente");
                return Ok(pool);
            }
            Err(e) => {
                let error_msg = e.to_string();
                
                if attempt < MAX_RETRIES {
                    if error_msg.contains("Connection refused") || 
                       error_msg.contains("could not connect") ||
                       error_msg.contains("timed out") {
                        tracing::warn!(
                            "⚠️  Intento {}/{} falló: {}. Reintentando en {} segundos...",
                            attempt,
                            MAX_RETRIES,
                            error_msg,
                            RETRY_DELAY_SECS
                        );
                        tokio::time::sleep(Duration::from_secs(RETRY_DELAY_SECS)).await;
                    } else {
                        // Error no relacionado con conexión, fallar inmediatamente
                        tracing::error!("❌ Error de base de datos no recuperable: {}", error_msg);
                        return Err(e);
                    }
                } else {
                    tracing::error!(
                        "❌ No se pudo conectar a la base de datos después de {} intentos: {}",
                        MAX_RETRIES,
                        error_msg
                    );
                    return Err(e);
                }
            }
        }
    }
    
    unreachable!()
}

/// Intenta crear un pool de conexiones sin reintentos
async fn try_create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(2)  // Reducido a 2 para optimizar memoria en reposo
        .min_connections(0)  // 0 conexiones mínimas = máxima optimización en idle
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Some(Duration::from_secs(60)))   // 1 minuto idle, luego cierra
        .max_lifetime(Some(Duration::from_secs(300)))  // 5 minutos máximo
        .test_before_acquire(true)
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
