use once_cell::sync::OnceCell;
use sea_orm::ModelTrait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};
use std::sync::Arc;

use crate::models::rol::{ActiveModel as RolActiveModel, Entity as RolEntity, Model as RolModel};

static ROL_SERVICE: OnceCell<Arc<RolService>> = OnceCell::new();

#[derive(Debug, Clone)]
pub struct RolService {
    conn: DatabaseConnection,
}

impl RolService {
    /// Obtiene la instancia global del servicio, inicializándola si es necesario
    pub fn global(conn: &DatabaseConnection) -> &'static Arc<Self> {
        ROL_SERVICE.get_or_init(|| Arc::new(Self { conn: conn.clone() }))
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<RolModel>, DbErr> {
        RolEntity::find_by_id(id).one(&self.conn).await
    }

    pub async fn obtener_roles(&self) -> Result<Vec<RolModel>, DbErr> {
        RolEntity::find().all(&self.conn).await
    }

    pub async fn create(&self, nombre: String) -> Result<RolModel, DbErr> {
        let new_rol = RolActiveModel {
            nombre: Set(nombre),
            ..Default::default()
        };
        new_rol.insert(&self.conn).await
    }

    pub async fn update(&self, id: i32, nombre: String) -> Result<RolModel, DbErr> {
        let rol = RolEntity::find_by_id(id)
            .one(&self.conn)
            .await?
            .ok_or(DbErr::RecordNotFound("Rol no encontrado".into()))?;

        let mut rol: RolActiveModel = rol.into();
        rol.nombre = Set(nombre);
        rol.update(&self.conn).await
    }

    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let rol = RolEntity::find_by_id(id)
            .one(&self.conn)
            .await?
            .ok_or(DbErr::RecordNotFound("Rol no encontrado".into()))?;

        rol.delete(&self.conn).await?;
        Ok(1) // Asumiendo que siempre se elimina 1 registro
    }
}
