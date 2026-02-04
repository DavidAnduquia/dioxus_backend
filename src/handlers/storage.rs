use axum::{body::Bytes, extract::Multipart, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    middleware::auth::AuthUser, services::storage_service::StorageService, utils::errors::AppError,
};

#[derive(Serialize)]
pub struct UploadResponse {
    pub success: bool,
    pub message: String,
    pub upload_url: String,
    pub file_key: String,
    pub download_url: Option<String>,
}

#[derive(Serialize)]
pub struct PresignedUrlResponse {
    pub upload_url: String,
    pub download_url: String,
    pub file_key: String,
}

#[derive(Deserialize)]
pub struct GenerateUrlRequest {
    pub file_name: String,
    pub content_type: String,
}

/// Genera una URL pre-firmada para subir archivos
pub async fn generate_upload_url(
    _auth_user: AuthUser,
    Json(request): Json<GenerateUrlRequest>,
) -> Result<Json<PresignedUrlResponse>, AppError> {
    let storage = StorageService::new().await.map_err(|e| {
        AppError::InternalServerError(format!("Failed to initialize storage: {}", e).into())
    })?;
    tracing::info!("✅ [BACKEND] StorageService inicializado correctamente");
    // Validar tamaño del nombre del archivo (máximo 255 caracteres)
    if request.file_name.len() > 255 {
        return Err(AppError::BadRequest("File name too long".into()));
    }

    // Validar tipo de contenido
    if request.content_type.is_empty() {
        return Err(AppError::BadRequest("Content type is required".into()));
    }

    // Generar un nombre único para el archivo para evitar colisiones
    let file_extension = get_file_extension(&request.file_name);
    let unique_file_key = format!("uploads/{}.{}", Uuid::new_v4(), file_extension);

    // Generar URLs pre-firmadas
    let upload_url = storage
        .generate_presigned_url(&unique_file_key, &request.content_type)
        .await?;
    let download_url = storage.generate_download_url(&unique_file_key).await?;

    // Crear la respuesta
    let response = PresignedUrlResponse {
        upload_url: upload_url.clone(),
        download_url: download_url.clone(),
        file_key: unique_file_key.clone(),
    };

    // Log detallado de la respuesta
    tracing::info!(
        "📤 [BACKEND] Enviando respuesta de subida: {}",
        serde_json::to_string_pretty(&response)
            .unwrap_or_else(|_| "Error al serializar la respuesta".into())
    );

    Ok(Json(response))
}

