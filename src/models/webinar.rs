use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "webinars")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub curso_id: i32,
    pub titulo: String,
    pub descripcion: Option<String>,
    pub progreso: i32,            // porcentaje 0-100
    pub estado: String,           // 'no_iniciado' | 'en_progreso' | 'completado'
    pub duracion: Option<String>, // ej: "45 min", "1.5 horas"
    pub modulos: i32,             // número de módulos
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Curso,
    Modulos,
    ProgresoEstudiantes,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Curso => Entity::belongs_to(super::curso::Entity)
                .from(Column::CursoId)
                .to(super::curso::Column::Id)
                .into(),
            Self::Modulos => Entity::has_many(super::webinar_modulo::Entity)
                .from(Column::Id)
                .to(super::webinar_modulo::Column::WebinarId)
                .into(),
            Self::ProgresoEstudiantes => {
                Entity::has_many(super::webinar_progreso_estudiante::Entity)
                    .from(Column::Id)
                    .to(super::webinar_progreso_estudiante::Column::WebinarId)
                    .into()
            }
        }
    }
}

impl Related<super::curso::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Curso.def()
    }
}

impl Related<super::webinar_modulo::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Modulos.def()
    }
}

impl Related<super::webinar_progreso_estudiante::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProgresoEstudiantes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
