#[async_trait::async_trait]
pub trait MatriculaServiceTrait {
    async fn matricular_estudiante(
        &self,
        estudiante_id: i64,
        curso_id: i32
    ) -> Result<HistorialModel, AppError>;

    async fn desmatricular_estudiante(
        &self,
        estudiante_id: i64,
        curso_id: i32
    ) -> Result<HistorialModel, AppError>;

    async fn obtener_matriculas_estudiante(
        &self,
        estudiante_id: i64
    ) -> Result<Vec<HistorialModel>, AppError>;

    async fn obtener_matriculas_curso(
        &self,
        curso_id: i32
    ) -> Result<Vec<HistorialModel>, AppError>;
}

#[derive(Debug, Clone)]
pub struct MatriculaService {
    db: DatabaseConnection,
}

#[async_trait::async_trait]
impl MatriculaServiceTrait for MatriculaService {
    async fn matricular_estudiante(
        &self,
        estudiante_id: i64,
        curso_id: i32
    ) -> Result<HistorialModel, AppError> {
        // Verificar que el estudiante existe
        let estudiante = Usuario::find_by_id(estudiante_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Estudiante no encontrado".into()))?;

        // Verificar que el curso existe
        let _curso = Curso::find_by_id(curso_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Curso no encontrado".into()))?;

        // Verificar matrícula existente
        let existe = Historial::find()
            .filter(historial::Column::EstudianteId.eq(estudiante_id))
            .filter(historial::Column::CursoId.eq(curso_id))
            .one(&self.db)
            .await?;

        if existe.is_some() {
            return Err(AppError::BadRequest("Estudiante ya matriculado".into()));
        }

        // Crear matrícula
        let ahora = Utc::now();
        let matricula = historial::ActiveModel {
            estudiante_id: Set(estudiante_id),
            curso_id: Set(curso_id),
            fecha_inscripcion: Set(ahora),
            estado: Set("activo".into()),
            created_at: Set(Some(ahora)),
            updated_at: Set(Some(ahora)),
            ..Default::default()
        };

        Ok(matricula.insert(&self.db).await?)
    }

    async fn desmatricular_estudiante(
        &self,
        estudiante_id: i64,
        curso_id: i32
    ) -> Result<HistorialModel, AppError> {
        // Verificar que el estudiante existe
        let estudiante = Usuario::find_by_id(estudiante_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Estudiante no encontrado".into()))?;

        // Verificar que el curso existe
        let _curso = Curso::find_by_id(curso_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Curso no encontrado".into()))?;

        // Verificar matrícula existente
        let existe = Historial::find()
            .filter(historial::Column::EstudianteId.eq(estudiante_id))
            .filter(historial::Column::CursoId.eq(curso_id))
            .one(&self.db)
            .await?;

        if existe.is_none() {
            return Err(AppError::BadRequest("Estudiante no matriculado".into()));
        }

        // Desmatricular
        let ahora = Utc::now();
        let matricula = existe.unwrap();
        let matricula_actualizada = matricula.update(&self.db, Set(historial::Column::Estado.eq("inactivo".into()))
            .await?
            .ok_or(AppError::InternalServerError("Error al desmatricular".into()))?;

        Ok(matricula_actualizada)
    }

    async fn obtener_matriculas_estudiante(
        &self,
        estudiante_id: i64
    ) -> Result<Vec<HistorialModel>, AppError> {
        let matriculas = Historial::find()
            .filter(historial::Column::EstudianteId.eq(estudiante_id))
            .order_by_asc(historial::Column::FechaInscripcion)
            .all(&self.db)
            .await?
            .ok_or(AppError::InternalServerError("Error al obtener matrículas".into()))?;

        Ok(matriculas)
    }

    async fn obtener_matriculas_curso(
        &self,
        curso_id: i32
    ) -> Result<Vec<HistorialModel>, AppError> {
        let matriculas = Historial::find()
            .filter(historial::Column::CursoId.eq(curso_id))
            .order_by_asc(historial::Column::FechaInscripcion)
            .all(&self.db)
            .await?
            .ok_or(AppError::InternalServerError("Error al obtener matrículas".into()))?;

        Ok(matriculas)
    }
}