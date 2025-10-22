use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "cursos")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub nombre: String,
    pub descripcion: String,
    pub fecha_inicio: Date,
    pub fecha_fin: Date,
    pub prerequisito: Option<String>,
    pub coordinador_id: i32,
    pub creado_en: Option<DateTime<Utc>>,
    pub plantilla_base_id: Option<i32>,
    pub semestre: Option<i32>,
    pub periodo: String,
    pub anio_pensum: i32,
    pub area_conocimiento_id: i32,
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
