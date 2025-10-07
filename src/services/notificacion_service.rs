use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set, TransactionTrait,
};
use serde_json::Value;

use crate::{
    models::notificacion::{self, Entity as Notificacion, Model as NotificacionModel},
    utils::errors::AppError,
};

#[derive(Debug, Clone)]
pub struct NotificacionService {
    db: DatabaseConnection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevaNotificacion {
    pub usuario_id: i64,
    pub titulo: String,
    pub mensaje: String,
    pub tipo: String,
    pub leida: Option<bool>,
    pub enlace: Option<String>,
    pub datos_adicionales: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActualizarNotificacion {
    pub titulo: Option<String>,
    pub mensaje: Option<String>,
    pub tipo: Option<String>,
    pub leida: Option<bool>,
    pub enlace: Option<String>,
    pub datos_adicionales: Option<Value>,
}

impl NotificacionService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn crear_notificacion(
        &self,
        nueva_notificacion: NuevaNotificacion,
    ) -> Result<NotificacionModel, AppError> {
        // Validaciones
        if nueva_notificacion.titulo.trim().is_empty() {
            return Err(AppError::BadRequest("El título es obligatorio".to_string()));
        }
        if nueva_notificacion.mensaje.trim().is_empty() {
            return Err(AppError::BadRequest("El mensaje es obligatorio".to_string()));
        }
        if nueva_notificacion.tipo.trim().is_empty() {
            return Err(AppError::BadRequest("El tipo es obligatorio".to_string()));
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

        let notificacion_creada = notificacion.insert(&self.db).await?;
        Ok(notificacion_creada)
    }

    pub async fn obtener_por_id(
        &self,
        id: i32,
    ) -> Result<Option<NotificacionModel>, DbErr> {
        Notificacion::find_by_id(id).one(&self.db).await
    }

    pub async fn obtener_por_usuario(
        &self,
        usuario_id: i64,
        leida: Option<bool>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<(Vec<NotificacionModel>, u64), AppError> {
        let mut query = Notificacion::find()
            .filter(notificacion::Column::UsuarioId.eq(usuario_id));

        if let Some(leida_val) = leida {
            query = query.filter(notificacion::Column::Leida.eq(leida_val));
        }

        let total = query.clone().count(&self.db).await?;

        if let Some(limit_val) = limit {
            query = query.limit(limit_val);
        }

        if let Some(offset_val) = offset {
            query = query.offset(offset_val);
        }

        let notificaciones = query
            .order_by_desc(notificacion::Column::CreatedAt)
            .all(&self.db)
            .await?;

        Ok((notificaciones, total))
    }

    pub async fn marcar_como_leida(
        &self,
        id: i32,
    ) -> Result<NotificacionModel, AppError> {
        let notificacion = Notificacion::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Notificación no encontrada".to_string()))?;

        let mut notificacion: notificacion::ActiveModel = notificacion.into();
        notificacion.leida = Set(true);
        notificacion.updated_at = Set(Some(Utc::now()));

        let notificacion_actualizada = notificacion.update(&self.db).await?;
        Ok(notificacion_actualizada)
    }

    pub async fn marcar_todas_como_leidas(
        &self,
        usuario_id: i64,
    ) -> Result<u64, AppError> {
        let result = Notificacion::update_many()
            .col_expr(notificacion::Column::Leida, Expr::value(true))
            .col_expr(notificacion::Column::UpdatedAt, Expr::value(Utc::now()))
            .filter(notificacion::Column::UsuarioId.eq(usuario_id))
            .filter(notificacion::Column::Leida.eq(false))
            .exec(&self.db)
            .await?;

        Ok(result.rows_affected)
    }

    pub async fn eliminar_notificacion(&self, id: i32) -> Result<(), AppError> {
        let notificacion = Notificacion::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Notificación no encontrada".to_string()))?;

        notificacion.delete(&self.db).await?;
        Ok(())
    }

    pub async fn obtener_no_leidas(
        &self,
        usuario_id: i64,
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
            .all(&self.db)
            .await
    }

    pub async fn actualizar_datos(
        &self,
        id: i32,
        datos: Value,
    ) -> Result<NotificacionModel, AppError> {
        let notificacion = Notificacion::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Notificación no encontrada".to_string()))?;

        let mut notificacion: notificacion::ActiveModel = notificacion.into();
        notificacion.datos_adicionales = Set(Some(datos));
        notificacion.updated_at = Set(Some(Utc::now()));

        let notificacion_actualizada = notificacion.update(&self.db).await?;
        Ok(notificacion_actualizada)
    }

    pub async fn obtener_estadisticas(
        &self,
        usuario_id: i64,
    ) -> Result<(u64, u64, u64), AppError> {
        let total = Notificacion::find()
            .filter(notificacion::Column::UsuarioId.eq(usuario_id))
            .count(&self.db)
            .await?;

        let leidas = Notificacion::find()
            .filter(notificacion::Column::UsuarioId.eq(usuario_id))
            .filter(notificacion::Column::Leida.eq(true))
            .count(&self.db)
            .await?;

        let no_leidas = Notificacion::find()
            .filter(notificacion::Column::UsuarioId.eq(usuario_id))
            .filter(notificacion::Column::Leida.eq(false))
            .count(&self.db)
            .await?;

        Ok((total, leidas, no_leidas))
    }
}

#[async_trait]
impl crate::traits::service::CrudService<NotificacionModel> for NotificacionService {
    async fn get_all(&self) -> Result<Vec<NotificacionModel>, AppError> {
        Notificacion::find()
            .order_by_desc(notificacion::Column::CreatedAt)
            .all(&self.db)
            .await
            .map_err(Into::into)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<NotificacionModel>, AppError> {
        self.obtener_por_id(id).await.map_err(Into::into)
    }

    async fn create(&self, data: NotificacionModel) -> Result<NotificacionModel, AppError> {
        self.crear_notificacion(NuevaNotificacion {
            usuario_id: data.usuario_id,
            titulo: data.titulo,
            mensaje: data.mensaje,
            tipo: data.tipo,
            leida: Some(data.leida),
            enlace: data.enlace,
            datos_adicionales: data.datos_adicionales,
        })
        .await
    }

    async fn update(
        &self,
        id: i32,
        data: NotificacionModel,
    ) -> Result<NotificacionModel, AppError> {
        self.actualizar_datos(
            id,
            ActualizarNotificacion {
                titulo: Some(data.titulo),
                mensaje: Some(data.mensaje),
                tipo: Some(data.tipo),
                leida: Some(data.leida),
                enlace: data.enlace,
                datos_adicionales: data.datos_adicionales,
            },
        )
        .await
    }

    async fn delete(&self, id: i32) -> Result<(), AppError> {
        self.eliminar_notificacion(id).await
    }
}
