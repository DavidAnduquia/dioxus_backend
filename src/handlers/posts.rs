use axum::{
    extract::{Path, Query, State},
    response::Json,
};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{
    middleware::auth::AuthUser,
    models::{ApiResponse, AppState, CreatePostRequest, Post, UpdatePostRequest},
    utils::errors::AppError,
};

#[derive(Debug, Deserialize)]
pub struct GetPostsQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub published: Option<bool>,
}

pub async fn get_posts(
    State(state): State<AppState>,
    Query(query): Query<GetPostsQuery>,
) -> Result<Json<ApiResponse<Vec<Post>>>, AppError> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10).min(100); // Max 100 items per page
    let offset = (page - 1) * limit;

    let mut sql = "SELECT * FROM posts".to_string();
    let mut conditions = Vec::new();

    if let Some(published) = query.published {
        conditions.push(format!("published = {}", published));
    }

    if !conditions.is_empty() {
        sql.push_str(&format!(" WHERE {}", conditions.join(" AND ")));
    }

    sql.push_str(" ORDER BY created_at DESC LIMIT $1 OFFSET $2");

    let posts = sqlx::query_as::<_, Post>(&sql)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&state.db)
        .await?;

    Ok(Json(ApiResponse::success(posts)))
}

pub async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Post>>, AppError> {
    let post = sqlx::query_as::<_, Post>(
        "SELECT * FROM posts WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

    Ok(Json(ApiResponse::success(post)))
}

pub async fn create_post(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreatePostRequest>,
) -> Result<Json<ApiResponse<Post>>, AppError> {
    // Validate input
    payload.validate()?;

    let post = sqlx::query_as::<_, Post>(
        r#"
        INSERT INTO posts (title, content, author_id, published)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(auth_user.user_id)
    .bind(payload.published.unwrap_or(false))
    .fetch_one(&state.db)
    .await?;

    Ok(Json(ApiResponse::success(post)))
}

pub async fn update_post(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePostRequest>,
) -> Result<Json<ApiResponse<Post>>, AppError> {
    // Validate input
    payload.validate()?;

    // Check if post exists and user owns it
    let existing_post = sqlx::query_as::<_, Post>(
        "SELECT * FROM posts WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

    if existing_post.author_id != auth_user.user_id {
        return Err(AppError::Forbidden("You can only update your own posts".to_string()));
    }

    // Simple approach - update fields individually if provided
    let mut post = existing_post;

    if let Some(title) = payload.title {
        post = sqlx::query_as::<_, Post>(
            "UPDATE posts SET title = $1, updated_at = NOW() WHERE id = $2 RETURNING *"
        )
        .bind(&title)
        .bind(id)
        .fetch_one(&state.db)
        .await?;
    }

    if let Some(content) = payload.content {
        post = sqlx::query_as::<_, Post>(
            "UPDATE posts SET content = $1, updated_at = NOW() WHERE id = $2 RETURNING *"
        )
        .bind(&content)
        .bind(id)
        .fetch_one(&state.db)
        .await?;
    }

    if let Some(published) = payload.published {
        post = sqlx::query_as::<_, Post>(
            "UPDATE posts SET published = $1, updated_at = NOW() WHERE id = $2 RETURNING *"
        )
        .bind(published)
        .bind(id)
        .fetch_one(&state.db)
        .await?;
    }

    Ok(Json(ApiResponse::success(post)))
}

pub async fn delete_post(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    // Check if post exists and user owns it
    let existing_post = sqlx::query_as::<_, Post>(
        "SELECT * FROM posts WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

    if existing_post.author_id != auth_user.user_id {
        return Err(AppError::Forbidden("You can only delete your own posts".to_string()));
    }

    sqlx::query("DELETE FROM posts WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await?;

    Ok(Json(ApiResponse::success(())))
}
