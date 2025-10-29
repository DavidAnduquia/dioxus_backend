use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "webinar_modulos")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub webinar_id: i32,
    pub titulo: String,
    pub descripcion: Option<String>,
    pub orden: i32,
    pub tipo_contenido: String, // 'video', 'presentacion', 'actividad', 'quiz'
    pub contenido_url: Option<String>,
    pub duracion_estimada: Option<i32>, // en minutos
    pub obligatorio: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Webinar,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Webinar => Entity::belongs_to(super::webinar::Entity)
                .from(Column::WebinarId)
                .to(super::webinar::Column::Id)
                .into(),
        }
    }
}

impl Related<super::webinar::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Webinar.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
