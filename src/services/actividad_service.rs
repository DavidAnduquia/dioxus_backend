use axum::extract::FromRef;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};

use crate::{
    database::DbExecutor,
    models::{
        actividad::{
            self, Entity as Actividad, Model as ActividadModel, NewActividad, UpdateActividad,
        },
        AppState,
    },
    utils::errors::AppError,
};

#[derive(Debug, Clone)]
pub struct ActividadService {
    db: DbExecutor,
}

impl ActividadService {
    pub fn new(db: DbExecutor) -> Self {
        Self { db }
    }

    fn connection(&self) -> DatabaseConnection {
        self.db.connection()
    }

    pub async fn obtener_actividades(&self) -> Result<Vec<ActividadModel>, DbErr> {
        let db = self.connection();
        Actividad::find().all(&db).await
    }

    pub async fn obtener_actividades_por_curso(
        &self,
        curso_id: i32,
    ) -> Result<Vec<ActividadModel>, DbErr> {
        let db = self.connection();
        Actividad::find()
            .filter(actividad::Column::CursoId.eq(curso_id))
            .all(&db)
            .await
    }

    pub async fn obtener_actividad_por_id(&self, id: i32) -> Result<Option<ActividadModel>, DbErr> {
        let db = self.connection();
        Actividad::find_by_id(id).one(&db).await
    }

    pub async fn crear_actividad(
        &self,
        nueva_actividad: NewActividad,
    ) -> Result<ActividadModel, AppError> {
        if nueva_actividad.nombre.trim().is_empty() {
            return Err(AppError::BadRequest("El nombre es obligatorio".into()));
        }

        let actividad = actividad::ActiveModel {
            curso_id: Set(nueva_actividad.curso_id),
            profesor_id: Set(nueva_actividad.profesor_id),
            nombre: Set(nueva_actividad.nombre),
            descripcion: Set(nueva_actividad.descripcion),
            fecha_inicio: Set(nueva_actividad.fecha_inicio),
            fecha_fin: Set(nueva_actividad.fecha_fin),
            tipo_actividad: Set(nueva_actividad.tipo_actividad),
            privacidad: Set(nueva_actividad.privacidad),
            ..Default::default()
        };

        let db = self.connection();
        let actividad_creada = actividad.insert(&db).await?;
        Ok(actividad_creada)
    }

    pub async fn actualizar_actividad(
        &self,
        id: i32,
        datos_actualizados: UpdateActividad,
    ) -> Result<ActividadModel, AppError> {
        let db = self.connection();
        let mut actividad = actividad::ActiveModel {
            id: Set(id),
            ..Default::default()
        };

        if let Some(curso_id) = datos_actualizados.curso_id {
            actividad.curso_id = Set(curso_id);
        }
        if let Some(profesor_id) = datos_actualizados.profesor_id {
            actividad.profesor_id = Set(profesor_id);
        }
        if let Some(nombre) = datos_actualizados.nombre {
            if nombre.trim().is_empty() {
                return Err(AppError::BadRequest(
                    "El nombre no puede estar vacío".into(),
                ));
            }
            actividad.nombre = Set(nombre);
        }
        if let Some(descripcion) = datos_actualizados.descripcion {
            actividad.descripcion = Set(Some(descripcion));
        }
        if let Some(fecha_inicio) = datos_actualizados.fecha_inicio {
            actividad.fecha_inicio = Set(fecha_inicio);
        }
        if let Some(fecha_fin) = datos_actualizados.fecha_fin {
            actividad.fecha_fin = Set(fecha_fin);
        }
        if let Some(tipo) = datos_actualizados.tipo_actividad {
            actividad.tipo_actividad = Set(tipo);
        }
        if let Some(privacidad) = datos_actualizados.privacidad {
            actividad.privacidad = Set(privacidad);
        }

        let actividad_actualizada = actividad.update(&db).await?;
        Ok(actividad_actualizada)
    }

    pub async fn eliminar_actividad(&self, id: i32) -> Result<(), AppError> {
        let db = self.connection();
        let res = Actividad::delete_by_id(id).exec(&db).await?;

        if res.rows_affected == 0 {
            return Err(AppError::NotFound("Actividad no encontrada".into()));
        }

        Ok(())
    }
}

impl FromRef<AppState> for ActividadService {
    fn from_ref(state: &AppState) -> Self {
        let executor = state
            .db
            .clone()
            .expect("Database connection is not available");
        ActividadService::new(executor)
    }
}
