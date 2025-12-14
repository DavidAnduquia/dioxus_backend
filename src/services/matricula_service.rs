use axum::extract::FromRef;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, Order, PaginatorTrait,
    QueryFilter, QueryOrder, Set,
};

use crate::{
    database::DbExecutor,
    models::{
        curso::Entity as Curso,
        historial_curso_estudiante::{self, Entity as Historial, Model as HistorialModel},
        usuario::Entity as Usuario,
        AppState,
    },
    utils::errors::AppError,
};

#[derive(Debug, Clone)]
pub struct MatriculaService {
    db: DbExecutor,
}

impl MatriculaService {
    pub fn new(db: DbExecutor) -> Self {
        Self { db }
    }

    fn connection(&self) -> DatabaseConnection {
        self.db.connection()
    }

    pub async fn matricular_estudiante(
        &self,
        estudiante_id: i32,
        curso_id: i32,
    ) -> Result<HistorialModel, AppError> {
        let db = self.connection();

        Usuario::find_by_id(estudiante_id)
            .one(&db)
            .await?
            .ok_or_else(|| AppError::NotFound("Estudiante no encontrado".into()))?;

        Curso::find_by_id(curso_id)
            .one(&db)
            .await?
            .ok_or_else(|| AppError::NotFound("Curso no encontrado".into()))?;

        if Historial::find()
            .filter(historial_curso_estudiante::Column::EstudianteId.eq(estudiante_id))
            .filter(historial_curso_estudiante::Column::CursoId.eq(curso_id))
            .one(&db)
            .await?
            .is_some()
        {
            return Err(AppError::BadRequest("Estudiante ya matriculado".into()));
        }

        let ahora = Utc::now();
        let matricula = historial_curso_estudiante::ActiveModel {
            estudiante_id: Set(estudiante_id),
            curso_id: Set(curso_id),
            fecha_inscripcion: Set(ahora),
            estado: Set("activo".into()),
            created_at: Set(Some(ahora)),
            updated_at: Set(Some(ahora)),
            ..Default::default()
        };

        Ok(matricula.insert(&db).await?)
    }

    pub async fn desmatricular_estudiante(
        &self,
        estudiante_id: i32,
        curso_id: i32,
    ) -> Result<HistorialModel, AppError> {
        let db = self.connection();

        Usuario::find_by_id(estudiante_id)
            .one(&db)
            .await?
            .ok_or_else(|| AppError::NotFound("Estudiante no encontrado".into()))?;

        Curso::find_by_id(curso_id)
            .one(&db)
            .await?
            .ok_or_else(|| AppError::NotFound("Curso no encontrado".into()))?;

        let matricula = Historial::find()
            .filter(historial_curso_estudiante::Column::EstudianteId.eq(estudiante_id))
            .filter(historial_curso_estudiante::Column::CursoId.eq(curso_id))
            .one(&db)
            .await?
            .ok_or_else(|| AppError::BadRequest("Estudiante no matriculado".into()))?;

        let ahora = Utc::now();
        let mut matricula: historial_curso_estudiante::ActiveModel = matricula.into();
        matricula.estado = Set("inactivo".into());
        matricula.updated_at = Set(Some(ahora));

        let matricula_actualizada = matricula.update(&db).await?;
        Ok(matricula_actualizada)
    }

    pub async fn obtener_matriculas_estudiante(
        &self,
        estudiante_id: i32,
    ) -> Result<Vec<HistorialModel>, AppError> {
        let db = self.connection();
        let matriculas = Historial::find()
            .filter(historial_curso_estudiante::Column::EstudianteId.eq(estudiante_id))
            .order_by(
                historial_curso_estudiante::Column::FechaInscripcion,
                Order::Asc,
            )
            .all(&db)
            .await?;

        Ok(matriculas)
    }

    pub async fn obtener_matriculas_curso(
        &self,
        curso_id: i32,
    ) -> Result<Vec<HistorialModel>, AppError> {
        let db = self.connection();

        println!(
            "🔍 Debug: Método obtener_matriculas_curso llamado para curso_id: {}",
            curso_id
        );

        // Verificar primero si existen matrículas
        let matriculas_count = Historial::find()
            .filter(historial_curso_estudiante::Column::CursoId.eq(curso_id))
            .count(&db)
            .await?;

        println!(
            "📊 Encontradas {} matrículas en total para curso {}",
            matriculas_count, curso_id
        );

        // Obtener matrículas con información del estudiante usando JOIN
        let matriculas_con_estudiantes = Historial::find()
            .filter(historial_curso_estudiante::Column::CursoId.eq(curso_id))
            .find_also_related(Usuario)
            .order_by(
                historial_curso_estudiante::Column::FechaInscripcion,
                Order::Asc,
            )
            .all(&db)
            .await?;

        // Debug: Verificar si los usuarios existen
        let estudiante_ids: Vec<i32> = matriculas_con_estudiantes
            .iter()
            .map(|(matricula, _)| matricula.estudiante_id)
            .collect();

        println!("👥 IDs de estudiantes encontrados: {:?}", estudiante_ids);

        for estudiante_id in &estudiante_ids {
            if let Some(usuario) = Usuario::find_by_id(*estudiante_id).one(&db).await? {
                println!(
                    "✅ Usuario {} existe: {} ({})",
                    estudiante_id, usuario.nombre, usuario.correo
                );
            } else {
                println!("❌ Usuario {} NO existe en tabla usuarios", estudiante_id);
            }
        }

        println!(
            "🔍 Debug JOIN: Encontradas {} matrículas con JOIN para curso {}",
            matriculas_con_estudiantes.len(),
            curso_id
        );
        for (matricula, estudiante_opt) in &matriculas_con_estudiantes {
            println!(
                "📝 Matrícula ID: {}, Estudiante ID: {}, Usuario encontrado: {}",
                matricula.id,
                matricula.estudiante_id,
                estudiante_opt.is_some()
            );
            if let Some(estudiante) = estudiante_opt {
                println!(
                    "👤 Usuario datos: ID={}, Nombre='{}', Correo='{}'",
                    estudiante.id, estudiante.nombre, estudiante.correo
                );
            } else {
                println!(
                    "❌ Usuario NO encontrado para estudiante_id: {}",
                    matricula.estudiante_id
                );
            }
        }

        // Transformar los resultados para incluir la información del estudiante en el modelo
        let mut matriculas_completas = Vec::new();

        for (matricula, estudiante_opt) in matriculas_con_estudiantes {
            let mut matricula_completa = matricula.clone();

            // Si tenemos información del estudiante, la agregamos a los campos adicionales
            if let Some(estudiante) = estudiante_opt {
                matricula_completa.nombre = Some(estudiante.nombre.clone());
                matricula_completa.email = Some(estudiante.correo.clone());
            }

            matriculas_completas.push(matricula_completa);
        }

        Ok(matriculas_completas)
    }
}

impl FromRef<AppState> for MatriculaService {
    fn from_ref(state: &AppState) -> Self {
        let executor = state
            .db
            .clone()
            .expect("Database connection is not available");
        MatriculaService::new(executor)
    }
}
