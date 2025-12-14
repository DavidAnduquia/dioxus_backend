use crate::{
    database::DbExecutor,
    models::modulo::{self, Entity as Modulo, Model as ModuloModel},
    models::AppState,
    utils::errors::AppError,
};
use axum::extract::FromRef;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait, Order,
    QueryFilter, QueryOrder, Set,
};

pub use crate::models::modulo::{ActualizarModulo, NuevoModulo};

#[derive(Debug, Clone)]
pub struct ModuloService {
    db: DbExecutor,
}

impl ModuloService {
    pub fn new(db: DbExecutor) -> Self {
        Self { db }
    }

    fn connection(&self) -> DatabaseConnection {
        self.db.connection()
    }

    pub async fn crear_modulo(&self, nuevo_modulo: NuevoModulo) -> Result<ModuloModel, AppError> {
        if nuevo_modulo.nombre.trim().is_empty() {
            return Err(AppError::BadRequest("El nombre es obligatorio".into()));
        }

        let db = self.connection();
        let modulo = modulo::ActiveModel {
            // No establecer id: dejar que la BD autoincremente
            curso_id: Set(nuevo_modulo.curso_id),
            nombre: Set(nuevo_modulo.nombre),
            descripcion: Set(nuevo_modulo.descripcion),
            orden: Set(nuevo_modulo.orden),
            tipo: Set(nuevo_modulo
                .tipo
                .unwrap_or_else(|| "estructura_contenido".to_string())),
            visible: Set(nuevo_modulo.visible),
            fecha_inicio: Set(nuevo_modulo.fecha_inicio),
            fecha_fin: Set(nuevo_modulo.fecha_fin),
            duracion_estimada: Set(nuevo_modulo.duracion_estimada),
            obligatorio: Set(nuevo_modulo.obligatorio.unwrap_or(true)),
            ..Default::default()
        };

        let modulo_creado = modulo.insert(&db).await?;
        Ok(modulo_creado)
    }

    pub async fn obtener_modulos_por_curso(
        &self,
        curso_id: i32,
    ) -> Result<Vec<ModuloModel>, DbErr> {
        let db = self.connection();
        Modulo::find()
            .filter(modulo::Column::CursoId.eq(curso_id))
            .order_by(modulo::Column::Orden, Order::Asc)
            .all(&db)
            .await
    }

    pub async fn obtener_modulo_por_id(&self, id: i32) -> Result<Option<ModuloModel>, DbErr> {
        let db = self.connection();
        Modulo::find_by_id(id).one(&db).await
    }

    pub async fn actualizar_modulo(
        &self,
        id: i32,
        datos_actualizados: ActualizarModulo,
    ) -> Result<ModuloModel, AppError> {
        let db = self.connection();
        let modulo = Modulo::find_by_id(id)
            .one(&db)
            .await?
            .ok_or_else(|| AppError::NotFound("Módulo no encontrado".into()))?;

        let mut modulo: modulo::ActiveModel = modulo.into();

        if let Some(nombre) = datos_actualizados.nombre {
            if nombre.trim().is_empty() {
                return Err(AppError::BadRequest(
                    "El nombre no puede estar vacío".into(),
                ));
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

        if let Some(tipo) = datos_actualizados.tipo {
            modulo.tipo = Set(tipo);
        }

        if let Some(fecha_inicio) = datos_actualizados.fecha_inicio {
            modulo.fecha_inicio = Set(Some(fecha_inicio));
        }

        if let Some(fecha_fin) = datos_actualizados.fecha_fin {
            modulo.fecha_fin = Set(Some(fecha_fin));
        }

        if let Some(duracion_estimada) = datos_actualizados.duracion_estimada {
            modulo.duracion_estimada = Set(Some(duracion_estimada));
        }

        if let Some(obligatorio) = datos_actualizados.obligatorio {
            modulo.obligatorio = Set(obligatorio);
        }

        let modulo_actualizado = modulo.update(&db).await?;

        Ok(modulo_actualizado)
    }

    pub async fn eliminar_modulo(&self, id: i32) -> Result<(), AppError> {
        let db = self.connection();
        let modulo = Modulo::find_by_id(id)
            .one(&db)
            .await?
            .ok_or_else(|| AppError::NotFound("Módulo no encontrado".into()))?;

        modulo.delete(&db).await?;
        Ok(())
    }
}

impl FromRef<AppState> for ModuloService {
    fn from_ref(state: &AppState) -> Self {
        let executor = state
            .db
            .clone()
            .expect("Database connection is not available");
        ModuloService::new(executor)
    }
}