/// Sube un archivo directamente al servidor (método alternativo)
pub async fn upload_file_direct(
    _auth_user: AuthUser,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, AppError> {
    tracing::info!("📤 [BACKEND] Nueva petición de file upload recibida");

    let storage = StorageService::new().await.map_err(|e| {
        tracing::error!("❌ [BACKEND] Error inicializando StorageService: {}", e);
        AppError::InternalServerError(format!("Failed to initialize storage: {}", e).into())
    })?;

    tracing::info!("✅ [BACKEND] StorageService inicializado correctamente");

    // Campos que esperamos del multipart: "file" (blob) y "path" (string opcional)
    let mut file_name: Option<String> = None;
    let mut content_type: Option<String> = None;
    let mut data: Option<Bytes> = None;
    let mut logical_path: Option<String> = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let field_name = field.name().map(|s| s.to_string());

        match field_name.as_deref() {
            Some("file") | None => {
                // Campo de archivo (compatibilidad con clientes que no envían name="file")
                let fname = field
                    .file_name()
                    .ok_or_else(|| {
                        tracing::warn!("⚠️ [BACKEND] No se proporcionó nombre de archivo");
                        AppError::BadRequest("No filename provided".into())
                    })?
                    .to_string();

                let ctype = field
                    .content_type()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| {
                        tracing::info!(
                            "📝 [BACKEND] Usando content-type por defecto: application/octet-stream"
                        );
                        "application/octet-stream".into()
                    });

                tracing::info!(
                    "📄 [BACKEND] Procesando archivo: {} (tipo: {})",
                    fname,
                    ctype
                );

                let bytes = field.bytes().await.map_err(|e| {
                    tracing::error!("❌ [BACKEND] Error leyendo bytes del archivo: {}", e);
                    AppError::MultipartField(format!("Failed to read field bytes: {}", e))
                })?;

                tracing::info!(
                    "📊 [BACKEND] Archivo leído correctamente. Tamaño: {} bytes",
                    bytes.len()
                );

                file_name = Some(fname);
                content_type = Some(ctype);
                data = Some(bytes);
            }
            Some("path") => {
                // Campo de ruta lógica enviada por el frontend
                let text = field.text().await.map_err(|e| {
                    tracing::error!("❌ [BACKEND] Error leyendo campo 'path': {}", e);
                    AppError::MultipartField(format!("Failed to read path field: {}", e))
                })?;

                let cleaned = text.trim().trim_matches('/').to_string();
                if cleaned.is_empty() {
                    tracing::warn!("⚠️ [BACKEND] Campo 'path' vacío, se usará solo 'uploads/'");
                } else {
                    tracing::info!("📁 [BACKEND] Path lógico recibido: {}", cleaned);
                    logical_path = Some(cleaned);
                }
            }
            Some(other) => {
                tracing::debug!("ℹ️ [BACKEND] Campo multipart ignorado: {}", other);
            }
        }
    }

    let file_name = file_name.ok_or_else(|| {
        tracing::warn!("⚠️ [BACKEND] No se encontró campo de archivo en multipart");
        AppError::BadRequest("No file field found in request".into())
    })?;

    let content_type = content_type.unwrap_or_else(|| "application/octet-stream".into());
    let data = data.ok_or_else(|| {
        tracing::warn!("⚠️ [BACKEND] No se pudieron leer los bytes del archivo");
        AppError::BadRequest("No file data found in request".into())
    })?;

    // Generar un nombre único para el archivo y construir la key completa
    let file_extension = get_file_extension(&file_name);
    let unique_id = Uuid::new_v4();

    let base_prefix = "uploads";
    let unique_file_key = if let Some(path) = logical_path {
        format!("{}/{}/{}.{}", base_prefix, path, unique_id, file_extension)
    } else {
        format!("{}/{}.{}", base_prefix, unique_id, file_extension)
    };

    tracing::info!(
        "🔄 [BACKEND] Subiendo archivo a R2 con key: {}",
        unique_file_key
    );

    // Subir archivo
    let _uploaded_file = storage
        .upload_file(unique_file_key.clone(), content_type, data)
        .await?;

    tracing::info!("✅ [BACKEND] Archivo subido exitosamente a R2");

    // Generar URL de descarga
    let download_url = storage.generate_download_url(&unique_file_key).await?;

    tracing::info!("🔗 [BACKEND] URL de descarga generada: {}", download_url);

    // Crear la respuesta
    let response = UploadResponse {
        success: true,
        message: "Archivo subido exitosamente".to_string(),
        upload_url: "".to_string(), // No aplica en subida directa
        file_key: unique_file_key,
        download_url: Some(download_url),
    };

    // Log detallado de la respuesta
    tracing::info!(
        "📤 [BACKEND] Respuesta JSON enviada: {}",
        serde_json::to_string_pretty(&response)
            .unwrap_or_else(|_| "Error al serializar la respuesta".into())
    );

    Ok(Json(response))
}

/// Descarga un archivo sirviendo como proxy para Cloudflare R2
pub async fn download_file(
    _auth_user: AuthUser,
    axum::extract::Path(file_key): axum::extract::Path<String>,
) -> Result<impl axum::response::IntoResponse, AppError> {
    let storage = StorageService::new().await.map_err(|e| {
        AppError::InternalServerError(format!("Failed to initialize storage: {}", e).into())
    })?;

    // Validar que el archivo esté en la carpeta uploads para seguridad
    if !file_key.starts_with("uploads/") {
        return Err(AppError::BadRequest("Invalid file key".into()));
    }

    // Generar URL de descarga pre-firmada
    let download_url = storage.generate_download_url(&file_key).await?;

    // Redirigir a la URL de Cloudflare R2
    Ok(axum::response::Redirect::temporary(&download_url))
}

/// Elimina un archivo
pub async fn delete_file(
    _auth_user: AuthUser,
    axum::extract::Path(file_key): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let storage = StorageService::new().await.map_err(|e| {
        AppError::InternalServerError(format!("Failed to initialize storage: {}", e).into())
    })?;

    // Validar que el archivo esté en la carpeta uploads para seguridad
    if !file_key.starts_with("uploads/") {
        return Err(AppError::BadRequest("Invalid file key".into()));
    }

    storage.delete_file(&file_key).await?;

    Ok(Json(serde_json::json!({
        "message": "File deleted successfully",
        "file_key": file_key
    })))
}

/// Función auxiliar para obtener la extensión del archivo
fn get_file_extension(file_name: &str) -> &str {
    std::path::Path::new(file_name)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("bin")
}
