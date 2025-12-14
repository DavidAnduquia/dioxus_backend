use chrono::{DateTime, NaiveDate, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "cursos")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub nombre: String,
    pub descripcion: String,
    pub fecha_inicio: NaiveDate,
    pub fecha_fin: NaiveDate,
    pub prerequisito: Option<String>,
    pub coordinador_id: i32,
    pub creado_en: DateTime<Utc>,
    pub plantilla_base_id: Option<i32>,
    pub semestre: Option<i32>,
    pub periodo: String,
    pub anio_pensum: i32,
    pub area_conocimiento_id: i32,
    pub fecha_eliminacion: Option<DateTime<Utc>>,
    pub fecha_actualizacion: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::area_conocimiento::Entity",
        from = "Column::AreaConocimientoId",
        to = "super::area_conocimiento::Column::Id"
    )]
    AreaConocimiento,
    #[sea_orm(
        belongs_to = "super::usuario::Entity",
        from = "Column::CoordinadorId",
        to = "super::usuario::Column::Id"
    )]
    Usuario,
}

impl Related<super::area_conocimiento::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AreaConocimiento.def()
    }
}

impl Related<super::usuario::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Usuario.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevoCurso {
    pub nombre: String,
    pub descripcion: String,
    pub fecha_inicio: NaiveDate,
    pub fecha_fin: NaiveDate,
    pub prerequisito: Option<String>,
    pub coordinador_id: i32,
    pub semestre: Option<i32>,
    pub periodo: String,
    pub anio_pensum: i32,
    pub area_conocimiento_id: i32,
    pub plantilla_base_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActualizarCurso {
    pub nombre: Option<String>,
    pub descripcion: Option<String>,
    pub fecha_inicio: Option<NaiveDate>,
    pub fecha_fin: Option<NaiveDate>,
    pub prerequisito: Option<String>,
    pub coordinador_id: Option<i32>,
    pub semestre: Option<i32>,
    pub periodo: Option<String>,
    pub anio_pensum: Option<i32>,
    pub area_conocimiento_id: Option<i32>,
    pub plantilla_base_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursoDetallado {
    pub curso: Model,
    pub area: Option<super::area_conocimiento::Model>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnidadAula {
    pub id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub orden: Option<i32>,
    pub tema_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemaAula {
    pub id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub orden: Option<i32>,
    pub unidades: Vec<UnidadAula>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AulaCurso {
    pub curso: Model,
    pub temas: Vec<TemaAula>,
}
