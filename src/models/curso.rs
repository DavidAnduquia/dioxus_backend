use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "cursos")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub nombre: String,
    pub codigo: String,
    pub descripcion: Option<String>,
    pub creditos: i32,
    pub horas_teoricas: i32,
    pub horas_practicas: i32,
    pub area_conocimiento_id: i32,
    pub estado: bool,
    pub periodo: Option<String>,
    pub fecha_inicio: Option<DateTime<Utc>>,
    pub fecha_fin: Option<DateTime<Utc>>,
    pub anio_pensum: Option<i32>,
    pub coordinador_id: Option<i64>,
    pub plantilla_base_id: Option<i32>,
    pub prerequisito: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::area_conocimiento::Entity",
        from = "Column::AreaConocimientoId",
        to = "super::area_conocimiento::Column::Id"
    )]
    AreaConocimiento,
}

impl Related<super::area_conocimiento::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AreaConocimiento.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
