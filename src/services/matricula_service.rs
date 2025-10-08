use axum::extract::FromRef;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set, SqlxPostgresConnector, Order};
use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    models::{
        curso::Entity as Curso,
        historial_curso_estudiante::{self, Entity as Historial, Model as HistorialModel},
        usuario::Entity as Usuario,
        AppState,
    },
    utils::errors::AppError,
};

pub trait MatriculaServiceTrait {
    async fn matricular_estudiante(
        &self,
        estudiante_id: i64,
        curso_id: i32,
    ) -> Result<HistorialModel, AppError>;

    async fn desmatricular_estudiante(
        &self,
        estudiante_id: i64,
        curso_id: i32,
    ) -> Result<HistorialModel, AppError>;

    async fn obtener_matriculas_estudiante(
        &self,
        estudiante_id: i64,
    ) -> Result<Vec<HistorialModel>, AppError>;

    async fn obtener_matriculas_curso(
        &self,
        curso_id: i32,
    ) -> Result<Vec<HistorialModel>, AppError>;
}

#[derive(Debug, Clone)]
pub struct MatriculaService {
    pool: Arc<Option<PgPool>>,
}

impl MatriculaService {
    pub fn new(pool: Arc<Option<PgPool>>) -> Self {
        Self { pool }
    }

    fn pool(&self) -> Result<&PgPool, AppError> {
        self.pool.as_ref().as_ref().ok_or_else(|| {
            AppError::ServiceUnavailable("Database connection is not available".to_string())
        })
    }

    fn connection(&self) -> Result<DatabaseConnection, AppError> {
        let pool = self.pool()?;
        Ok(SqlxPostgresConnector::from_sqlx_postgres_pool(pool.clone()))
    }
}

impl MatriculaServiceTrait for MatriculaService {
    async fn matricular_estudiante(
        &self,
        estudiante_id: i64,
        curso_id: i32,
    ) -> Result<HistorialModel, AppError> {
        let db = self.connection()?;

        Usuario::find_by_id(estudiante_id)
            .one(&db)
            .await?
            .ok_or_else(|| AppError::NotFound("Estudiante no encontrado".into()))?;

        Curso::find_by_id(curso_id)
            .one(&db)
            .await?
            .ok_or_else(|| AppError::NotFound("Curso no encontrado".into()))?;

        if Historial::find()
            .filter(historial_curso_estudiante::Column::EstudianteId.eq(estudiante_id))
            .filter(historial_curso_estudiante::Column::CursoId.eq(curso_id))
            .one(&db)
            .await?
            .is_some()
        {
            return Err(AppError::BadRequest("Estudiante ya matriculado".into()));
        }

        let ahora = Utc::now();
        let matricula = historial_curso_estudiante::ActiveModel {
            estudiante_id: Set(estudiante_id),
            curso_id: Set(curso_id),
            fecha_inscripcion: Set(ahora),
            estado: Set("activo".into()),
            created_at: Set(Some(ahora)),
            updated_at: Set(Some(ahora)),
            ..Default::default()
        };

        Ok(matricula.insert(&db).await?)
    }

    async fn desmatricular_estudiante(
        &self,
        estudiante_id: i64,
        curso_id: i32,
    ) -> Result<HistorialModel, AppError> {
        let db = self.connection()?;

        Usuario::find_by_id(estudiante_id)
            .one(&db)
            .await?
            .ok_or_else(|| AppError::NotFound("Estudiante no encontrado".into()))?;

        Curso::find_by_id(curso_id)
            .one(&db)
            .await?
            .ok_or_else(|| AppError::NotFound("Curso no encontrado".into()))?;

        let matricula = Historial::find()
            .filter(historial_curso_estudiante::Column::EstudianteId.eq(estudiante_id))
            .filter(historial_curso_estudiante::Column::CursoId.eq(curso_id))
            .one(&db)
            .await?
            .ok_or_else(|| AppError::BadRequest("Estudiante no matriculado".into()))?;

        let ahora = Utc::now();
        let mut matricula: historial_curso_estudiante::ActiveModel = matricula.into();
        matricula.estado = Set("inactivo".into());
        matricula.updated_at = Set(Some(ahora));

        let matricula_actualizada = matricula.update(&db).await?;
        Ok(matricula_actualizada)
    }

    async fn obtener_matriculas_estudiante(
        &self,
        estudiante_id: i64,
    ) -> Result<Vec<HistorialModel>, AppError> {
        let db = self.connection()?;
        let matriculas = Historial::find()
            .filter(historial_curso_estudiante::Column::EstudianteId.eq(estudiante_id))
            .order_by(historial_curso_estudiante::Column::FechaInscripcion, Order::Asc)
            .all(&db)
            .await?;

        Ok(matriculas)
    }

    async fn obtener_matriculas_curso(
        &self,
        curso_id: i32,
    ) -> Result<Vec<HistorialModel>, AppError> {
        let db = self.connection()?;
        let matriculas = Historial::find()
            .filter(historial_curso_estudiante::Column::CursoId.eq(curso_id))
            .order_by(historial_curso_estudiante::Column::FechaInscripcion, Order::Asc)
            .all(&db)
            .await?;

        Ok(matriculas)
    }
}

impl FromRef<AppState> for MatriculaService {
    fn from_ref(state: &AppState) -> Self {
        MatriculaService::new(Arc::clone(&state.db))
    }
}