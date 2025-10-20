use chrono::Utc;
use axum::extract::FromRef;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    Set, IntoActiveModel,
};
use tracing::instrument;

use crate::{
    database::DbExecutor,
    models::{
        rol,
        usuario::{self, Entity as Usuario, Model as UsuarioModel, NewUsuario, UpdateUsuario},
        AppState,
    },
    utils::errors::AppError,
};

// Validación básica de email sin dependencia extra
fn is_valid_email(email: &str) -> bool {
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 { return false; }
    let (local, domain) = (parts[0], parts[1]);
    !local.is_empty() && domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.')
}

#[derive(Debug, Clone)]
pub struct UsuarioService {
    db: DbExecutor,
}

impl UsuarioService {
    pub fn new(db: DbExecutor) -> Self {
        Self { db }
    }


    /// Obtiene una conexión del pool de manera eficiente
    async fn get_connection(&self) -> DatabaseConnection {
        self.db.connection()
    }
    
    // No necesitamos el método begin_transaction separado ya que usamos begin() directamente

    // Iniciar sesión
    #[instrument(skip(self))]
    pub async fn login_usuario(
        &self,
        identificador: &str,
        contrasena: &str,
    ) -> Result<UsuarioModel, AppError> {
        let db = self.get_connection().await;
        
        // Buscar usuario
        let usuario = Usuario::find()
            .filter(
                usuario::Column::DocumentoNit
                    .eq(identificador)
                    .or(usuario::Column::Correo.eq(identificador)),
            )
            .filter(usuario::Column::Contrasena.eq(contrasena))
            .one(&db)
            .await?
            .ok_or_else(|| AppError::NotFound("Credenciales inválidas".to_string()))?;

        // Actualizar última conexión
        let now = Utc::now();
        let mut usuario = usuario.into_active_model();
        usuario.fecha_actualizacion = Set(Some(now));
        usuario.fecha_ultima_conexion = Set(Some(now));
        let usuario = usuario.update(&db).await?;

        Ok(usuario)
    }

    // Cerrar sesión
    #[instrument(skip(self))]
    pub async fn logout_usuario(&self, id: i32) -> Result<UsuarioModel, AppError> {
        let db = self.get_connection().await;
        
        let usuario = Usuario::find_by_id(id)
            .one(&db)
            .await?
            .ok_or_else(|| AppError::NotFound("Usuario no encontrado".to_string()))?;

        // Actualizar última conexión
        let now = Utc::now();
        let mut usuario = usuario.into_active_model();
        usuario.fecha_actualizacion = Set(Some(now));
        usuario.fecha_ultima_conexion = Set(Some(now));
        let usuario = usuario.update(&db).await?;

        Ok(usuario)
    }

    // Obtener todos los usuarios con su rol
    #[instrument(skip(self))]
    pub async fn obtener_usuarios(&self) -> Result<Vec<usuario::UsuarioConRol>, AppError> {
        let db = self.get_connection().await;
        
        let usuarios = Usuario::find()
            .find_also_related(rol::Entity)
            .all(&db)
            .await?;

        let usuarios_con_rol = usuarios
            .into_iter()
            .filter_map(|(usuario, rol_opt)| {
                rol_opt.map(|rol| usuario::UsuarioConRol { usuario, rol })
            })
            .collect();

        Ok(usuarios_con_rol)
    }

