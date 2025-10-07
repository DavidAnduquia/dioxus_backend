use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};

use crate::{
    models::modulo::{self, Entity as Modulo, Model as ModuloModel},
    utils::errors::AppError,
};

#[derive(Debug, Clone)]
pub struct ModuloService {
    db: DatabaseConnection,
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
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn crear_modulo(
        &self,
        nuevo_modulo: NuevoModulo,
    ) -> Result<ModuloModel, AppError> {
        if nuevo_modulo.nombre.trim().is_empty() {
            return Err(AppError::BadRequest("El nombre es obligatorio".to_string()));
        }

        let ahora = Utc::now();
        let modulo = modulo::ActiveModel {
            curso_id: Set(nuevo_modulo.curso_id),
            nombre: Set(nuevo_modulo.nombre),
            descripcion: Set(nuevo_modulo.descripcion),
            orden: Set(nuevo_modulo.orden),
            visible: Set(nuevo_modulo.visible),
            created_at: Set(Some(ahora)),
            updated_at: Set(Some(ahora)),
            ..Default::default()
        };

        let modulo_creado = modulo.insert(&self.db).await?;
        Ok(modulo_creado)
    }

    pub async fn obtener_modulos_por_curso(
        &self,
        curso_id: i32,
    ) -> Result<Vec<ModuloModel>, DbErr> {
        Modulo::find()
            .filter(modulo::Column::CursoId.eq(curso_id))
            .order_by_asc(modulo::Column::Orden)
            .all(&self.db)
            .await
    }

    pub async fn obtener_modulo_por_id(
        &self,
        id: i32,
    ) -> Result<Option<ModuloModel>, DbErr> {
        Modulo::find_by_id(id).one(&self.db).await
    }

    pub async fn actualizar_modulo(
        &self,
        id: i32,
        datos_actualizados: ActualizarModulo,
    ) -> Result<ModuloModel, AppError> {
        let modulo = Modulo::find_by_id(id)
            .one(&self.db)
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
        let modulo_actualizado = modulo.update(&self.db).await?;

        Ok(modulo_actualizado)
    }

    pub async fn eliminar_modulo(&self, id: i32) -> Result<(), AppError> {
        let modulo = Modulo::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Módulo no encontrado".to_string()))?;

        modulo.delete(&self.db).await?;
        Ok(())
    }
}

#[async_trait]
impl crate::traits::service::CrudService<ModuloModel> for ModuloService {
    async fn get_all(&self) -> Result<Vec<ModuloModel>, AppError> {
        Modulo::find()
            .order_by_asc(modulo::Column::Orden)
            .all(&self.db)
            .await
            .map_err(Into::into)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<ModuloModel>, AppError> {
        self.obtener_modulo_por_id(id).await.map_err(Into::into)
    }

    async fn create(&self, data: ModuloModel) -> Result<ModuloModel, AppError> {
        self.crear_modulo(NuevoModulo {
            curso_id: data.curso_id,
            nombre: data.nombre,
            descripcion: data.descripcion,
            orden: data.orden,
            visible: data.visible,
        })
        .await
    }

    async fn update(
        &self,
        id: i32,
        data: ModuloModel,
    ) -> Result<ModuloModel, AppError> {
        self.actualizar_modulo(
            id,
            ActualizarModulo {
                nombre: Some(data.nombre),
                descripcion: data.descripcion,
                orden: Some(data.orden),
                visible: Some(data.visible),
            },
        )
        .await
    }

    async fn delete(&self, id: i32) -> Result<(), AppError> {
        self.eliminar_modulo(id).await
    }
}
