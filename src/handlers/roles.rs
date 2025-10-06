use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    entities::rol::Model as Rol,
    models::AppState,
    services::rol_service::RolService,
};

pub async fn get_rol(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<Option<Rol>>, String> {
    RolService::find_by_id(&state.db, id)
        .await
        .map(Json)
        .map_err(|e| e.to_string())
}

pub async fn list_roles(
    State(state): State<AppState>,
) -> Result<Json<Vec<Rol>>, String> {
    RolService::find_all(&state.db)
        .await
        .map(Json)
        .map_err(|e| e.to_string())
}

pub async fn create_rol(
    State(state): State<AppState>,
    Json(payload): Json<String>, // Solo nombre
) -> Result<Json<Rol>, String> {
    RolService::create(&state.db, payload)
        .await
        .map(Json)
        .map_err(|e| e.to_string())
}

pub async fn update_rol(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(nombre): Json<String>,
) -> Result<Json<Rol>, String> {
    RolService::update(&state.db, id, nombre)
        .await
        .map(Json)
        .map_err(|e| e.to_string())
}

pub async fn delete_rol(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<String>, String> {
    RolService::delete(&state.db, id)
        .await
        .map(|_| Json("Rol eliminado".into()))
        .map_err(|e| e.to_string())
}