use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    middleware::auth::AuthUser,
    models::rol::Model as Rol,
    models::AppState,
    services::rol_service::RolService,
};

pub async fn get_rol(
    _auth_user: AuthUser,  // Validar JWT automáticamente
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<Option<Rol>>, String> {
    let db = state.db.as_ref().ok_or("DB no disponible".to_string())?;
    let conn = db.connection();
    let service = RolService::global(&conn);
    let role = service.find_by_id(id).await.map_err(|e| e.to_string())?;
    drop(conn); // Agregar esta línea para cerrar la conexión
    Ok(Json(role))
}

pub async fn list_roles(
    _auth_user: AuthUser,  // Validar JWT automáticamente
    State(state): State<AppState>,
) -> Result<Json<Vec<Rol>>, String> {
    let db = state.db.as_ref().ok_or("DB no disponible".to_string())?;
    let conn = db.connection();
    let service = RolService::global(&conn);
    let roles = service.obtener_roles().await.map_err(|e| e.to_string())?;
    drop(conn); // Agregar esta línea para cerrar la conexión
    Ok(Json(roles))
}

pub async fn create_role(
    _auth_user: AuthUser,  // Validar JWT automáticamente
    State(state): State<AppState>,
    Json(nombre): Json<String>,
) -> Result<Json<Rol>, String> {
    let db = state.db.as_ref().ok_or("DB no disponible".to_string())?;
    let conn = db.connection();
    let service = RolService::global(&conn);
    let new_role = service.create(nombre).await.map_err(|e| e.to_string())?;
    drop(conn); // Agregar esta línea para cerrar la conexión
    Ok(Json(new_role))
}

pub async fn update_role(
    _auth_user: AuthUser,  // Validar JWT automáticamente
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(nombre): Json<String>,
) -> Result<Json<Rol>, String> {
    let db = state.db.as_ref().ok_or("DB no disponible".to_string())?;
    let conn = db.connection();
    let service = RolService::global(&conn);
    let updated_role = service.update(id, nombre).await.map_err(|e| e.to_string())?;
    drop(conn); // Agregar esta línea para cerrar la conexión
    Ok(Json(updated_role))
}

pub async fn delete_role(
    _auth_user: AuthUser,  // Validar JWT automáticamente
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<String>, String> {
    let db = state.db.as_ref().ok_or("DB no disponible".to_string())?;
    let conn = db.connection();
    let service = RolService::global(&conn);
    service.delete(id).await.map_err(|e| e.to_string())?;
    drop(conn); // Agregar esta línea para cerrar la conexión
    Ok(Json("Rol eliminado".into()))
}