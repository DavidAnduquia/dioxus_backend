use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "webinar_progreso_estudiantes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub webinar_id: i32,
    #[sea_orm(primary_key)]
    pub estudiante_id: i32,
    pub progreso_actual: i32, // porcentaje 0-100
    pub modulos_completados: i32,
    pub tiempo_total_visto: i32, // en minutos
    pub ultima_actividad: Option<DateTime<Utc>>,
    pub completado: bool,
    pub fecha_completado: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Webinar,
    Estudiante,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Webinar => Entity::belongs_to(super::webinar::Entity)
                .from(Column::WebinarId)
                .to(super::webinar::Column::Id)
                .into(),
            Self::Estudiante => Entity::belongs_to(super::usuario::Entity)
                .from(Column::EstudianteId)
                .to(super::usuario::Column::Id)
                .into(),
        }
    }
}

impl Related<super::webinar::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Webinar.def()
    }
}

impl Related<super::usuario::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Estudiante.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
