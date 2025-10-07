use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder,
    Set, TransactionTrait,
};
use validator::Validate;

use crate::{
    models::{
        rol,
        usuario::{self, Entity as Usuario, Model as UsuarioModel, NewUsuario, UpdateUsuario},
    },
    utils::errors::AppError,
};

#[derive(Debug, Clone)]
pub struct UsuarioService {
    db: DatabaseConnection,
}

impl UsuarioService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    // Iniciar sesión
    pub async fn login_usuario(
        &self,
        identificador: &str,
        contrasena: &str,
    ) -> Result<UsuarioModel, AppError> {
        let usuario = Usuario::find()
            .filter(
                usuario::Column::DocumentoNit
                    .eq(identificador)
                    .or(usuario::Column::Correo.eq(identificador)),
            )
            .filter(usuario::Column::Contrasena.eq(contrasena))
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Credenciales inválidas".to_string()))?;

        // Actualizar última conexión
        let mut usuario: usuario::ActiveModel = usuario.into();
        usuario.updated_at = Set(Some(Utc::now()));
        let usuario = usuario.update(&self.db).await?;

        Ok(usuario)
    }

    // Cerrar sesión
    pub async fn logout_usuario(&self, id: i64) -> Result<UsuarioModel, AppError> {
        let usuario = Usuario::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Usuario no encontrado".to_string()))?;

        // Actualizar última conexión
        let mut usuario: usuario::ActiveModel = usuario.into();
        usuario.updated_at = Set(Some(Utc::now()));
        let usuario = usuario.update(&self.db).await?;

        Ok(usuario)
    }

    // Obtener todos los usuarios con su rol
    pub async fn obtener_usuarios(&self) -> Result<Vec<usuario::UsuarioConRol>, DbErr> {
        let usuarios = Usuario::find()
            .find_with_related(rol::Entity)
            .all(&self.db)
            .await?;

        let usuarios_con_rol = usuarios
            .into_iter()
            .map(|(usuario, roles)| usuario::UsuarioConRol {
                usuario,
                rol: roles.into_iter().next().unwrap(), // Asumimos que hay al menos un rol
            })
            .collect();

        Ok(usuarios_con_rol)
    }

    // Crear un nuevo usuario
    pub async fn crear_usuario(
        &self,
        nuevo_usuario: NewUsuario,
    ) -> Result<usuario::Model, AppError> {
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
        if !validator::validate_email(&nuevo_usuario.correo) {
            return Err(AppError::BadRequest(
                "El formato del correo electrónico no es válido".to_string(),
            ));
        }

        if nuevo_usuario.contrasena.trim().is_empty() {
            return Err(AppService::BadRequest(
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
            .one(&self.db)
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
                    .one(&self.db)
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
            estado: Set(nuevo_usuario.estado.unwrap_or(true)),
            created_at: Set(Some(Utc::now())),
            updated_at: Set(Some(Utc::now())),
            ..Default::default()
        };

        let usuario = usuario.insert(&self.db).await?;
        Ok(usuario)
    }

    // Editar usuario existente
    pub async fn editar_usuario(
        &self,
        id: i64,
        datos_actualizados: UpdateUsuario,
    ) -> Result<usuario::Model, AppError> {
        let usuario = Usuario::find_by_id(id)
            .one(&self.db)
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
            if !validator::validate_email(&correo) {
                return Err(AppError::BadRequest(
                    "El formato del correo electrónico no es válido".to_string(),
                ));
            }
            // Verificar que el nuevo correo no esté en uso por otro usuario
            let existe_correo = Usuario::find()
                .filter(usuario::Column::Correo.eq(&correo))
                .filter(usuario::Column::Id.ne(id))
                .one(&self.db)
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
                    .one(&self.db)
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
            usuario.estado = Set(estado);
        }

        usuario.updated_at = Set(Some(Utc::now()));
        let usuario = usuario.update(&self.db).await?;

        Ok(usuario)
    }

    // Obtener usuario por ID
    pub async fn obtener_usuario_por_id(
        &self,
        id: i64,
    ) -> Result<Option<usuario::Model>, DbErr> {
        Usuario::find_by_id(id).one(&self.db).await
    }
}