    // Crear un nuevo usuario
    pub async fn crear_usuario(
        &self,
        nuevo_usuario: NewUsuario,
    ) -> Result<usuario::Model, AppError> {
        let db = self.get_connection().await;
        // Validar campos obligatorios
        if nuevo_usuario.nombre.trim().is_empty() {
            return Err(AppError::BadRequest(
                "El nombre completo es obligatorio".to_string(),
            ));
        }

        if nuevo_usuario.correo.trim().is_empty() {
            return Err(AppError::BadRequest(
                "El correo electrónico es obligatorio".to_string(),
            ));
        }

        // Validar formato de correo
        if !is_valid_email(&nuevo_usuario.correo) {
            return Err(AppError::BadRequest(
                "El formato del correo electrónico no es válido".to_string(),
            ));
        }

        if nuevo_usuario.contrasena.trim().is_empty() {
            return Err(AppError::BadRequest(
                "La contraseña es obligatoria".to_string(),
            ));
        }

        if nuevo_usuario.contrasena.len() < 6 {
            return Err(AppError::BadRequest(
                "La contraseña debe tener al menos 6 caracteres".to_string(),
            ));
        }

        // Verificar que el correo no esté en uso
        let existe_correo = Usuario::find()
            .filter(usuario::Column::Correo.eq(&nuevo_usuario.correo))
            .one(&db)
            .await?;

        if existe_correo.is_some() {
            return Err(AppError::Conflict(
                "Ya existe un usuario con este correo electrónico".to_string(),
            ));
        }

        // Si hay documento_nit, verificar que no esté en uso
        if let Some(documento) = &nuevo_usuario.documento_nit {
            if !documento.trim().is_empty() {
                let existe_documento = Usuario::find()
                    .filter(usuario::Column::DocumentoNit.eq(documento))
                    .one(&db)
                    .await?;

                if existe_documento.is_some() {
                    return Err(AppError::Conflict(
                        "Ya existe un usuario con este documento".to_string(),
                    ));
                }
            }
        }

        // Crear el nuevo usuario
        let usuario = usuario::ActiveModel {
            nombre: Set(nuevo_usuario.nombre),
            documento_nit: Set(nuevo_usuario.documento_nit),
            correo: Set(nuevo_usuario.correo),
            contrasena: Set(nuevo_usuario.contrasena),
            rol_id: Set(nuevo_usuario.rol_id),
            estado: Set(Some(nuevo_usuario.estado.unwrap_or(true))),
            fecha_creacion: Set(Some(Utc::now())),
            fecha_actualizacion: Set(Some(Utc::now())),
            ..Default::default()
        };

        let usuario = usuario.insert(&db).await?;
        Ok(usuario)
    }

    // Editar usuario existente
    pub async fn editar_usuario(
        &self,
        id: i32,
        datos_actualizados: UpdateUsuario,
    ) -> Result<usuario::Model, AppError> {
        let db = self.get_connection().await;
        let usuario = Usuario::find_by_id(id)
            .one(&db)
            .await?
            .ok_or_else(|| AppError::NotFound("Usuario no encontrado".to_string()))?;

        let mut usuario: usuario::ActiveModel = usuario.into();

        if let Some(nombre) = datos_actualizados.nombre {
            if nombre.trim().is_empty() {
                return Err(AppError::BadRequest("El nombre no puede estar vacío".to_string()));
            }
            usuario.nombre = Set(nombre);
        }

        if let Some(correo) = datos_actualizados.correo {
            if !is_valid_email(&correo) {
                return Err(AppError::BadRequest(
                    "El formato del correo electrónico no es válido".to_string(),
                ));
            }
            // Verificar que el nuevo correo no esté en uso por otro usuario
            let existe_correo = Usuario::find()
                .filter(usuario::Column::Correo.eq(&correo))
                .filter(usuario::Column::Id.ne(id))
                .one(&db)
                .await?;

            if existe_correo.is_some() {
                return Err(AppError::Conflict(
                    "Ya existe un usuario con este correo electrónico".to_string(),
                ));
            }
            usuario.correo = Set(correo);
        }

        if let Some(documento) = datos_actualizados.documento_nit {
            // Verificar que el documento no esté en uso por otro usuario
            if !documento.trim().is_empty() {
                let existe_documento = Usuario::find()
                    .filter(usuario::Column::DocumentoNit.eq(&documento))
                    .filter(usuario::Column::Id.ne(id))
                    .one(&db)
                    .await?;

                if existe_documento.is_some() {
                    return Err(AppError::Conflict(
                        "Ya existe un usuario con este documento".to_string(),
                    ));
                }
            }
            usuario.documento_nit = Set(Some(documento));
        }

        if let Some(contrasena) = datos_actualizados.contrasena {
            if contrasena.len() < 6 {
                return Err(AppError::BadRequest(
                    "La contraseña debe tener al menos 6 caracteres".to_string(),
                ));
            }
            usuario.contrasena = Set(contrasena);
        }

        if let Some(rol_id) = datos_actualizados.rol_id {
            usuario.rol_id = Set(rol_id);
        }

        if let Some(estado) = datos_actualizados.estado {
            usuario.estado = Set(Some(estado));
        }

        usuario.fecha_actualizacion = Set(Some(Utc::now()));
        let usuario = usuario.update(&db).await?;

        Ok(usuario)
    }

    // Obtener usuario por ID
    pub async fn obtener_usuario_por_id(
        &self,
        id: i32,
    ) -> Result<Option<usuario::Model>, AppError> {
        let db = self.get_connection().await;
        let usuario = Usuario::find_by_id(id).one(&db).await?;
        Ok(usuario)
    }
}

impl FromRef<AppState> for UsuarioService {
    fn from_ref(app_state: &AppState) -> Self {
        let executor = app_state.db.clone().expect("Database connection is not available");
        UsuarioService::new(executor)
    }
}
