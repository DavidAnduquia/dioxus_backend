use axum::{
    extract::State,
    response::Json,
};

use crate::{
    middleware::auth::AuthUser,
    models::{ApiResponse, AppState, User, UserResponse},
    utils::errors::AppError,
};

#[utoipa::path(
    get,
    path = "/users/me",
    responses(
        (status = 200, description = "Usuario actual", body = ApiResponse<UserResponse>),
        (status = 401, description = "No autorizado"),
        (status = 500, description = "Error interno del servidor")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_current_user(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<ApiResponse<UserResponse>>, AppError> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(auth_user.user_id)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(ApiResponse::success(user.into())))
}
