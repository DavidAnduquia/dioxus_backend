use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "portafolios")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub curso_id: Option<i32>,
    pub nombre: String,
    pub descripcion: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Deserialize, Serialize)]
pub struct NuevoPortafolio {
    pub curso_id: Option<i32>,
    pub nombre: String,
    pub descripcion: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ActualizarPortafolio {
    pub nombre: Option<String>,
    pub descripcion: Option<String>,
}
