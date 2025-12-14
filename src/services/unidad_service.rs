use axum::extract::FromRef;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait, Order,
    QueryFilter, QueryOrder, Set,
};

use crate::{
    database::DbExecutor,
    models::{
        unidad::{self, Entity as Unidad, Model as UnidadModel},
        AppState,
    },
    utils::errors::AppError,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct UnidadService {
    db: DbExecutor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevaUnidad {
    pub tema_id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub orden: i32,
    pub visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActualizarUnidad {
    pub nombre: Option<String>,
    pub descripcion: Option<String>,
    pub orden: Option<i32>,
    pub visible: Option<bool>,
}

impl UnidadService {
    pub fn new(db: DbExecutor) -> Self {
        Self { db }
    }

    fn connection(&self) -> DatabaseConnection {
        self.db.connection()
    }

    pub async fn crear_unidad(&self, nueva_unidad: NuevaUnidad) -> Result<UnidadModel, AppError> {
        if nueva_unidad.nombre.trim().is_empty() {
            return Err(AppError::BadRequest("El nombre es obligatorio".into()));
        }

        let db = self.connection();

        let unidad = unidad::ActiveModel {
            id: Set(0),
            tema_id: Set(nueva_unidad.tema_id),
            nombre: Set(nueva_unidad.nombre),
            descripcion: Set(nueva_unidad.descripcion),
            orden: Set(nueva_unidad.orden),
            visible: Set(nueva_unidad.visible),
        };

        let unidad_creada = unidad.insert(&db).await?;
        Ok(unidad_creada)
    }

    pub async fn obtener_unidades_por_tema(&self, tema_id: i32) -> Result<Vec<UnidadModel>, DbErr> {
        let db = self.connection();
        Unidad::find()
            .filter(unidad::Column::TemaId.eq(tema_id))
            .order_by(unidad::Column::Orden, Order::Asc)
            .all(&db)
            .await
    }

    pub async fn obtener_unidad_por_id(&self, id: i32) -> Result<Option<UnidadModel>, DbErr> {
        let db = self.connection();
        Unidad::find_by_id(id).one(&db).await
    }

    pub async fn actualizar_unidad(
        &self,
        id: i32,
        datos: ActualizarUnidad,
    ) -> Result<UnidadModel, AppError> {
        let db = self.connection();
        let unidad = Unidad::find_by_id(id)
            .one(&db)
            .await?
            .ok_or_else(|| AppError::NotFound("Unidad no encontrada".into()))?;

        let mut unidad: unidad::ActiveModel = unidad.into();

        if let Some(nombre) = datos.nombre {
            if nombre.trim().is_empty() {
                return Err(AppError::BadRequest(
                    "El nombre no puede estar vacío".into(),
                ));
            }
            unidad.nombre = Set(nombre);
        }

        if let Some(descripcion) = datos.descripcion {
            unidad.descripcion = Set(Some(descripcion));
        }

        if let Some(orden) = datos.orden {
            unidad.orden = Set(orden);
        }

        if let Some(visible) = datos.visible {
            unidad.visible = Set(visible);
        }

        let unidad_actualizada = unidad.update(&db).await?;
        Ok(unidad_actualizada)
    }

    pub async fn eliminar_unidad(&self, id: i32) -> Result<(), AppError> {
        let db = self.connection();
        let unidad = Unidad::find_by_id(id)
            .one(&db)
            .await?
            .ok_or_else(|| AppError::NotFound("Unidad no encontrada".into()))?;

        unidad.delete(&db).await?;
        Ok(())
    }
}

impl FromRef<AppState> for UnidadService {
    fn from_ref(state: &AppState) -> Self {
        let executor = state
            .db
            .clone()
            .expect("Database connection is not available");
        UnidadService::new(executor)
    }
}
