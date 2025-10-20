use axum::extract::FromRef;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, QueryOrder, Set,
};
use serde::{Deserialize, Serialize};

use crate::{
    database::DbExecutor,
    models::{
        area_conocimiento::{self, Entity as AreaConocimiento, Model as AreaConocimientoModel},
        AppState,
    },
    utils::errors::AppError,
};

#[derive(Debug, Clone)]
pub struct AreaConocimientoService {
    db: DbExecutor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevaArea {
    pub nombre: String,
    pub descripcion: Option<String>,
    pub estado: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActualizarArea {
    pub nombre: Option<String>,
    pub descripcion: Option<String>,
    pub estado: Option<bool>,
}

impl AreaConocimientoService {
    pub fn new(db: DbExecutor) -> Self {
        Self { db }
    }


    fn connection(&self) -> DatabaseConnection {
        self.db.connection()
    }

    pub async fn obtener_areas(&self) -> Result<Vec<AreaConocimientoModel>, AppError> {
        let db = self.connection();
        let areas = AreaConocimiento::find()
            .order_by_desc(area_conocimiento::Column::CreatedAt)
            .all(&db)
            .await?;
        Ok(areas)
    }

    pub async fn obtener_area_por_id(
        &self,
        id: i32,
    ) -> Result<Option<AreaConocimientoModel>, AppError> {
        let db = self.connection();
        let area = AreaConocimiento::find_by_id(id).one(&db).await?;
        Ok(area)
    }

    pub async fn obtener_areas_activas(&self) -> Result<Vec<AreaConocimientoModel>, AppError> {
        let db = self.connection();
        let areas = AreaConocimiento::find()
            .filter(area_conocimiento::Column::Estado.eq(true))
            .order_by_asc(area_conocimiento::Column::Nombre)
            .all(&db)
            .await?;
        Ok(areas)
    }

    pub async fn crear_area(
        &self,
        nueva_area: NuevaArea,
    ) -> Result<AreaConocimientoModel, AppError> {
        if nueva_area.nombre.trim().is_empty() {
            return Err(AppError::BadRequest("El nombre es obligatorio".to_string()));
        }

        let db = self.connection();
        let ahora = Utc::now();
        let area = area_conocimiento::ActiveModel {
            id: Set(0), // Auto-increment field
            nombre: Set(nueva_area.nombre),
            descripcion: Set(nueva_area.descripcion),
            estado: Set(nueva_area.estado),
            created_at: Set(Some(ahora)),
            updated_at: Set(Some(ahora)),
        };

        let area_creada = area.insert(&db).await?;
        Ok(area_creada)
    }

    pub async fn actualizar_area(
        &self,
        id: i32,
        datos_actualizados: ActualizarArea,
    ) -> Result<AreaConocimientoModel, AppError> {
        let db = self.connection();
        let area = AreaConocimiento::find_by_id(id)
            .one(&db)
            .await?
            .ok_or_else(|| AppError::NotFound("Área de conocimiento no encontrada".to_string()))?;

        let mut area: area_conocimiento::ActiveModel = area.into();

        if let Some(nombre) = datos_actualizados.nombre {
            if nombre.trim().is_empty() {
                return Err(AppError::BadRequest("El nombre no puede estar vacío".to_string()));
            }
            area.nombre = Set(nombre);
        }
        if let Some(descripcion) = datos_actualizados.descripcion {
            area.descripcion = Set(Some(descripcion));
        }
        if let Some(estado) = datos_actualizados.estado {
            area.estado = Set(estado);
        }

        area.updated_at = Set(Some(Utc::now()));
        let area_actualizada = area.update(&db).await?;
        Ok(area_actualizada)
    }

    pub async fn cambiar_estado(&self, id: i32, estado: bool) -> Result<AreaConocimientoModel, AppError> {
        let db = self.connection();
        let area = AreaConocimiento::find_by_id(id)
            .one(&db)
            .await?
            .ok_or_else(|| AppError::NotFound("Área de conocimiento no encontrada".to_string()))?;

        let mut area: area_conocimiento::ActiveModel = area.into();
        area.estado = Set(estado);
        area.updated_at = Set(Some(Utc::now()));

        let area_actualizada = area.update(&db).await?;
        Ok(area_actualizada)
    }

    pub async fn eliminar_area(&self, id: i32) -> Result<(), AppError> {
        let db = self.connection();
        let area = AreaConocimiento::find_by_id(id)
            .one(&db)
            .await?
            .ok_or_else(|| AppError::NotFound("Área de conocimiento no encontrada".to_string()))?;
        area.delete(&db).await?;
        Ok(())
    }
}

impl FromRef<AppState> for AreaConocimientoService {
    fn from_ref(state: &AppState) -> Self {
        let executor = state.db.clone().expect("Database connection is not available");
        AreaConocimientoService::new(executor)
    }
}
