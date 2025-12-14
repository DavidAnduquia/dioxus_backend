use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "actividades_entrega")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub unidad_id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub fecha_limite: DateTime<Utc>,
    pub tipo_actividad: String, // 'entrega_obligatoria' | 'entrega_opcional'
    pub activo: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Unidad,
    Entregas,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Unidad => Entity::belongs_to(super::unidad::Entity)
                .from(Column::UnidadId)
                .to(super::unidad::Column::Id)
                .into(),
            Self::Entregas => Entity::has_many(super::entrega::Entity)
                .from(Column::Id)
                .to(super::entrega::Column::ActividadEntregaId)
                .into(),
        }
    }
}

impl Related<super::unidad::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Unidad.def()
    }
}

impl Related<super::entrega::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Entregas.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
