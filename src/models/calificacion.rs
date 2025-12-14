use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "calificaciones")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub actividad_id: i32,
    pub estudiante_id: i64,
    pub calificacion: f64,
    pub retroalimentacion: Option<String>,
    pub fecha_calificacion: DateTime<Utc>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevaCalificacion {
    pub actividad_id: i32,
    pub estudiante_id: i64,
    pub calificacion: f64,
    pub retroalimentacion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActualizarCalificacion {
    pub calificacion: Option<f64>,
    pub retroalimentacion: Option<String>,
}
