use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "temas")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub modulo_id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub orden: i32,
    pub visible: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Modulo,
    Unidades,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Modulo => Entity::belongs_to(super::modulo::Entity)
                .from(Column::ModuloId)
                .to(super::modulo::Column::Id)
                .into(),
            Self::Unidades => Entity::has_many(super::unidad::Entity)
                .from(Column::Id)
                .to(super::unidad::Column::TemaId)
                .into(),
        }
    }
}

impl Related<super::modulo::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Modulo.def()
    }
}

impl Related<super::unidad::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Unidades.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
