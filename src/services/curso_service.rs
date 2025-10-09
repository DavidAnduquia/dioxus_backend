use axum::extract::FromRef;
use chrono::{DateTime, Datelike, Utc};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait,
    QueryFilter, QueryOrder, Set, TransactionTrait, Order
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    database::DbExecutor,
    models::{
        actividad::{self, Entity as Actividad, Model as ActividadModel},
        area_conocimiento::{Entity as AreaConocimientoEntity, Model as AreaConocimiento},
        contenido_transversal::{self, Entity as ContenidoTransversal, Model as ContenidoTransversalModel},
        curso::{self, Entity as Curso, Model as CursoModel},
        evaluacion_sesion::{self, Entity as EvaluacionSesion, Model as EvaluacionSesionModel},
        plantilla_curso::{self, Entity as PlantillaCurso},
        usuario::Entity as Usuario,
        AppState,
    },
    utils::errors::AppError,
};

#[derive(Debug, Clone)]
pub struct CursoService {
    db: DbExecutor,
}

impl FromRef<AppState> for CursoService {
    fn from_ref(state: &AppState) -> Self {
        let executor = state.db.clone().expect("Database connection is not available");
        CursoService::new(executor)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevoCurso {
    pub nombre: String,
    pub codigo: String,
    pub descripcion: Option<String>,
    pub creditos: i32,
    pub horas_teoricas: i32,
    pub horas_practicas: i32,
    pub area_conocimiento_id: i32,
    pub estado: bool,
    pub prerequisito: Option<String>,
    pub periodo: Option<String>,
    pub fecha_inicio: DateTime<Utc>,
    pub fecha_fin: DateTime<Utc>,
    pub anio_pensum: Option<i32>,
    pub coordinador_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActualizarCurso {
    pub nombre: Option<String>,
    pub codigo: Option<String>,
    pub descripcion: Option<String>,
    pub creditos: Option<i32>,
    pub horas_teoricas: Option<i32>,
    pub horas_practicas: Option<i32>,
    pub area_conocimiento_id: Option<i32>,
    pub estado: Option<bool>,
    pub prerequisito: Option<String>,
    pub periodo: Option<String>,
    pub fecha_inicio: Option<DateTime<Utc>>,
    pub fecha_fin: Option<DateTime<Utc>>,
    pub anio_pensum: Option<i32>,
    pub coordinador_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursoDetallado {
    pub curso: CursoModel,
    pub area: Option<AreaConocimiento>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AulaCurso {
    pub curso: CursoModel,
    pub modulos: Vec<ContenidoTransversalModel>,
    pub actividades: Vec<ActividadModel>,
    pub evaluaciones: Vec<EvaluacionSesionModel>,
}

impl CursoService {
    pub fn new(db: DbExecutor) -> Self {
        Self { db }
    }

    fn pool(&self) -> &PgPool {
        self.db.pool()
    }

    fn connection(&self) -> DatabaseConnection {
        self.db.connection()
    }

    pub async fn obtener_cursos(&self) -> Result<Vec<CursoDetallado>, AppError> {
        let db = self.connection();
        let cursos = Curso::find()
            .order_by_desc(curso::Column::CreatedAt)
            .find_also_related(AreaConocimientoEntity)
            .all(&db)
            .await
            .map_err(map_db_err)?
            .into_iter()
            .map(|(curso, area)| CursoDetallado { curso, area })
            .collect();

        Ok(cursos)
    }

    pub async fn obtener_curso_por_id(
        &self,
        id: i32,
    ) -> Result<CursoDetallado, AppError> {
        let db = self.connection();
        let result = Curso::find_by_id(id)
            .find_also_related(AreaConocimientoEntity)
            .one(&db)
            .await
            .map_err(map_db_err)?
            .ok_or_else(|| AppError::NotFound(format!("Curso con id {} no encontrado", id)))?;

        Ok(CursoDetallado {
            curso: result.0,
            area: result.1,
        })
    }

    pub async fn crear_curso(&self, datos: NuevoCurso) -> Result<CursoModel, AppError> {
        if datos.nombre.trim().is_empty() {
            return Err(AppError::BadRequest("El nombre del curso es obligatorio".to_string()));
        }
        if datos.descripcion.as_ref().map_or(true, |d| d.trim().is_empty()) {
            return Err(AppError::BadRequest("La descripción del curso es obligatoria".to_string()));
        }
        if datos.fecha_fin <= datos.fecha_inicio {
            return Err(AppError::BadRequest(
                "La fecha de fin debe ser posterior a la fecha de inicio".to_string(),
            ));
        }

        // Validar área de conocimiento
        let db = self.connection();
        let area = AreaConocimientoEntity::find_by_id(datos.area_conocimiento_id)
            .one(&db)
            .await
            .map_err(map_db_err)?
            .ok_or_else(|| AppError::BadRequest("El área de conocimiento especificada no existe".to_string()))?;
        if !area.estado {
            return Err(AppError::BadRequest(
                "El área de conocimiento especificada no está activa".to_string(),
            ));
        }

        // Validar coordinador si se envía
        if let Some(coordinador_id) = datos.coordinador_id {
            Usuario::find_by_id(coordinador_id)
                .one(&db)
                .await
                .map_err(map_db_err)?
                .ok_or_else(|| AppError::BadRequest("El coordinador especificado no existe".to_string()))?;
        }

        let periodo_final = datos.periodo.clone().unwrap_or_else(|| {
            let year = datos.fecha_inicio.year();
            let month = datos.fecha_inicio.month();
            format!("{}-{}", year, if month <= 6 { 1 } else { 2 })
        });
        let anio_pensum_final = datos.anio_pensum.unwrap_or(datos.fecha_inicio.year());

        let mut txn = db.begin().await.map_err(map_db_err)?;
        let ahora = Utc::now();

        let curso_activo = curso::ActiveModel {
            id: Set(0), // Auto-increment field
            nombre: Set(datos.nombre.clone()),
            codigo: Set(datos.codigo.clone()),
            descripcion: Set(datos.descripcion.clone()),
            creditos: Set(datos.creditos),
            horas_teoricas: Set(datos.horas_teoricas),
            horas_practicas: Set(datos.horas_practicas),
            area_conocimiento_id: Set(datos.area_conocimiento_id),
            estado: Set(datos.estado),
            periodo: Set(Some(periodo_final.clone())),
            fecha_inicio: Set(Some(datos.fecha_inicio)),
            fecha_fin: Set(Some(datos.fecha_fin)),
            anio_pensum: Set(Some(anio_pensum_final)),
            coordinador_id: Set(datos.coordinador_id),
            plantilla_base_id: Set(None),
            prerequisito: Set(datos.prerequisito.clone()),
            created_at: Set(Some(ahora)),
            updated_at: Set(Some(ahora)),
        };

        let curso_creado = curso_activo
            .insert(&mut txn)
            .await
            .map_err(map_db_err)?;

        let plantilla_activa = plantilla_curso::ActiveModel {
            id: Set(0), // Auto-increment field
            nombre: Set(format!("Plantilla - {}", curso_creado.nombre)),
            descripcion: Set(curso_creado.descripcion.clone()),
            activa: Set(true),
            curso_id: Set(Some(curso_creado.id)),
            fecha_creacion: Set(Some(Utc::now())),
            created_at: Set(Some(Utc::now())),
            updated_at: Set(Some(Utc::now())),
        };

        let plantilla_creada = plantilla_activa
            .insert(&mut txn)
            .await
            .map_err(map_db_err)?;

        let mut curso_para_actualizar: curso::ActiveModel = curso_creado.clone().into();
        curso_para_actualizar.plantilla_base_id = Set(Some(plantilla_creada.id));
        curso_para_actualizar.updated_at = Set(Some(Utc::now()));

        let curso_final = curso_para_actualizar
            .update(&mut txn)
            .await
            .map_err(map_db_err)?;

        txn.commit().await.map_err(map_db_err)?;

        Ok(curso_final)
    }

    pub async fn editar_curso(
        &self,
        id: i32,
        datos: ActualizarCurso,
    ) -> Result<CursoModel, AppError> {
        let db = self.connection();
        let mut txn = db.begin().await.map_err(map_db_err)?;

        let mut curso_model = Curso::find_by_id(id)
            .one(&mut txn)
            .await
            .map_err(map_db_err)?
            .ok_or_else(|| AppError::NotFound("Curso no encontrado".to_string()))?;

        if let Some(nombre) = datos.nombre {
            if nombre.trim().is_empty() {
                return Err(AppError::BadRequest("El nombre no puede estar vacío".to_string()));
            }
            curso_model.nombre = nombre;
        }

        if let Some(codigo) = datos.codigo {
            curso_model.codigo = codigo;
        }

        if let Some(descripcion) = datos.descripcion {
            curso_model.descripcion = Some(descripcion);
        }

        if let Some(creditos) = datos.creditos {
            curso_model.creditos = creditos;
        }

        if let Some(horas_teoricas) = datos.horas_teoricas {
            curso_model.horas_teoricas = horas_teoricas;
        }

        if let Some(horas_practicas) = datos.horas_practicas {
            curso_model.horas_practicas = horas_practicas;
        }

        if let Some(area_id) = datos.area_conocimiento_id {
            let area = AreaConocimientoEntity::find_by_id(area_id)
                .one(&mut txn)
                .await
                .map_err(map_db_err)?
                .ok_or_else(|| AppError::BadRequest("El área de conocimiento especificada no existe".to_string()))?;
            if !area.estado {
                return Err(AppError::BadRequest(
                    "El área de conocimiento especificada no está activa".to_string(),
                ));
            }
            curso_model.area_conocimiento_id = area_id;
        }

        if let Some(estado) = datos.estado {
            curso_model.estado = estado;
        }

        if let Some(prerequisito) = datos.prerequisito {
            curso_model.prerequisito = Some(prerequisito);
        }

        if let Some(coordinador_id) = datos.coordinador_id {
            Usuario::find_by_id(coordinador_id)
                .one(&mut txn)
                .await
                .map_err(map_db_err)?
                .ok_or_else(|| AppError::BadRequest("El coordinador especificado no existe".to_string()))?;
            curso_model.coordinador_id = Some(coordinador_id);
        }

        if let Some(fecha_inicio) = datos.fecha_inicio {
            curso_model.fecha_inicio = Some(fecha_inicio);
        }

        if let Some(fecha_fin) = datos.fecha_fin {
            curso_model.fecha_fin = Some(fecha_fin);
        }

        if let (Some(inicio), Some(fin)) = (curso_model.fecha_inicio, curso_model.fecha_fin) {
            if fin <= inicio {
                return Err(AppError::BadRequest(
                    "La fecha de fin debe ser posterior a la fecha de inicio".to_string(),
                ));
            }
        }

        if let Some(periodo) = datos.periodo {
            curso_model.periodo = Some(periodo);
        }

        if let Some(anio_pensum) = datos.anio_pensum {
            curso_model.anio_pensum = Some(anio_pensum);
        }

        curso_model.updated_at = Some(Utc::now());

        let curso_activo: curso::ActiveModel = curso_model.into();
        let curso_actualizado = curso_activo
            .update(&mut txn)
            .await
            .map_err(map_db_err)?;

        txn.commit().await.map_err(map_db_err)?;

        Ok(curso_actualizado)
    }

    pub async fn eliminar_curso(&self, id: i32) -> Result<(), AppError> {
        let db = self.connection();
        let mut txn = db.begin().await.map_err(map_db_err)?;

        let curso = Curso::find_by_id(id)
            .one(&mut txn)
            .await
            .map_err(map_db_err)?
            .ok_or_else(|| AppError::NotFound("Curso no encontrado".to_string()))?;

        if let Some(plantilla_id) = curso.plantilla_base_id {
            PlantillaCurso::delete_by_id(plantilla_id)
                .exec(&mut txn)
                .await
                .map_err(map_db_err)?;
        }

        curso.delete(&mut txn).await.map_err(map_db_err)?;
        txn.commit().await.map_err(map_db_err)?;
        Ok(())
    }

    pub async fn obtener_cursos_por_plantilla(
        &self,
        plantilla_id: i32,
    ) -> Result<Vec<CursoModel>, AppError> {
        let db = self.connection();
        let cursos = Curso::find()
            .filter(curso::Column::PlantillaBaseId.eq(plantilla_id))
            .all(&db)
            .await
            .map_err(map_db_err)?;

        Ok(cursos)
    }

    pub async fn obtener_cursos_por_area_y_periodo(
        &self,
        area_conocimiento_id: i32,
        periodo: &str,
    ) -> Result<Vec<CursoDetallado>, AppError> {
        let db = self.connection();
        let cursos = Curso::find()
            .filter(curso::Column::AreaConocimientoId.eq(area_conocimiento_id))
            .filter(curso::Column::Periodo.eq(periodo))
            .find_also_related(AreaConocimientoEntity)
            .all(&db)
            .await
            .map_err(map_db_err)?
            .into_iter()
            .map(|(curso, area)| CursoDetallado { curso, area })
            .collect();

        Ok(cursos)
    }

    pub async fn obtener_aula_por_curso_id(&self, id: i32) -> Result<AulaCurso, AppError> {
        let db = self.connection();
        let curso = Curso::find_by_id(id)
            .one(&db)
            .await
            .map_err(map_db_err)?
            .ok_or_else(|| AppError::NotFound("Curso no encontrado".to_string()))?;

        let modulos = ContenidoTransversal::find()
            .filter(contenido_transversal::Column::CursoId.eq(id))
            .order_by(contenido_transversal::Column::CreatedAt, Order::Asc)
            .all(&db)
            .await
            .map_err(map_db_err)?;

        let actividades = Actividad::find()
            .filter(actividad::Column::CursoId.eq(id))
            .order_by(actividad::Column::CreatedAt, Order::Asc)
            .all(&db)
            .await
            .map_err(map_db_err)?;

        let actividades_ids: Vec<i32> = actividades.iter().map(|a| a.id).collect();

        let evaluaciones = if actividades_ids.is_empty() {
            Vec::new()
        } else {
            EvaluacionSesion::find()
                .filter(evaluacion_sesion::Column::SesionId.is_in(actividades_ids))
                .all(&db)
                .await
                .map_err(map_db_err)?
        };

        Ok(AulaCurso {
            curso,
            modulos,
            actividades,
            evaluaciones,
        })
    }
}
fn map_db_err(err: DbErr) -> AppError {
    AppError::InternalServerError(err.to_string())
}