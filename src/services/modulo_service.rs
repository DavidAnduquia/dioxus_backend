use axum::extract::FromRef;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait, QueryFilter, QueryOrder, Set, SqlxPostgresConnector, Order};
use crate::{
    models::modulo::{self, Entity as Modulo, Model as ModuloModel},
    models::AppState,
    utils::errors::AppError,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ModuloService {
    pool: Arc<Option<PgPool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevoModulo {
    pub curso_id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub orden: i32,
    pub visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActualizarModulo {
    pub nombre: Option<String>,
    pub descripcion: Option<String>,
    pub orden: Option<i32>,
    pub visible: Option<bool>,
}

impl ModuloService {
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

    pub async fn crear_modulo(
        &self,
        nuevo_modulo: NuevoModulo,
    ) -> Result<ModuloModel, AppError> {
        if nuevo_modulo.nombre.trim().is_empty() {
            return Err(AppError::BadRequest("El nombre es obligatorio".to_string()));
        }

        let db = self.connection()?;
        let ahora = Utc::now();
        let modulo = modulo::ActiveModel {
            id: Set(0), // Auto-increment field
            curso_id: Set(nuevo_modulo.curso_id),
            nombre: Set(nuevo_modulo.nombre),
            descripcion: Set(nuevo_modulo.descripcion),
            orden: Set(nuevo_modulo.orden),
            visible: Set(nuevo_modulo.visible),
            created_at: Set(Some(ahora)),
            updated_at: Set(Some(ahora)),
        };

        let modulo_creado = modulo.insert(&db).await?;
        Ok(modulo_creado)
    }

    pub async fn obtener_modulos_por_curso(
        &self,
        curso_id: i32,
    ) -> Result<Vec<ModuloModel>, DbErr> {
        let db = self.connection().map_err(|e| DbErr::Custom(e.to_string()))?;
        Modulo::find()
            .filter(modulo::Column::CursoId.eq(curso_id))
            .order_by(modulo::Column::Orden, Order::Asc)
            .all(&db)
            .await
    }

    pub async fn obtener_modulo_por_id(
        &self,
        id: i32,
    ) -> Result<Option<ModuloModel>, DbErr> {
        let db = self.connection().map_err(|e| DbErr::Custom(e.to_string()))?;
        Modulo::find_by_id(id).one(&db).await
    }

    pub async fn actualizar_modulo(
        &self,
        id: i32,
        datos_actualizados: ActualizarModulo,
    ) -> Result<ModuloModel, AppError> {
        let db = self.connection()?;
        let modulo = Modulo::find_by_id(id)
            .one(&db)
            .await?
            .ok_or_else(|| AppError::NotFound("Módulo no encontrado".to_string()))?;

        let mut modulo: modulo::ActiveModel = modulo.into();
        let ahora = Utc::now();

        if let Some(nombre) = datos_actualizados.nombre {
            if nombre.trim().is_empty() {
                return Err(AppError::BadRequest("El nombre no puede estar vacío".to_string()));
            }
            modulo.nombre = Set(nombre);
        }

        if let Some(descripcion) = datos_actualizados.descripcion {
            modulo.descripcion = Set(Some(descripcion));
        }

        if let Some(orden) = datos_actualizados.orden {
            modulo.orden = Set(orden);
        }

        if let Some(visible) = datos_actualizados.visible {
            modulo.visible = Set(visible);
        }

        modulo.updated_at = Set(Some(ahora));
        let modulo_actualizado = modulo.update(&db).await?;

        Ok(modulo_actualizado)
    }

    pub async fn eliminar_modulo(&self, id: i32) -> Result<(), AppError> {
        let db = self.connection()?;
        let modulo = Modulo::find_by_id(id)
            .one(&db)
            .await?
            .ok_or_else(|| AppError::NotFound("Módulo no encontrado".to_string()))?;

        modulo.delete(&db).await?;
        Ok(())
    }
}

impl FromRef<AppState> for ModuloService {
    fn from_ref(state: &AppState) -> Self {
        ModuloService::new(Arc::clone(&state.db))
    }
}
