use axum::extract::FromRef;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait, Order,
    QueryFilter, QueryOrder, Set,
};

use crate::{
    database::DbExecutor,
    models::{
        tema::{self, Entity as Tema, Model as TemaModel},
        AppState,
    },
    utils::errors::AppError,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct TemaService {
    db: DbExecutor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevoTema {
    pub modulo_id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub orden: i32,
    pub visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActualizarTema {
    pub nombre: Option<String>,
    pub descripcion: Option<String>,
    pub orden: Option<i32>,
    pub visible: Option<bool>,
}

impl TemaService {
    pub fn new(db: DbExecutor) -> Self {
        Self { db }
    }

    fn connection(&self) -> DatabaseConnection {
        self.db.connection()
    }

    pub async fn crear_tema(&self, nuevo_tema: NuevoTema) -> Result<TemaModel, AppError> {
        if nuevo_tema.nombre.trim().is_empty() {
            return Err(AppError::BadRequest("El nombre es obligatorio".into()));
        }

        let db = self.connection();
        let ahora = Utc::now();

        let tema = tema::ActiveModel {
            id: Set(0),
            modulo_id: Set(nuevo_tema.modulo_id),
            nombre: Set(nuevo_tema.nombre),
            descripcion: Set(nuevo_tema.descripcion),
            orden: Set(nuevo_tema.orden),
            visible: Set(nuevo_tema.visible),
            created_at: Set(Some(ahora)),
            updated_at: Set(Some(ahora)),
        };

        let tema_creado = tema.insert(&db).await?;
        Ok(tema_creado)
    }

    pub async fn obtener_temas_por_modulo(&self, modulo_id: i32) -> Result<Vec<TemaModel>, DbErr> {
        let db = self.connection();
        Tema::find()
            .filter(tema::Column::ModuloId.eq(modulo_id))
            .order_by(tema::Column::Orden, Order::Asc)
            .all(&db)
            .await
    }

    pub async fn obtener_tema_por_id(&self, id: i32) -> Result<Option<TemaModel>, DbErr> {
        let db = self.connection();
        Tema::find_by_id(id).one(&db).await
    }

    pub async fn actualizar_tema(
        &self,
        id: i32,
        datos: ActualizarTema,
    ) -> Result<TemaModel, AppError> {
        let db = self.connection();
        let tema = Tema::find_by_id(id)
            .one(&db)
            .await?
            .ok_or_else(|| AppError::NotFound("Tema no encontrado".into()))?;

        let mut tema: tema::ActiveModel = tema.into();
        let ahora = Utc::now();

        if let Some(nombre) = datos.nombre {
            if nombre.trim().is_empty() {
                return Err(AppError::BadRequest(
                    "El nombre no puede estar vacío".into(),
                ));
            }
            tema.nombre = Set(nombre);
        }

        if let Some(descripcion) = datos.descripcion {
            tema.descripcion = Set(Some(descripcion));
        }

        if let Some(orden) = datos.orden {
            tema.orden = Set(orden);
        }

        if let Some(visible) = datos.visible {
            tema.visible = Set(visible);
        }

        tema.updated_at = Set(Some(ahora));
        let tema_actualizado = tema.update(&db).await?;
        Ok(tema_actualizado)
    }

    pub async fn eliminar_tema(&self, id: i32) -> Result<(), AppError> {
        let db = self.connection();
        let tema = Tema::find_by_id(id)
            .one(&db)
            .await?
            .ok_or_else(|| AppError::NotFound("Tema no encontrado".into()))?;

        tema.delete(&db).await?;
        Ok(())
    }
}

impl FromRef<AppState> for TemaService {
    fn from_ref(state: &AppState) -> Self {
        let executor = state
            .db
            .clone()
            .expect("Database connection is not available");
        TemaService::new(executor)
    }
}
