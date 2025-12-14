use axum::extract::FromRef;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, Set,
};
use serde_json::Value;

use crate::{
    database::DbExecutor,
    models::{
        notificacion::{self, Entity as Notificacion, Model as NotificacionModel},
        AppState,
    },
    utils::errors::AppError,
};

pub use crate::models::notificacion::NuevaNotificacion;

#[derive(Debug, Clone)]
pub struct NotificacionService {
    db: DbExecutor,
}

impl FromRef<AppState> for NotificacionService {
    fn from_ref(state: &AppState) -> Self {
        let executor = state
            .db
            .clone()
            .expect("Database connection is not available");
        NotificacionService::new(executor)
    }
}

impl NotificacionService {
    pub fn new(db: DbExecutor) -> Self {
        Self { db }
    }

    pub async fn crear_notificacion(
        &self,
        nueva_notificacion: NuevaNotificacion,
    ) -> Result<NotificacionModel, AppError> {
        // Validaciones
        if nueva_notificacion.titulo.trim().is_empty() {
            return Err(AppError::BadRequest("El título es obligatorio".into()));
        }
        if nueva_notificacion.mensaje.trim().is_empty() {
            return Err(AppError::BadRequest("El mensaje es obligatorio".into()));
        }
        if nueva_notificacion.tipo.trim().is_empty() {
            return Err(AppError::BadRequest("El tipo es obligatorio".into()));
        }

        // Limpiar datos adicionales
        let datos_adicionales = match nueva_notificacion.datos_adicionales {
            Some(data) if !data.is_null() => Some(data),
            _ => None,
        };

        let ahora = Utc::now();
        let notificacion = notificacion::ActiveModel {
            usuario_id: Set(nueva_notificacion.usuario_id),
            titulo: Set(nueva_notificacion.titulo),
            mensaje: Set(nueva_notificacion.mensaje),
            tipo: Set(nueva_notificacion.tipo),
            leida: Set(nueva_notificacion.leida.unwrap_or(false)),
            enlace: Set(nueva_notificacion.enlace),
            datos_adicionales: Set(datos_adicionales),
            created_at: Set(Some(ahora)),
            updated_at: Set(Some(ahora)),
            ..Default::default()
        };

        let notificacion_creada = notificacion.insert(&self.db.connection()).await?;
        Ok(notificacion_creada)
    }

    pub async fn obtener_por_usuario(
        &self,
        usuario_id: i32,
        leida: Option<bool>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<(Vec<NotificacionModel>, u64), AppError> {
        let mut query = Notificacion::find().filter(notificacion::Column::UsuarioId.eq(usuario_id));

        if let Some(leida_val) = leida {
            query = query.filter(notificacion::Column::Leida.eq(leida_val));
        }

        let total = query.clone().count(&self.db.connection()).await?;

        if let Some(limit_val) = limit {
            query = query.limit(limit_val);
        }

        if let Some(offset_val) = offset {
            query = query.offset(offset_val);
        }

        let notificaciones = query
            .order_by_desc(notificacion::Column::CreatedAt)
            .all(&self.db.connection())
            .await?;

        Ok((notificaciones, total))
    }

    pub async fn marcar_como_leida(&self, id: i32) -> Result<NotificacionModel, AppError> {
        let notificacion = Notificacion::find_by_id(id)
            .one(&self.db.connection())
            .await?
            .ok_or_else(|| AppError::NotFound("Notificación no encontrada".into()))?;

        let mut notificacion: notificacion::ActiveModel = notificacion.into();
        notificacion.leida = Set(true);
        notificacion.updated_at = Set(Some(Utc::now()));

        let notificacion_actualizada = notificacion.update(&self.db.connection()).await?;
        Ok(notificacion_actualizada)
    }

    pub async fn marcar_todas_como_leidas(&self, usuario_id: i32) -> Result<u64, AppError> {
        // Obtener todas las notificaciones no leídas del usuario
        let notificaciones = Notificacion::find()
            .filter(notificacion::Column::UsuarioId.eq(usuario_id))
            .filter(notificacion::Column::Leida.eq(false))
            .all(&self.db.connection())
            .await?;

        let mut count = 0;
        for notif in notificaciones {
            let mut notif: notificacion::ActiveModel = notif.into();
            notif.leida = Set(true);
            notif.updated_at = Set(Some(Utc::now()));
            notif.update(&self.db.connection()).await?;
            count += 1;
        }

        Ok(count)
    }

    #[allow(dead_code)]
    pub async fn eliminar_notificacion(&self, id: i32) -> Result<(), AppError> {
        let notificacion = Notificacion::find_by_id(id)
            .one(&self.db.connection())
            .await?
            .ok_or_else(|| AppError::NotFound("Notificación no encontrada".into()))?;

        notificacion.delete(&self.db.connection()).await?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn obtener_no_leidas(
        &self,
        usuario_id: i32,
        limit: Option<u64>,
    ) -> Result<Vec<NotificacionModel>, DbErr> {
        let mut query = Notificacion::find()
            .filter(notificacion::Column::UsuarioId.eq(usuario_id))
            .filter(notificacion::Column::Leida.eq(false));

        if let Some(limit_val) = limit {
            query = query.limit(limit_val);
        }

        query
            .order_by_desc(notificacion::Column::CreatedAt)
            .all(&self.db.connection())
            .await
    }

    #[allow(dead_code)]
    pub async fn actualizar_datos(
        &self,
        id: i32,
        datos: Value,
    ) -> Result<NotificacionModel, AppError> {
        let notificacion = Notificacion::find_by_id(id)
            .one(&self.db.connection())
            .await?
            .ok_or_else(|| AppError::NotFound("Notificación no encontrada".into()))?;

        let mut notificacion: notificacion::ActiveModel = notificacion.into();
        notificacion.datos_adicionales = Set(Some(datos));
        notificacion.updated_at = Set(Some(Utc::now()));

        let notificacion_actualizada = notificacion.update(&self.db.connection()).await?;
        Ok(notificacion_actualizada)
    }

    #[allow(dead_code)]
    pub async fn obtener_estadisticas(&self, usuario_id: i32) -> Result<(u64, u64, u64), AppError> {
        let total = Notificacion::find()
            .filter(notificacion::Column::UsuarioId.eq(usuario_id))
            .count(&self.db.connection())
            .await?;

        let leidas = Notificacion::find()
            .filter(notificacion::Column::UsuarioId.eq(usuario_id))
            .filter(notificacion::Column::Leida.eq(true))
            .count(&self.db.connection())
            .await?;

        let no_leidas = Notificacion::find()
            .filter(notificacion::Column::UsuarioId.eq(usuario_id))
            .filter(notificacion::Column::Leida.eq(false))
            .count(&self.db.connection())
            .await?;

        Ok((total, leidas, no_leidas))
    }
}
