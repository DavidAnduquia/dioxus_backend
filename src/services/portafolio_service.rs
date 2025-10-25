// use async_trait::async_trait;  // Ahora disponible por Cargo.toml
use sea_orm_migration::async_trait;
use chrono::Utc;
use once_cell::sync::OnceCell;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    models::portafolio::{self, Entity as Portafolio, Model as PortafolioModel},
    utils::errors::AppError,
};

// Estructura para crear un nuevo portafolio
#[derive(Debug, Deserialize, Serialize)]
pub struct NuevoPortafolio {
    pub estudiante_id: i64,
    pub curso_id: i32,
    pub titulo: String,
    pub descripcion: Option<String>,
    pub estado: String,
}

// Estructura para actualizar un portafolio existente
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ActualizarPortafolio {
    pub titulo: Option<String>,
    pub descripcion: Option<String>,
    pub estado: Option<String>,
}

// Singleton compartido optimizado
static PORTAFOLIO_SERVICE: OnceCell<Arc<PortafolioService>> = OnceCell::new();

#[derive(Debug, Clone)]
pub struct PortafolioService {
    db: DatabaseConnection,
}

impl PortafolioService {
    /// Obtiene la instancia global del servicio, inicializándola si es necesario
    pub fn global(db: &DatabaseConnection) -> &'static Arc<Self> {
        PORTAFOLIO_SERVICE.get_or_init(|| {
            Arc::new(Self { db: db.clone() })
        })
    }

    // Crear un nuevo portafolio
    pub async fn crear_portafolio(
        &self,
        nuevo_portafolio: NuevoPortafolio,
    ) -> Result<PortafolioModel, AppError> {
        // Validar campos obligatorios
        if nuevo_portafolio.titulo.trim().is_empty() {
            return Err(AppError::BadRequest(
                "El título del portafolio es obligatorio".to_string(),
            ));
        }

        if nuevo_portafolio.estado.trim().is_empty() {
            return Err(AppError::BadRequest(
                "El estado del portafolio es obligatorio".to_string(),
            ));
        }

        let ahora = Utc::now();
        let portafolio = portafolio::ActiveModel {
            estudiante_id: Set(nuevo_portafolio.estudiante_id),
            curso_id: Set(nuevo_portafolio.curso_id),
            titulo: Set(nuevo_portafolio.titulo),
            descripcion: Set(nuevo_portafolio.descripcion),
            estado: Set(nuevo_portafolio.estado),
            fecha_creacion: Set(ahora),
            fecha_actualizacion: Set(ahora),
            created_at: Set(Some(ahora)),
            updated_at: Set(Some(ahora)),
            ..Default::default()
        };

        let portafolio_creado = portafolio.insert(&self.db).await?;
        Ok(portafolio_creado)
    }

    // Obtener un portafolio por ID
    pub async fn obtener_portafolio_por_id(
        &self,
        id: i32,
    ) -> Result<Option<PortafolioModel>, DbErr> {
        Portafolio::find_by_id(id).one(&self.db).await
    }

    // Obtener portafolios por estudiante
    pub async fn obtener_portafolios_por_estudiante(
        &self,
        estudiante_id: i64,
    ) -> Result<Vec<PortafolioModel>, DbErr> {
        Portafolio::find()
            .filter(portafolio::Column::EstudianteId.eq(estudiante_id))
            .order_by_desc(portafolio::Column::FechaActualizacion)
            .all(&self.db)
            .await
    }

    // Obtener portafolios por curso
    pub async fn obtener_portafolios_por_curso(
        &self,
        curso_id: i32,
    ) -> Result<Vec<PortafolioModel>, DbErr> {
        Portafolio::find()
            .filter(portafolio::Column::CursoId.eq(curso_id))
            .order_by_desc(portafolio::Column::FechaActualizacion)
            .all(&self.db)
            .await
    }

    // Actualizar un portafolio
    pub async fn actualizar_portafolio(
        &self,
        id: i32,
        datos_actualizados: ActualizarPortafolio,
    ) -> Result<PortafolioModel, AppError> {
        let portafolio = Portafolio::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Portafolio no encontrado".to_string()))?;

        let mut portafolio: portafolio::ActiveModel = portafolio.into();
        let ahora = Utc::now();

        if let Some(titulo) = datos_actualizados.titulo {
            if titulo.trim().is_empty() {
                return Err(AppError::BadRequest(
                    "El título del portafolio no puede estar vacío".to_string(),
                ));
            }
            portafolio.titulo = Set(titulo);
        }

        if let Some(descripcion) = datos_actualizados.descripcion {
            portafolio.descripcion = Set(Some(descripcion));
        }

        if let Some(estado) = datos_actualizados.estado {
            if estado.trim().is_empty() {
                return Err(AppError::BadRequest(
                    "El estado del portafolio no puede estar vacío".to_string(),
                ));
            }
            portafolio.estado = Set(estado);
        }

        portafolio.fecha_actualizacion = Set(ahora);
        portafolio.updated_at = Set(Some(ahora));

        let portafolio_actualizado = portafolio.update(&self.db).await?;
        Ok(portafolio_actualizado)
    }

    // Eliminar un portafolio
    pub async fn eliminar_portafolio(&self, id: i32) -> Result<(), AppError> {
        let portafolio = Portafolio::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Portafolio no encontrado".to_string()))?;

        let _ = portafolio.delete(&self.db).await?;
        Ok(())
    }
}

/*
#[async_trait]
impl crate::traits::service::CrudService<PortafolioModel> for PortafolioService {
    async fn get_all(&self) -> Result<Vec<PortafolioModel>, AppError> {
        Portafolio::find()
            .order_by_desc(portafolio::Column::FechaActualizacion)
            .all(&self.db)
            .await
            .map_err(Into::into)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<PortafolioModel>, AppError> {
        self.obtener_portafolio_por_id(id).await.map_err(Into::into)
    }

    async fn create(&self, data: PortafolioModel) -> Result<PortafolioModel, AppError> {
        self.crear_portafolio(NuevoPortafolio {
            estudiante_id: data.estudiante_id,
            curso_id: data.curso_id,
            titulo: data.titulo,
            descripcion: data.descripcion,
            estado: data.estado,
        })
        .await
    }

    async fn update(
        &self,
        id: i32,
        data: PortafolioModel,
    ) -> Result<PortafolioModel, AppError> {
        self.actualizar_portafolio(
            id,
            ActualizarPortafolio {
                titulo: Some(data.titulo),
                descripcion: data.descripcion,
                estado: Some(data.estado),
            },
        )
        .await
    }

    async fn delete(&self, id: i32) -> Result<(), AppError> {
        self.eliminar_portafolio(id).await
    }
}
