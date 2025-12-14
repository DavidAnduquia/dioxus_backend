use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "portafolios")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub estudiante_id: i64,
    pub curso_id: i32,
    pub titulo: String,
    pub descripcion: Option<String>,
    pub estado: String,
    pub fecha_creacion: DateTime<Utc>,
    pub fecha_actualizacion: DateTime<Utc>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Deserialize, Serialize)]
pub struct NuevoPortafolio {
    pub estudiante_id: i64,
    pub curso_id: i32,
    pub titulo: String,
    pub descripcion: Option<String>,
    pub estado: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ActualizarPortafolio {
    pub titulo: Option<String>,
    pub descripcion: Option<String>,
    pub estado: Option<String>,
}
