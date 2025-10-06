use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use crate::database::migrator::AutoMigrate;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, FromRow)]
#[sea_orm(table_name = "roles", schema_name = "rustdema")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub nombre: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// Implementación del trait genérico
impl AutoMigrate for Entity {
    fn entity_name() -> &'static str {
        "roles"
    }

    fn seed_data() -> Option<Vec<String>> {
        Some(vec![
            r#"
            INSERT INTO rustdema.roles (nombre) 
            VALUES ('admin'), ('user'), ('moderator')
            ON CONFLICT (nombre) DO NOTHING
            "#.to_string()
        ])
    }
}