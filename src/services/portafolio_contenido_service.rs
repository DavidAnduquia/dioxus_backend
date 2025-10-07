use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder,
    Set,
};

use crate::{
    models::portafolio_contenido::{self, Entity as PortafolioContenido, Model as PortafolioContenidoModel},
    utils::errors::AppError,
};

#[derive(Debug, Clone)]
pub struct PortafolioContenidoService {
    db: DatabaseConnection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevoPortafolioContenido {
    pub portafolio_id: i32,
    pub tipo_contenido: String,
    pub titulo: String,
    pub descripcion: Option<String>,
    pub contenido: String,
    pub orden: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActualizarPortafolioContenido {
    pub tipo_contenido: Option<String>,
    pub titulo: Option<String>,
    pub descripcion: Option<String>,
    pub contenido: Option<String>,
    pub orden: Option<i32>,
}

impl PortafolioContenidoService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn crear_contenido(
        &self,
        nuevo_contenido: NuevoPortafolioContenido,
    ) -> Result<PortafolioContenidoModel, AppError> {
        if nuevo_contenido.titulo.trim().is_empty() {
            return Err(AppError::BadRequest("El título es obligatorio".to_string()));
        }

        if nuevo_contenido.contenido.trim().is_empty() {
            return Err(AppError::BadRequest("El contenido es obligatorio".to_string()));
        }

        let ahora = Utc::now();
        let contenido = portafolio_contenido::ActiveModel {
            portafolio_id: Set(nuevo_contenido.portafolio_id),
            tipo_contenido: Set(nuevo_contenido.tipo_contenido),
            titulo: Set(nuevo_contenido.titulo),
            descripcion: Set(nuevo_contenido.descripcion),
            contenido: Set(nuevo_contenido.contenido),
            orden: Set(nuevo_contenido.orden),
            created_at: Set(Some(ahora)),
            updated_at: Set(Some(ahora)),
            ..Default::default()
        };

        let contenido_creado = contenido.insert(&self.db).await?;
        Ok(contenido_creado)
    }

    pub async fn obtener_contenidos(
        &self,
        portafolio_id: Option<i32>,
    ) -> Result<Vec<PortafolioContenidoModel>, DbErr> {
        let mut query = PortafolioContenido::find();

        if let Some(id) = portafolio_id {
            query = query.filter(portafolio_contenido::Column::PortafolioId.eq(id));
        }

        query
            .order_by_asc(portafolio_contenido::Column::Orden)
            .all(&self.db)
            .await
    }

    pub async fn obtener_contenido_por_id(
        &self,
        id: i32,
    ) -> Result<Option<PortafolioContenidoModel>, DbErr> {
        PortafolioContenido::find_by_id(id).one(&self.db).await
    }

    pub async fn actualizar_contenido(
        &self,
        id: i32,
        datos_actualizados: ActualizarPortafolioContenido,
    ) -> Result<PortafolioContenidoModel, AppError> {
        let contenido = PortafolioContenido::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Contenido no encontrado".to_string()))?;

        let mut contenido: portafolio_contenido::ActiveModel = contenido.into();
        let ahora = Utc::now();

        if let Some(tipo) = datos_actualizados.tipo_contenido {
            contenido.tipo_contenido = Set(tipo);
        }

        if let Some(titulo) = datos_actualizados.titulo {
            if titulo.trim().is_empty() {
                return Err(AppError::BadRequest("El título no puede estar vacío".to_string()));
            }
            contenido.titulo = Set(titulo);
        }

        if let Some(descripcion) = datos_actualizados.descripcion {
            contenido.descripcion = Set(Some(descripcion));
        }

        if let Some(contenido_valor) = datos_actualizados.contenido {
            if contenido_valor.trim().is_empty() {
                return Err(AppError::BadRequest("El contenido no puede estar vacío".to_string()));
            }
            contenido.contenido = Set(contenido_valor);
        }

        if let Some(orden) = datos_actualizados.orden {
            contenido.orden = Set(orden);
        }

        contenido.updated_at = Set(Some(ahora));
        let contenido_actualizado = contenido.update(&self.db).await?;

        Ok(contenido_actualizado)
    }

    pub async fn eliminar_contenido(&self, id: i32) -> Result<(), AppError> {
        let contenido = PortafolioContenido::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Contenido no encontrado".to_string()))?;

        contenido.delete(&self.db).await?;
        Ok(())
    }
}

#[async_trait]
impl crate::traits::service::CrudService<PortafolioContenidoModel> for PortafolioContenidoService {
    async fn get_all(&self) -> Result<Vec<PortafolioContenidoModel>, AppError> {
        self.obtener_contenidos(None).await.map_err(Into::into)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<PortafolioContenidoModel>, AppError> {
        self.obtener_contenido_por_id(id).await.map_err(Into::into)
    }

    async fn create(&self, data: PortafolioContenidoModel) -> Result<PortafolioContenidoModel, AppError> {
        self.crear_contenido(NuevoPortafolioContenido {
            portafolio_id: data.portafolio_id,
            tipo_contenido: data.tipo_contenido,
            titulo: data.titulo,
            descripcion: data.descripcion,
            contenido: data.contenido,
            orden: data.orden,
        })
        .await
    }

    async fn update(
        &self,
        id: i32,
        data: PortafolioContenidoModel,
    ) -> Result<PortafolioContenidoModel, AppError> {
        self.actualizar_contenido(
            id,
            ActualizarPortafolioContenido {
                tipo_contenido: Some(data.tipo_contenido),
                titulo: Some(data.titulo),
                descripcion: data.descripcion,
                contenido: Some(data.contenido),
                orden: Some(data.orden),
            },
        )
        .await
    }

    async fn delete(&self, id: i32) -> Result<(), AppError> {
        self.eliminar_contenido(id).await
    }
}
