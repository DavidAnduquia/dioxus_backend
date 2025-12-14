use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "modulos")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub curso_id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub orden: i32,
    pub visible: bool,
    pub tipo: String, // 'estructura_contenido' | 'taller' | 'evaluacion' | etc.
    pub fecha_inicio: Option<DateTime<Utc>>,
    pub fecha_fin: Option<DateTime<Utc>>,
    pub duracion_estimada: Option<i32>, // en minutos
    pub obligatorio: bool,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Curso,
    Temas,
    Archivos,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Curso => Entity::belongs_to(super::curso::Entity)
                .from(Column::CursoId)
                .to(super::curso::Column::Id)
                .into(),
            Self::Temas => Entity::has_many(super::tema::Entity)
                .from(Column::Id)
                .to(super::tema::Column::ModuloId)
                .into(),
            Self::Archivos => Entity::has_many(super::modulo_archivo::Entity)
                .from(Column::Id)
                .to(super::modulo_archivo::Column::ModuloId)
                .into(),
        }
    }
}

impl Related<super::curso::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Curso.def()
    }
}

impl Related<super::tema::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Temas.def()
    }
}

impl Related<super::modulo_archivo::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Archivos.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevoModulo {
    pub curso_id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub orden: i32,
    pub tipo: Option<String>,
    pub visible: bool,
    pub fecha_inicio: Option<DateTime<Utc>>,
    pub fecha_fin: Option<DateTime<Utc>>,
    pub duracion_estimada: Option<i32>,
    pub obligatorio: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActualizarModulo {
    pub nombre: Option<String>,
    pub descripcion: Option<String>,
    pub orden: Option<i32>,
    pub tipo: Option<String>,
    pub visible: Option<bool>,
    pub fecha_inicio: Option<DateTime<Utc>>,
    pub fecha_fin: Option<DateTime<Utc>>,
    pub duracion_estimada: Option<i32>,
    pub obligatorio: Option<bool>,
}
