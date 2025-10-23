use axum::{
    extract::State,
    response::Json,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use validator::Validate;

use crate::{
    models::{
        ApiResponse, AppState, AuthResponse, Claims, CreateUserRequest, LoginRequest, User,
    },
    utils::errors::AppError,
};

#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "Usuario registrado exitosamente", body = AuthResponse),
        (status = 400, description = "Datos inválidos o usuario ya existe"),
        (status = 500, description = "Error interno del servidor")
    )
)]
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, AppError> {
    // Validate input
    payload.validate()?;

    // Obtener conexión a la base de datos
    let db = state.get_db()?;

    // Check if user already exists
    let user_exists: Option<i32> = sqlx::query_scalar(
        "SELECT 1 FROM users WHERE email = $1"
    )
    .bind(&payload.email)
    .fetch_optional(db)
    .await?;

    if user_exists.is_some() {
        return Err(AppError::BadRequest("User already exists".into()));
    }

    // Hash password
    let password = payload.password.clone();
    let password_hash = tokio::task::spawn_blocking(move || hash(password, DEFAULT_COST))
        .await
        .map_err(|e| AppError::InternalServerError(format!("Task join error: {}", e).into()))??;

    // Create user
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (email, password_hash, name)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(&payload.email)
    .bind(&password_hash)
    .bind(&payload.name)
    .fetch_one(db)
    .await?;

    // Generate JWT token
    let token = generate_token(&user, state.jwt_encoding_key.as_ref())?;

    let response = AuthResponse {
        token,
        user: user.into(),
    };

    Ok(Json(ApiResponse::success(response)))
}

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login exitoso", body = AuthResponse),
        (status = 401, description = "Credenciales inválidas"),
        (status = 400, description = "Datos inválidos"),
        (status = 500, description = "Error interno del servidor")
    )
)]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, AppError> {
    // Validate input
    payload.validate()?;

    // Obtener conexión a la base de datos
    let db = state.get_db()?;

    // Find user by email
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&payload.email)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| AppError::Unauthorized("Invalid credentials".into()))?;

    // Verify password
    let password = payload.password.clone();
    let password_hash = user.password_hash.clone();
    let is_valid = tokio::task::spawn_blocking(move || verify(&password, &password_hash))
        .await
        .map_err(|e| AppError::InternalServerError(format!("Task join error: {}", e).into()))??;

    if !is_valid {
        return Err(AppError::Unauthorized("Invalid credentials".into()));
    }

    // Generate JWT token
    let token = generate_token(&user, state.jwt_encoding_key.as_ref())?;

    let response = AuthResponse {
        token,
        user: user.into(),
    };

    Ok(Json(ApiResponse::success(response)))
}

fn generate_token(user: &User, encoding_key: &jsonwebtoken::EncodingKey) -> Result<String, AppError> {
    use chrono::Utc;
    use jsonwebtoken::{encode, Header};

    let now = Utc::now();
    let exp = (now + chrono::Duration::hours(24)).timestamp() as usize;
    let iat = now.timestamp() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        email: user.email.clone(),
        exp,
        iat,
    };

    Ok(encode(&Header::default(), &claims, encoding_key)?)
}
