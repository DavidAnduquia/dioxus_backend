use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    models::rol::Model as Rol,
    models::AppState,
    services::rol_service::RolService,
};

pub async fn get_rol(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<Option<Rol>>, String> {
    let db = state.get_db().map_err(|e| e.to_string())?;
    RolService::find_by_id(db, id)
        .await
        .map(Json)
        .map_err(|e| e.to_string())
}

pub async fn list_roles(
    State(state): State<AppState>,
) -> Result<Json<Vec<Rol>>, String> {
    let db = state.get_db().map_err(|e| e.to_string())?;
    RolService::obtener_roles(db)
        .await
        .map(Json)
        .map_err(|e| e.to_string())
}

pub async fn create_rol(
    State(state): State<AppState>,
    Json(payload): Json<String>, // Solo nombre
) -> Result<Json<Rol>, String> {
    let db = state.get_db().map_err(|e| e.to_string())?;
    RolService::create(db, payload)
        .await
        .map(Json)
        .map_err(|e| e.to_string())
}

pub async fn update_rol(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(nombre): Json<String>,
) -> Result<Json<Rol>, String> {
    let db = state.get_db().map_err(|e| e.to_string())?;
    RolService::update(db, id, nombre)
        .await
        .map(Json)
        .map_err(|e| e.to_string())
}

pub async fn delete_rol(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<String>, String> {
    let db = state.get_db().map_err(|e| e.to_string())?;
    RolService::delete(db, id)
        .await
        .map(|_| Json("Rol eliminado".into()))
        .map_err(|e| e.to_string())
}