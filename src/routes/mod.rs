use axum::{
    extract::{Form, State},
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use utoipa::OpenApi;

use crate::{handlers, models::AppState};

pub mod roles;
pub mod usuarios;
pub mod area_conocimiento;
pub mod curso;
pub mod examen;
pub mod matricula;
pub mod modulo;

#[derive(Deserialize)]
#[allow(dead_code)]
struct OAuth2TokenRequest {
    grant_type: String,
    username: String,
    password: String,
    #[serde(default)]
    scope: String,
    #[serde(default)]
    client_id: String,
    #[serde(default)]
    client_secret: String,
}

#[derive(Serialize)]
struct OAuth2TokenResponse {
    access_token: String,
    token_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_in: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<String>,
}

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(handlers::health::health_check))
        .route("/ready", get(handlers::health::readiness_check))
        .route("/live", get(handlers::health::liveness_check))
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/login", post(handlers::auth::login))
        .route("/auth/token", post(oauth2_token_endpoint))  // Endpoint OAuth2
        .route("/users/me", get(handlers::users::get_current_user))
        .route("/ws", get(handlers::socket_manager::websocket_handler))  // WebSocket endpoint
        // .route("/posts", get(handlers::posts::get_posts))
        // .route("/posts", post(handlers::posts::create_post))
        // .route("/posts/:id", get(handlers::posts::get_post))
        // .route("/posts/:id", put(handlers::posts::update_post))
        // .route("/posts/:id", delete(handlers::posts::delete_post))
}

pub fn create_app() -> Router<AppState> {
    Router::new()
        .merge(create_routes())
        .merge(roles::roles_routes())  // Agregar rutas de roles
        .merge(usuarios::usuarios_routes()) // Agregar rutas de usuarios
        .merge(area_conocimiento::area_conocimiento_routes())
        .merge(curso::curso_routes())
        .merge(examen::examen_routes())
        .merge(matricula::matricula_routes())
        .merge(modulo::modulo_routes())
        .route("/api-docs/openapi.json", get(serve_openapi_spec))
        .route("/swagger-ui", get(serve_swagger_ui))
        .route("/swagger-ui/", get(serve_swagger_ui))
        .route("/swagger-ui/index.html", get(serve_swagger_ui))
        .route("/swagger-ui/oauth2-redirect.html", get(serve_oauth2_redirect))
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Rust API Backend",
        description = "API backend con autenticación JWT y documentación automática",
        version = "1.0.0"
    ),
    servers(
        (url = "http://localhost:3000", description = "Local development server"),
        (url = "https://api.example.com", description = "Production server"),
        (url = "https://staging-api.example.com", description = "Staging server")
    ),
    paths(
        handlers::auth::register,
        handlers::auth::login,
        handlers::users::get_current_user
    ),
    components(
        schemas(
            crate::models::CreateUserRequest,
            crate::models::LoginRequest,
            crate::models::AuthResponse,
            crate::models::UserResponse
        )
    )
)]
pub struct ApiDoc;


