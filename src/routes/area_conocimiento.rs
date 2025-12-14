use axum::{
    routing::{get, patch},
    Router,
};

use crate::{handlers::area_conocimiento, models::AppState};

pub fn area_conocimiento_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/areas-conocimiento",
            get(area_conocimiento::listar_areas).post(area_conocimiento::crear_area),
        )
        .route(
            "/api/areas-conocimiento/activas",
            get(area_conocimiento::listar_areas_activas),
        )
        .route(
            "/api/areas-conocimiento/{id}",
            get(area_conocimiento::obtener_area)
                .put(area_conocimiento::actualizar_area)
                .delete(area_conocimiento::eliminar_area),
        )
        .route(
            "/api/areas-conocimiento/{id}/estado",
            patch(area_conocimiento::cambiar_estado),
        )
}
