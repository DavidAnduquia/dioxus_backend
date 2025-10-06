use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usuario {
    pub id: i64,
    pub nombre: String,
    pub documento_nit: Option<String>,
    pub correo: String,
    #[serde(skip_serializing)]  // Seguridad: no exponer en responses
    pub contrasena: String,
    pub rol_id: i32,
    pub estado: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct UsuarioConRol {
    #[serde(flatten)]
    pub usuario: Usuario,
    pub rol: Rol,  // Rol debe ser definido en mod.rs
}