use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "entregas")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub actividad_entrega_id: i32,
    pub estudiante_id: i32,
    pub documento_nombre: String,
    pub documento_tipo: String,
    pub documento_tamanio: i64,
    pub documento_url: String,
    pub fecha_entrega: DateTime<Utc>,
    pub calificacion: Option<f32>,
    pub comentario_profesor: Option<String>,
    pub fecha_calificacion: Option<DateTime<Utc>>,
    pub estado: String, // 'pendiente' | 'calificado' | 'rechazado'
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    ActividadEntrega,
    Estudiante,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::ActividadEntrega => Entity::belongs_to(super::actividad_entrega::Entity)
                .from(Column::ActividadEntregaId)
                .to(super::actividad_entrega::Column::Id)
                .into(),
            Self::Estudiante => Entity::belongs_to(super::usuario::Entity)
                .from(Column::EstudianteId)
                .to(super::usuario::Column::Id)
                .into(),
        }
    }
}

impl Related<super::actividad_entrega::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ActividadEntrega.def()
    }
}

impl Related<super::usuario::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Estudiante.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
