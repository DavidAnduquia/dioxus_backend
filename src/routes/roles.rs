use axum::{
    routing::get,
    Router,
};

use crate::{handlers::roles, models::AppState};

pub fn roles_routes() -> Router<AppState> {
    Router::new()
        .route("/api/roles", get(roles::list_roles).post(roles::create_rol))
        .route("/api/roles/:id", 
            get(roles::get_rol)
            .put(roles::update_rol)
            .delete(roles::delete_rol)
        )
}