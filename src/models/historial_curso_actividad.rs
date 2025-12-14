use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "historial_cursos_actividades")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub historial_curso_id: i32,
    pub actividad_id: i32,
    pub calificacion: Option<f64>,
    pub completado: bool,
    pub fecha_completado: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevaActividadHistorial {
    pub historial_curso_id: i32,
    pub actividad_id: i32,
    pub calificacion: Option<f64>,
    pub completado: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActualizarActividadHistorial {
    pub calificacion: Option<f64>,
    pub completado: Option<bool>,
}
