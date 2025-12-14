use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "portafolios_contenidos")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub portafolio_id: i32,
    pub tipo_contenido: String,
    pub titulo: String,
    pub descripcion: Option<String>,
    pub contenido: String,
    pub orden: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevoPortafolioContenido {
    pub portafolio_id: i32,
    pub tipo_contenido: String,
    pub titulo: String,
    pub descripcion: Option<String>,
    pub contenido: String,
    pub orden: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActualizarPortafolioContenido {
    pub tipo_contenido: Option<String>,
    pub titulo: Option<String>,
    pub descripcion: Option<String>,
    pub contenido: Option<String>,
    pub orden: Option<i32>,
}
