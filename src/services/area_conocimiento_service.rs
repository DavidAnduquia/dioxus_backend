use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder, Set};

use crate::{
    models::area_conocimiento::{self, Entity as AreaConocimiento, Model as AreaConocimientoModel},
    utils::errors::AppError,
};

#[derive(Debug, Clone)]
pub struct AreaConocimientoService {
    db: DatabaseConnection,
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
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn obtener_areas(&self) -> Result<Vec<AreaConocimientoModel>, DbErr> {
        AreaConocimiento::find()
            .order_by_desc(area_conocimiento::Column::CreatedAt)
            .all(&self.db)
            .await
    }

    pub async fn obtener_area_por_id(&self, id: i32) -> Result<Option<AreaConocimientoModel>, DbErr> {
        AreaConocimiento::find_by_id(id).one(&self.db).await
    }

    pub async fn obtener_areas_activas(&self) -> Result<Vec<AreaConocimientoModel>, DbErr> {
        AreaConocimiento::find()
            .filter(area_conocimiento::Column::Estado.eq(true))
            .order_by_asc(area_conocimiento::Column::Nombre)
            .all(&self.db)
            .await
    }

    pub async fn crear_area(
        &self,
        nueva_area: NuevaArea,
    ) -> Result<AreaConocimientoModel, AppError> {
        if nueva_area.nombre.trim().is_empty() {
            return Err(AppError::BadRequest("El nombre es obligatorio".to_string()));
        }

        let ahora = Utc::now();
        let area = area_conocimiento::ActiveModel {
            nombre: Set(nueva_area.nombre),
            descripcion: Set(nueva_area.descripcion),
            estado: Set(nueva_area.estado),
            created_at: Set(Some(ahora)),
            updated_at: Set(Some(ahora)),
            ..Default::default()
        };

        let area_creada = area.insert(&self.db).await?;
        Ok(area_creada)
    }

    pub async fn actualizar_area(
        &self,
        id: i32,
        datos_actualizados: ActualizarArea,
    ) -> Result<AreaConocimientoModel, AppError> {
        let area = AreaConocimiento::find_by_id(id)
            .one(&self.db)
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
        let area_actualizada = area.update(&self.db).await?;
        Ok(area_actualizada)
    }

    pub async fn cambiar_estado(&self, id: i32, estado: bool) -> Result<AreaConocimientoModel, AppError> {
        let area = AreaConocimiento::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Área de conocimiento no encontrada".to_string()))?;

        let mut area: area_conocimiento::ActiveModel = area.into();
        area.estado = Set(estado);
        area.updated_at = Set(Some(Utc::now()));

        let area_actualizada = area.update(&self.db).await?;
        Ok(area_actualizada)
    }

    pub async fn eliminar_area(&self, id: i32) -> Result<(), AppError> {
        let area = AreaConocimiento::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Área de conocimiento no encontrada".to_string()))?;

        area.delete(&self.db).await?;
        Ok(())
    }
}

#[async_trait]
impl crate::traits::service::CrudService<AreaConocimientoModel> for AreaConocimientoService {
    async fn get_all(&self) -> Result<Vec<AreaConocimientoModel>, AppError> {
        self.obtener_areas().await.map_err(Into::into)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<AreaConocimientoModel>, AppError> {
        self.obtener_area_por_id(id).await.map_err(Into::into)
    }

    async fn create(&self, data: AreaConocimientoModel) -> Result<AreaConocimientoModel, AppError> {
        self.crear_area(NuevaArea {
            nombre: data.nombre,
            descripcion: data.descripcion,
            estado: data.estado,
        }).await
    }

    async fn update(&self, id: i32, data: AreaConocimientoModel) -> Result<AreaConocimientoModel, AppError> {
        self.actualizar_area(id, ActualizarArea {
            nombre: Some(data.nombre),
            descripcion: data.descripcion,
            estado: Some(data.estado),
        }).await
    }

    async fn delete(&self, id: i32) -> Result<(), AppError> {
        self.eliminar_area(id).await
    }
}
