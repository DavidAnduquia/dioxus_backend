use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "examenes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub curso_id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub fecha_inicio: DateTime<Utc>,
    pub fecha_fin: DateTime<Utc>,
    pub duracion_minutos: i32,
    pub intentos_permitidos: i32,
    pub mostrar_resultados: bool,
    pub estado: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevoExamen {
    pub curso_id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub fecha_inicio: DateTime<Utc>,
    pub fecha_fin: DateTime<Utc>,
    pub duracion_minutos: i32,
    pub intentos_permitidos: i32,
    pub mostrar_resultados: bool,
    pub estado: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActualizarExamen {
    pub nombre: Option<String>,
    pub descripcion: Option<String>,
    pub fecha_inicio: Option<DateTime<Utc>>,
    pub fecha_fin: Option<DateTime<Utc>>,
    pub duracion_minutos: Option<i32>,
    pub intentos_permitidos: Option<i32>,
    pub mostrar_resultados: Option<bool>,
    pub estado: Option<String>,
}
