use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use tokio::task::JoinHandle;
use tokio::time::{self, Duration};

use crate::{
    models::evento_programado::{self, Entity as EventoProgramado},
    utils::errors::AppError,
};

/// Intervalo en segundos para evaluar tareas programadas
const DEFAULT_INTERVAL_SECONDS: u64 = 60;

/// Registro global de jobs corriendo. Cada job se gestiona con un JoinHandle.
static JOBS: Lazy<Arc<Mutex<HashMap<i32, JoinHandle<()>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

#[derive(Clone)]
pub struct CronService {
    db: DatabaseConnection,
    interval_seconds: u64,
}

impl CronService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            interval_seconds: DEFAULT_INTERVAL_SECONDS,
        }
    }

    /// Permite configurar cada cuánto se re-evaluarán los cron jobs.
    pub fn with_interval(mut self, seconds: u64) -> Self {
        self.interval_seconds = seconds;
        self
    }

    /// Inicia (o reinicia) un job que se ejecutará cada `interval_seconds`.
    pub fn iniciar_job<F>(&self, id: i32, mut tarea: F)
    where
        F: FnMut() + Send + 'static,
    {
        // Detener caso exista
        self.detener_job(id);

        let interval = self.interval_seconds;
        let handle = tokio::spawn(async move {
            let mut ticker = time::interval(Duration::from_secs(interval));
            loop {
                ticker.tick().await;
                tarea();
            }
        });

        JOBS.lock().unwrap().insert(id, handle);
    }

    /// Detiene y elimina un job en ejecución.
    pub fn detener_job(&self, id: i32) {
        if let Some(handle) = JOBS.lock().unwrap().remove(&id) {
            handle.abort();
        }
    }

    /// Lista los jobs activos y sus identificadores.
    pub fn listar_jobs(&self) -> Vec<i32> {
        JOBS.lock().unwrap().keys().cloned().collect()
    }

    /// Busca en la base de datos los eventos `pendiente` y programa su ejecución.
    pub async fn iniciar_jobs_desde_eventos(&self) -> Result<(), AppError> {
        let eventos = EventoProgramado::find()
            .filter(evento_programado::Column::Estado.eq("pendiente"))
            .all(&self.db)
            .await?;

        for evento in eventos {
            let db_clone = self.db.clone();
            let evento_id = evento.id;

            self.iniciar_job(evento_id, move || {
                tokio::spawn(manejar_evento(db_clone.clone(), evento_id));
            });
        }

        Ok(())
    }
}

async fn manejar_evento(db: DatabaseConnection, evento_id: i32) {
    if let Ok(Some(mut evento)) = EventoProgramado::find_by_id(evento_id).one(&db).await {
        if evento.estado != "pendiente" {
            return;
        }

        // Ejecutar lógica propia del evento
        tracing::info!("Ejecutando evento programado {}", evento.tipo_evento);

        evento.estado = "ejecutado".to_string();
        evento.updated_at = Some(Utc::now());

        let mut active: evento_programado::ActiveModel = evento.into();
        if let Err(err) = active.update(&db).await {
            tracing::error!("No se pudo actualizar evento {}: {}", evento_id, err);
        }
    }
}
