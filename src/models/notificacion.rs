use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sea_orm::{entity::prelude::*, JsonValue};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "notificaciones")]
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
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
