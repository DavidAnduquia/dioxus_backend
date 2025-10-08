use chrono::{NaiveTime, Utc};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait, QueryFilter, Set};

use crate::{
    models::actividad::{self, Entity as Actividad, Model as ActividadModel, NewActividad, UpdateActividad},
    utils::errors::AppError,
};

#[derive(Debug, Clone)]
pub struct ActividadService {
    db: DatabaseConnection,
}

impl ActividadService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn obtener_actividades(&self) -> Result<Vec<ActividadModel>, DbErr> {
        Actividad::find().all(&self.db).await
    }

    pub async fn obtener_actividades_por_curso(
        &self,
        curso_id: i32,
    ) -> Result<Vec<ActividadModel>, DbErr> {
        Actividad::find()
            .filter(actividad::Column::CursoId.eq(curso_id))
            .all(&self.db)
            .await
    }

    pub async fn obtener_actividad_por_id(&self, id: i32) -> Result<Option<ActividadModel>, DbErr> {
        Actividad::find_by_id(id).one(&self.db).await
    }

    pub async fn crear_actividad(
        &self,
        nueva_actividad: NewActividad,
    ) -> Result<ActividadModel, AppError> {
        if nueva_actividad.nombre.trim().is_empty() {
            return Err(AppError::BadRequest("El nombre es obligatorio".to_string()));
        }

        let ahora = Utc::now();
        let actividad = actividad::ActiveModel {
            curso_id: Set(nueva_actividad.curso_id),
            profesor_id: Set(nueva_actividad.profesor_id),
            nombre: Set(nueva_actividad.nombre),
            descripcion: Set(nueva_actividad.descripcion),
            fecha_inicio: Set(nueva_actividad.fecha_inicio),
            fecha_fin: Set(nueva_actividad.fecha_fin),
            tipo_actividad: Set(nueva_actividad.tipo_actividad),
            privacidad: Set(nueva_actividad.privacidad),
            created_at: Set(Some(ahora)),
            updated_at: Set(Some(ahora)),
            ..Default::default()
        };

        let actividad_creada = actividad.insert(&self.db).await?;
        Ok(actividad_creada)
    }

    pub async fn actualizar_actividad(
        &self,
        id: i32,
        datos_actualizados: UpdateActividad,
    ) -> Result<ActividadModel, AppError> {
        let actividad = Actividad::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Actividad no encontrada".to_string()))?;

        let mut actividad: actividad::ActiveModel = actividad.into();

        if let Some(curso_id) = datos_actualizados.curso_id {
            actividad.curso_id = Set(curso_id);
        }
        if let Some(profesor_id) = datos_actualizados.profesor_id {
            actividad.profesor_id = Set(profesor_id);
        }
        if let Some(nombre) = datos_actualizados.nombre {
            if nombre.trim().is_empty() {
                return Err(AppError::BadRequest("El nombre no puede estar vacío".to_string()));
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

        actividad.updated_at = Set(Some(Utc::now()));
        let actividad_actualizada = actividad.update(&self.db).await?;
        Ok(actividad_actualizada)
    }

    pub async fn eliminar_actividad(&self, id: i32) -> Result<(), AppError> {
        let actividad = Actividad::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Actividad no encontrada".to_string()))?;

        actividad.delete(&self.db).await?;
        Ok(())
    }
}
