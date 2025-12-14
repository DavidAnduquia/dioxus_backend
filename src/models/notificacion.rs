use chrono::{DateTime, Utc};
use sea_orm::{entity::prelude::*, JsonValue};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "notificaciones", schema_name = "rustdema2")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub usuario_id: i32,
    pub titulo: String,
    pub mensaje: String,
    pub tipo: String,
    pub leida: bool,
    pub enlace: Option<String>,
    pub datos_adicionales: Option<JsonValue>,
    #[sea_orm(column_name = "fecha_creacion")]
    pub created_at: Option<DateTime<Utc>>,
    #[sea_orm(column_name = "fecha_actualizacion")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevaNotificacion {
    pub usuario_id: i32,
    pub titulo: String,
    pub mensaje: String,
    pub tipo: String,
    pub leida: Option<bool>,
    pub enlace: Option<String>,
    pub datos_adicionales: Option<serde_json::Value>,
}
