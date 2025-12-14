use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(schema_name = "rustdema2", table_name = "unidades")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub tema_id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub orden: i32,
    pub visible: bool,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Tema,
    ActividadesEntrega,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Tema => Entity::belongs_to(super::tema::Entity)
                .from(Column::TemaId)
                .to(super::tema::Column::Id)
                .into(),
            Self::ActividadesEntrega => Entity::has_many(super::actividad_entrega::Entity)
                .from(Column::Id)
                .to(super::actividad_entrega::Column::UnidadId)
                .into(),
        }
    }
}

impl Related<super::tema::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tema.def()
    }
}

impl Related<super::actividad_entrega::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ActividadesEntrega.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevaUnidad {
    pub tema_id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub orden: i32,
    pub visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActualizarUnidad {
    pub nombre: Option<String>,
    pub descripcion: Option<String>,
    pub orden: Option<i32>,
    pub visible: Option<bool>,
}
