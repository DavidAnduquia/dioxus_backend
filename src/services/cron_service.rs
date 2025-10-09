use std::collections::HashMap;
use std::sync::Mutex;

use axum::extract::FromRef;
use chrono::Utc;
use once_cell::sync::Lazy;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use sqlx::PgPool;
use tokio::task::JoinHandle;
use tokio::time::{self, Duration};

use crate::{
    database::DbExecutor,
    models::{
        evento_programado::{self, Entity as EventoProgramado},
        AppState,
    },
    utils::errors::AppError,
};

/// Intervalo en segundos para evaluar tareas programadas
const DEFAULT_INTERVAL_SECONDS: u64 = 60;

/// Registro global de jobs corriendo. Cada job se gestiona con un JoinHandle.
static JOBS: Lazy<Mutex<HashMap<i32, JoinHandle<()>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Clone)]
pub struct CronService {
    db: DbExecutor,
    interval_seconds: u64,
}

impl CronService {
    pub fn new(db: DbExecutor) -> Self {
        Self {
            db,
            interval_seconds: DEFAULT_INTERVAL_SECONDS,
        }
    }

    fn pool(&self) -> &PgPool {
        self.db.pool()
    }

    fn connection(&self) -> DatabaseConnection {
        self.db.connection()
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

        let lock_result = JOBS.lock();
        match lock_result {
            Ok(mut jobs) => {
                jobs.insert(id, handle);
            }
            Err(err) => {
                tracing::error!(job_id = id, error = %err, "Failed to acquire JOBS lock (insert)");
                handle.abort();
            }
        }
    }

    /// Detiene y elimina un job en ejecución.
    pub fn detener_job(&self, id: i32) {
        match JOBS.lock() {
            Ok(mut jobs) => {
                if let Some(handle) = jobs.remove(&id) {
                    handle.abort();
                }
            }
            Err(err) => {
                tracing::error!(job_id = id, error = %err, "Failed to acquire JOBS lock (remove)");
            }
        }
    }

    /// Lista los jobs activos y sus identificadores.
    pub fn listar_jobs(&self) -> Vec<i32> {
        match JOBS.lock() {
            Ok(jobs) => {
                let mut ids = Vec::with_capacity(jobs.len());
                ids.extend(jobs.keys().copied());
                ids
            }
            Err(err) => {
                tracing::error!(error = %err, "Failed to acquire JOBS lock (list)");
                Vec::new()
            }
        }
    }

    /// Busca en la base de datos los eventos `pendiente` y programa su ejecución.
    pub async fn iniciar_jobs_desde_eventos(&self) -> Result<(), AppError> {
        let db = self.connection();
        let eventos = EventoProgramado::find()
            .filter(evento_programado::Column::Estado.eq("pendiente"))
            .all(&db)
            .await?;

        for evento in eventos {
            let db_executor = self.db.clone();
            let evento_id = evento.id;

            self.iniciar_job(evento_id, move || {
                let db_executor = db_executor.clone();
                tokio::spawn(async move {
                    let connection = db_executor.connection();
                    manejar_evento(connection, evento_id).await;
                });
            });
        }

        Ok(())
    }
}

impl FromRef<AppState> for CronService {
    fn from_ref(state: &AppState) -> Self {
        let executor = state.db.clone().expect("Database connection is not available");
        CronService::new(executor)
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
