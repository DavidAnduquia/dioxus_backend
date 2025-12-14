use aws_sdk_s3::{
    config::Credentials, presigning::PresigningConfig, primitives::ByteStream, Client,
};
use axum::body::Bytes;
use std::time::Duration;
use thiserror::Error;

const MAX_FILE_SIZE: u64 = 20 * 1024 * 1024; // 20MB

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("File too large. Max size is 20MB")]
    FileTooLarge,
    #[error("Invalid file type")]
    InvalidFileType,
    #[error("Storage error: {0}")]
    StorageError(String),
}

impl From<aws_sdk_s3::error::SdkError<aws_sdk_s3::operation::put_object::PutObjectError>>
    for StorageError
{
    fn from(
        err: aws_sdk_s3::error::SdkError<aws_sdk_s3::operation::put_object::PutObjectError>,
    ) -> Self {
        StorageError::StorageError(format!("S3 put object error: {}", err))
    }
}

impl From<aws_sdk_s3::error::SdkError<aws_sdk_s3::operation::get_object::GetObjectError>>
    for StorageError
{
    fn from(
        err: aws_sdk_s3::error::SdkError<aws_sdk_s3::operation::get_object::GetObjectError>,
    ) -> Self {
        StorageError::StorageError(format!("S3 get object error: {}", err))
    }
}

impl From<aws_sdk_s3::error::SdkError<aws_sdk_s3::operation::delete_object::DeleteObjectError>>
    for StorageError
{
    fn from(
        err: aws_sdk_s3::error::SdkError<aws_sdk_s3::operation::delete_object::DeleteObjectError>,
    ) -> Self {
        StorageError::StorageError(format!("S3 delete object error: {}", err))
    }
}

impl From<aws_sdk_s3::presigning::PresigningConfigError> for StorageError {
    fn from(err: aws_sdk_s3::presigning::PresigningConfigError) -> Self {
        StorageError::StorageError(format!("Presigning config error: {}", err))
    }
}

#[derive(Clone)]
pub struct StorageService {
    client: Client,
    bucket: String,
}

impl StorageService {
    /// Crea un nuevo servicio de almacenamiento con configuración de R2
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        tracing::info!("🔧 [STORAGE] Inicializando StorageService...");

        // Configuración hardcoded para coincidir con el frontend React
        let config = aws_config::from_env()
            .region("auto") // Cloudflare R2 uses 'auto' region
            .credentials_provider(Credentials::new(
                "94693ac7f6b38fa08f74fc9833f14fe2", // NEW Access Key ID
                "ec37cebc510f3de5d59992e497d9870c1d6c59295d2a1c7e891d630a79ec17dc", // NEW Secret Access Key
                None,
                None,
                "aulatrix-r2-token-new",
            ))
            .load()
            .await;

        tracing::info!("✅ [STORAGE] Configuración AWS cargada");

        let s3_config = aws_sdk_s3::config::Builder::from(&config)
            .endpoint_url("https://fdf4a26651358f5e1f66f4af2b7cb4cb.r2.cloudflarestorage.com")
            .force_path_style(true) // Required for Cloudflare R2
            .build();

        tracing::info!("✅ [STORAGE] Configuración S3 creada con endpoint: https://fdf4a26651358f5e1f66f4af2b7cb4cb.r2.cloudflarestorage.com/aulatrix-bucket");

        let client = aws_sdk_s3::Client::from_conf(s3_config);
        let bucket = "aulatrix-bucket".to_string(); // Bucket name from React frontend

        tracing::info!("✅ [STORAGE] Cliente S3 creado, bucket: {}", bucket);
        tracing::info!("🚀 [STORAGE] StorageService inicializado correctamente");

        Ok(Self { client, bucket })
    }

    /// Genera una URL pre-firmada para subir archivos
    pub async fn generate_presigned_url(
        &self,
        file_name: &str,
        content_type: &str,
    ) -> Result<String, StorageError> {
        let expires_in = Duration::from_secs(3600); // 1 hora de validez

        let presigned_request = self
            .client
            .put_object()
            .bucket(&self.bucket)
            .key(file_name)
            .content_type(content_type)
            .presigned(PresigningConfig::expires_in(expires_in)?)
            .await?;

        Ok(presigned_request.uri().to_string())
    }

    /// Sube un archivo directamente a R2
    pub async fn upload_file(
        &self,
        file_name: String,
        content_type: String,
        bytes: Bytes,
    ) -> Result<String, StorageError> {
        tracing::info!(
            "🔄 [STORAGE] Iniciando subida de archivo: {} (tipo: {}, tamaño: {} bytes)",
            file_name,
            content_type,
            bytes.len()
        );
        tracing::info!("📦 [STORAGE] Bucket configurado: {}", self.bucket);

        // Validar tamaño del archivo
        if bytes.len() as u64 > MAX_FILE_SIZE {
            tracing::error!(
                "❌ [STORAGE] Archivo demasiado grande: {} bytes (máximo: {} bytes)",
                bytes.len(),
                MAX_FILE_SIZE
            );
            return Err(StorageError::FileTooLarge);
        }
        tracing::info!("✅ [STORAGE] Validación de tamaño: OK");

        // Validar tipo de archivo (opcional)
        if !is_allowed_content_type(&content_type) {
            tracing::error!(
                "❌ [STORAGE] Tipo de archivo no permitido: {}",
                content_type
            );
            return Err(StorageError::InvalidFileType);
        }
        tracing::info!("✅ [STORAGE] Validación de tipo de archivo: OK");

        let body = ByteStream::from(bytes);
        tracing::info!("📋 [STORAGE] ByteStream creado correctamente");

        tracing::info!("🚀 [STORAGE] Enviando petición PUT a R2...");
        let result = self
            .client
            .put_object()
            .bucket(&self.bucket)
            .key(&file_name)
            .content_type(content_type)
            .body(body)
            .send()
            .await;

        match result {
            Ok(_) => {
                tracing::info!(
                    "✅ [STORAGE] Archivo subido exitosamente a R2: {}",
                    file_name
                );
                Ok(file_name)
            }
            Err(e) => {
                tracing::error!("❌ [STORAGE] Error al subir archivo a R2: {:?}", e);
                tracing::error!("❌ [STORAGE] Detalles del error: {}", e.to_string());
                Err(StorageError::StorageError(e.to_string()))
            }
        }
    }

    /// Obtiene una URL pre-firmada para descargar archivos
    pub async fn generate_download_url(&self, file_name: &str) -> Result<String, StorageError> {
        let expires_in = Duration::from_secs(3600); // 1 hora de validez

        let presigned_request = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(file_name)
            .presigned(PresigningConfig::expires_in(expires_in)?)
            .await?;

        Ok(presigned_request.uri().to_string())
    }

    /// Elimina un archivo de R2
    pub async fn delete_file(&self, file_name: &str) -> Result<(), StorageError> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(file_name)
            .send()
            .await
            .map_err(|e| StorageError::StorageError(e.to_string()))?;

        Ok(())
    }
}

/// Función auxiliar para validar tipos de contenido permitidos
fn is_allowed_content_type(content_type: &str) -> bool {
    // Lista blanca de tipos MIME permitidos
    let allowed_types = [
        "image/jpeg",
        "image/jpg",
        "image/png",
        "image/gif",
        "application/pdf",
        "text/plain",
        "application/msword",
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "application/vnd.ms-excel",
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "application/vnd.ms-powerpoint",
        "application/vnd.openxmlformats-officedocument.presentationml.presentation",
    ];

    allowed_types.contains(&content_type)
}
