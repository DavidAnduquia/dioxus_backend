use axum::extract::FromRef;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait, QueryFilter, Set,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::database::DbExecutor;
use crate::models::{AppState, examen::{self, Entity as Examen, Model as ExamenModel}};
use crate::utils::errors::AppError;

#[derive(Debug, Clone)]
pub struct ExamenService {
    db: DbExecutor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevoExamen {
    pub curso_id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub fecha_inicio: DateTime<Utc>,
    pub fecha_fin: DateTime<Utc>,
    pub duracion_minutos: i32,
    pub intentos_permitidos: i32,
    pub mostrar_resultados: bool,
    pub estado: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActualizarExamen {
    pub nombre: Option<String>,
    pub descripcion: Option<String>,
    pub fecha_inicio: Option<DateTime<Utc>>,
    pub fecha_fin: Option<DateTime<Utc>>,
    pub duracion_minutos: Option<i32>,
    pub intentos_permitidos: Option<i32>,
    pub mostrar_resultados: Option<bool>,
    pub estado: Option<String>,
}

impl ExamenService {
    pub fn new(db: DbExecutor) -> Self {
        Self { db }
    }

    fn pool(&self) -> &PgPool {
        self.db.pool()
    }

    fn connection(&self) -> DatabaseConnection {
        self.db.connection()
    }

    pub async fn crear_examen(
        &self,
        nuevo_examen: NuevoExamen,
    ) -> Result<ExamenModel, AppError> {
        if nuevo_examen.nombre.trim().is_empty() {
            return Err(AppError::BadRequest("El nombre es obligatorio".to_string()));
        }
        if nuevo_examen.fecha_inicio >= nuevo_examen.fecha_fin {
            return Err(AppError::BadRequest("La fecha de inicio debe ser anterior a la fecha de fin".to_string()));
        }
        if nuevo_examen.duracion_minutos <= 0 {
            return Err(AppError::BadRequest("La duración debe ser mayor a 0".to_string()));
        }
        if nuevo_examen.intentos_permitidos <= 0 {
            return Err(AppError::BadRequest("Los intentos permitidos deben ser mayor a 0".to_string()));
        }

        let db = self.connection();
        let ahora = Utc::now();
        let examen = examen::ActiveModel {
            curso_id: Set(nuevo_examen.curso_id),
            nombre: Set(nuevo_examen.nombre),
            descripcion: Set(nuevo_examen.descripcion),
            fecha_inicio: Set(nuevo_examen.fecha_inicio),
            fecha_fin: Set(nuevo_examen.fecha_fin),
            duracion_minutos: Set(nuevo_examen.duracion_minutos),
            intentos_permitidos: Set(nuevo_examen.intentos_permitidos),
            mostrar_resultados: Set(nuevo_examen.mostrar_resultados),
            estado: Set(nuevo_examen.estado),
            created_at: Set(Some(ahora)),
            updated_at: Set(Some(ahora)),
            ..Default::default()
        };

        let examen_creado = examen.insert(&db).await?;
        Ok(examen_creado)
    }

    pub async fn obtener_examenes_por_curso(
        &self,
        curso_id: i32,
    ) -> Result<Vec<ExamenModel>, DbErr> {
        let db = self.connection();
        Examen::find()
            .filter(examen::Column::CursoId.eq(curso_id))
            .all(&db)
            .await
    }

    pub async fn obtener_examen_por_id(
        &self,
        id: i32,
    ) -> Result<Option<ExamenModel>, DbErr> {
        let db = self.connection();
        Examen::find_by_id(id).one(&db).await
    }

    pub async fn actualizar_examen(
        &self,
        id: i32,
        datos_actualizados: ActualizarExamen,
    ) -> Result<ExamenModel, AppError> {
        let db = self.connection();
        let examen = Examen::find_by_id(id)
            .one(&db)
            .await?
            .ok_or_else(|| AppError::NotFound("Examen no encontrado".to_string()))?;

        let mut examen: examen::ActiveModel = examen.into();
        let ahora = Utc::now();

        if let Some(nombre) = datos_actualizados.nombre {
            if nombre.trim().is_empty() {
                return Err(AppError::BadRequest("El nombre no puede estar vacío".to_string()));
            }
            examen.nombre = Set(nombre);
        }

        if let Some(descripcion) = datos_actualizados.descripcion {
            examen.descripcion = Set(Some(descripcion));
        }

        if let Some(fecha_inicio) = datos_actualizados.fecha_inicio {
            examen.fecha_inicio = Set(fecha_inicio);
        }

        if let Some(fecha_fin) = datos_actualizados.fecha_fin {
            examen.fecha_fin = Set(fecha_fin);
        }

        if let Some(duracion) = datos_actualizados.duracion_minutos {
            examen.duracion_minutos = Set(duracion);
        }

        if let Some(intentos) = datos_actualizados.intentos_permitidos {
            examen.intentos_permitidos = Set(intentos);
        }

        if let Some(mostrar) = datos_actualizados.mostrar_resultados {
            examen.mostrar_resultados = Set(mostrar);
        }

        if let Some(estado) = datos_actualizados.estado {
            examen.estado = Set(estado);
        }

        examen.updated_at = Set(Some(ahora));
        let examen_actualizado = examen.update(&db).await?;

        Ok(examen_actualizado)
    }

    pub async fn eliminar_examen(&self, id: i32) -> Result<(), AppError> {
        let db = self.connection();
        let examen = Examen::find_by_id(id)
            .one(&db)
            .await?
            .ok_or_else(|| AppError::NotFound("Examen no encontrado".to_string()))?;

        examen.delete(&db).await?;
        Ok(())
    }
}

impl FromRef<AppState> for ExamenService {
    fn from_ref(state: &AppState) -> Self {
        let executor = state.db.clone().expect("Database connection is not available");
        ExamenService::new(executor)
    }
}
