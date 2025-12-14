use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "modulos_archivos")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub modulo_id: i32,
    pub nombre_archivo: String,
    pub ruta_archivo: String,
    pub tipo_archivo: String,
    pub tamano: i64,
    pub descripcion: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Modulo,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Modulo => Entity::belongs_to(super::modulo::Entity)
                .from(Column::ModuloId)
                .to(super::modulo::Column::Id)
                .into(),
        }
    }
}

impl Related<super::modulo::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Modulo.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevoModuloArchivo {
    pub modulo_id: i32,
    pub nombre_archivo: String,
    pub ruta_archivo: String,
    pub tipo_archivo: String,
    pub tamano: i64,
    pub descripcion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActualizarModuloArchivo {
    pub nombre_archivo: Option<String>,
    pub ruta_archivo: Option<String>,
    pub tipo_archivo: Option<String>,
    pub tamano: Option<i64>,
    pub descripcion: Option<String>,
}