async fn oauth2_token_endpoint(
    State(state): State<AppState>,
    Form(form): Form<OAuth2TokenRequest>,
) -> Result<Json<OAuth2TokenResponse>, Json<Value>> {
    use bcrypt::verify;
    use jsonwebtoken::{encode, Header};
    use crate::models::{Claims, User};

    // Validar grant_type
    if form.grant_type != "password" {
        return Err(Json(json!({
            "error": "unsupported_grant_type",
            "error_description": "Only 'password' grant type is supported"
        })));
    }

    // Obtener conexión a la base de datos
    let db = state.get_db().map_err(|_| {
        Json(json!({
            "error": "server_error",
            "error_description": "Database connection unavailable"
        }))
    })?;

    // Buscar usuario por email
    let user = sqlx::query_as::<_, User>(
        "SELECT id, email, password_hash, name, created_at, updated_at FROM users WHERE email = $1"
    )
    .bind(&form.username)
    .fetch_optional(db)
    .await
    .map_err(|e| {
        tracing::error!("Database error in OAuth2 token endpoint: {:?}", e);
        Json(json!({
            "error": "server_error",
            "error_description": "Database error"
        }))
    })?;

    let user = match user {
        Some(user) => user,
        None => {
            return Err(Json(json!({
                "error": "invalid_grant",
                "error_description": "Invalid username or password"
            })));
        }
    };

    // Verificar password
    let password_valid = verify(&form.password, &user.password_hash)
        .map_err(|e| {
            tracing::error!("Password verification error: {:?}", e);
            Json(json!({
                "error": "server_error",
                "error_description": "Password verification failed"
            }))
        })?;

    if !password_valid {
        return Err(Json(json!({
            "error": "invalid_grant",
            "error_description": "Invalid username or password"
        })));
    }

    // Generar JWT token
    let now = chrono::Utc::now();
    let exp = now + chrono::Duration::hours(24);

    let claims = Claims {
        sub: user.id.to_string(),
        email: user.email.clone(),
        exp: exp.timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &state.jwt_encoding_key,
    )
    .map_err(|e| {
        tracing::error!("Token generation error: {:?}", e);
        Json(json!({
            "error": "server_error",
            "error_description": "Token generation failed"
        }))
    })?;

    Ok(Json(OAuth2TokenResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
        expires_in: Some(86400),
        refresh_token: None,
        scope: Some("read write".to_string()),
    }))
}

async fn serve_openapi_spec() -> String {
    use utoipa::openapi::security::{SecurityScheme, OAuth2, Flow, Password, Scopes};

    let mut openapi = ApiDoc::openapi();

    // Agregar esquemas de seguridad de manera más eficiente
    let components = openapi.components.get_or_insert_with(Default::default);

    // Bearer Token para uso manual
    components.add_security_scheme(
        "bearer_auth",
        SecurityScheme::Http(
            utoipa::openapi::security::Http::new(utoipa::openapi::security::HttpAuthScheme::Bearer)
        ),
    );

    // OAuth2 Password Flow para login automático
    let password_flow = Password::new("/auth/token", Scopes::new());
    let oauth2 = OAuth2::new([Flow::Password(password_flow)]);

    components.add_security_scheme(
        "oauth2_password",
        SecurityScheme::OAuth2(oauth2)
    );

    // Agregar seguridad global
    let security = vec![
        utoipa::openapi::security::SecurityRequirement::new("bearer_auth", Vec::<String>::new()),
        utoipa::openapi::security::SecurityRequirement::new("oauth2_password", Vec::<String>::new())
    ];

    // Crear la estructura OpenAPI modificada de manera más eficiente
    openapi.components = Some(components.clone());
    openapi.security = Some(security);

    openapi.to_json().unwrap_or_else(|e| {
        tracing::error!("Failed to serialize OpenAPI spec: {}", e);
        "{}".to_string()
    })
}

async fn serve_swagger_ui() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Rust API Backend - Swagger UI</title>
    <link rel="stylesheet" type="text/css" href="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui.css" />
    <style>
        .swagger-ui .topbar { display: none !important; }
        .swagger-ui .information-container { margin-top: 0 !important; }
        body { margin: 0; padding: 20px; }
    </style>
</head>
<body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui-bundle.js"></script>
    <script>
        SwaggerUIBundle({
            url: '/api-docs/openapi.json',
            dom_id: '#swagger-ui',
            presets: [
                SwaggerUIBundle.presets.apis,
                SwaggerUIBundle.presets.standalone
            ],
            layout: "BaseLayout",
            deepLinking: true,
            showExtensions: false,
            showCommonExtensions: false,
            filter: false,
            persistAuthorization: true
        });
    </script>
</body>
</html>
"#)
}

async fn serve_oauth2_redirect() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <title>Swagger UI: OAuth2 Redirect</title>
</head>
<body>
    <script src="https://unpkg.com/swagger-ui-dist@5.9.0/oauth2-redirect.html"></script>
</body>
</html>
"#)
}

