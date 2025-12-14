use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "areas_conocimiento", schema_name = "rustdema2")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub color_etiqueta: String,
    pub estado: bool,
    #[sea_orm(column_name = "fecha_creacion")]
    pub fecha_creacion: Option<DateTime<Utc>>,
    #[sea_orm(column_name = "fecha_actualizacion")]
    pub fecha_actualizacion: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::curso::Entity")]
    Curso,
}

impl Related<super::curso::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Curso.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
