use axum::extract::FromRef;
use chrono::{NaiveDate, Utc};
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    Order, QueryFilter, QueryOrder, QuerySelect, RelationTrait, Set, TransactionTrait,
};
use serde::{Deserialize, Serialize};

use crate::{
    database::DbExecutor,
    models::{
        area_conocimiento::{Entity as AreaConocimientoEntity, Model as AreaConocimiento},
        curso::{self, Entity as Curso, Model as CursoModel},
        modulo::{self},
        tema::{self, Entity as Tema, Model as TemaModel},
        unidad::{self, Entity as Unidad, Model as UnidadModel},
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
        let executor = state
            .db
            .clone()
            .expect("Database connection is not available");
        CursoService::new(executor)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevoCurso {
    pub nombre: String,
    pub descripcion: String,
    pub fecha_inicio: NaiveDate,
    pub fecha_fin: NaiveDate,
    pub prerequisito: Option<String>,
    pub coordinador_id: i32,
    pub semestre: Option<i32>,
    pub periodo: String,
    pub anio_pensum: i32,
    pub area_conocimiento_id: i32,
    pub plantilla_base_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActualizarCurso {
    pub nombre: Option<String>,
    pub descripcion: Option<String>,
    pub fecha_inicio: Option<NaiveDate>,
    pub fecha_fin: Option<NaiveDate>,
    pub prerequisito: Option<String>,
    pub coordinador_id: Option<i32>,
    pub semestre: Option<i32>,
    pub periodo: Option<String>,
    pub anio_pensum: Option<i32>,
    pub area_conocimiento_id: Option<i32>,
    pub plantilla_base_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursoDetallado {
    pub curso: CursoModel,
    pub area: Option<AreaConocimiento>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnidadAula {
    pub id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub orden: Option<i32>,
    pub tema_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemaAula {
    pub id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub orden: Option<i32>,
    pub unidades: Vec<UnidadAula>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AulaCurso {
    pub curso: CursoModel,
    pub temas: Vec<TemaAula>,
}

impl CursoService {
    pub fn new(db: DbExecutor) -> Self {
        Self { db }
    }

    fn connection(&self) -> DatabaseConnection {
        self.db.connection()
    }

    pub async fn obtener_cursos(&self) -> Result<Vec<CursoDetallado>, AppError> {
        let db = self.connection();
        let cursos = Curso::find()
            .order_by_desc(curso::Column::CreadoEn)
            .find_also_related(AreaConocimientoEntity)
            .all(&db)
            .await
            .map_err(map_db_err)?
            .into_iter()
            .map(|(curso, area)| CursoDetallado { curso, area })
            .collect();

        Ok(cursos)
    }

    pub async fn obtener_curso_por_id(&self, id: i32) -> Result<CursoDetallado, AppError> {
        let db = self.connection();
        let result = Curso::find_by_id(id)
            .find_also_related(AreaConocimientoEntity)
            .one(&db)
            .await
            .map_err(map_db_err)?
            .ok_or_else(|| {
                AppError::NotFound(format!("Curso con id {} no encontrado", id).into())
            })?;

        Ok(CursoDetallado {
            curso: result.0,
            area: result.1,
        })
    }

    pub async fn crear_curso(&self, datos: NuevoCurso) -> Result<CursoModel, AppError> {
        if datos.nombre.trim().is_empty() {
            return Err(AppError::BadRequest(
                "El nombre del curso es obligatorio".into(),
            ));
        }
        if datos.descripcion.trim().is_empty() {
            return Err(AppError::BadRequest(
                "La descripción del curso es obligatoria".into(),
            ));
        }
        if datos.fecha_fin <= datos.fecha_inicio {
            return Err(AppError::BadRequest(
                "La fecha de fin debe ser posterior a la fecha de inicio".into(),
            ));
        }

        // Validar área de conocimiento
        let db = self.connection();
        let area = AreaConocimientoEntity::find_by_id(datos.area_conocimiento_id)
            .one(&db)
            .await
            .map_err(map_db_err)?
            .ok_or_else(|| {
                AppError::BadRequest("El área de conocimiento especificada no existe".into())
            })?;
        if !area.estado {
            return Err(AppError::BadRequest(
                "El área de conocimiento especificada no está activa".into(),
            ));
        }

        // Validar coordinador
        Usuario::find_by_id(datos.coordinador_id)
            .one(&db)
            .await
            .map_err(map_db_err)?
            .ok_or_else(|| AppError::BadRequest("El coordinador especificado no existe".into()))?;

        let periodo_final = datos.periodo.clone();
        let anio_pensum_final = datos.anio_pensum;

        let mut txn = db.begin().await.map_err(map_db_err)?;
        let ahora = Utc::now();

        let curso_activo = curso::ActiveModel {
            id: NotSet, // Auto-increment field
            nombre: Set(datos.nombre.clone()),
            descripcion: Set(datos.descripcion.clone()),
            fecha_inicio: Set(datos.fecha_inicio),
            fecha_fin: Set(datos.fecha_fin),
            prerequisito: Set(datos.prerequisito.clone()),
            coordinador_id: Set(datos.coordinador_id),
            creado_en: Set(ahora),
            plantilla_base_id: Set(None),
            semestre: Set(datos.semestre),
            periodo: Set(periodo_final.clone()),
            anio_pensum: Set(anio_pensum_final),
            area_conocimiento_id: Set(datos.area_conocimiento_id),
            fecha_eliminacion: Set(None),
            fecha_actualizacion: Set(ahora),
        };

        let curso_creado = curso_activo.insert(&mut txn).await.map_err(map_db_err)?;

        txn.commit().await.map_err(map_db_err)?;

        Ok(curso_creado)
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
            .ok_or_else(|| AppError::NotFound("Curso no encontrado".into()))?;

        if let Some(nombre) = datos.nombre {
            if nombre.trim().is_empty() {
                return Err(AppError::BadRequest(
                    "El nombre no puede estar vacío".into(),
                ));
            }
            curso_model.nombre = nombre;
        }

        if let Some(descripcion) = datos.descripcion {
            curso_model.descripcion = descripcion;
        }

        if let Some(area_id) = datos.area_conocimiento_id {
            let area = AreaConocimientoEntity::find_by_id(area_id)
                .one(&mut txn)
                .await
                .map_err(map_db_err)?
                .ok_or_else(|| {
                    AppError::BadRequest("El área de conocimiento especificada no existe".into())
                })?;
            if !area.estado {
                return Err(AppError::BadRequest(
                    "El área de conocimiento especificada no está activa".into(),
                ));
            }
            curso_model.area_conocimiento_id = area_id;
        }

        if let Some(prerequisito) = datos.prerequisito {
            curso_model.prerequisito = Some(prerequisito);
        }

        if let Some(coordinador_id) = datos.coordinador_id {
            Usuario::find_by_id(coordinador_id)
                .one(&mut txn)
                .await
                .map_err(map_db_err)?
                .ok_or_else(|| {
                    AppError::BadRequest("El coordinador especificado no existe".into())
                })?;
            curso_model.coordinador_id = coordinador_id;
        }

        if let Some(fecha_inicio) = datos.fecha_inicio {
            curso_model.fecha_inicio = fecha_inicio;
        }

        if let Some(fecha_fin) = datos.fecha_fin {
            curso_model.fecha_fin = fecha_fin;
        }

        if curso_model.fecha_fin <= curso_model.fecha_inicio {
            return Err(AppError::BadRequest(
                "La fecha de fin debe ser posterior a la fecha de inicio".into(),
            ));
        }

        if let Some(periodo) = datos.periodo {
            curso_model.periodo = periodo;
        }

        if let Some(anio_pensum) = datos.anio_pensum {
            curso_model.anio_pensum = anio_pensum;
        }

        curso_model.fecha_actualizacion = Utc::now();

        let curso_activo: curso::ActiveModel = curso_model.into();
        let curso_actualizado = curso_activo.update(&mut txn).await.map_err(map_db_err)?;

        txn.commit().await.map_err(map_db_err)?;

        Ok(curso_actualizado)
    }

    pub async fn eliminar_curso(&self, id: i32) -> Result<(), AppError> {
        let db = self.connection();
        let mut txn = db.begin().await.map_err(map_db_err)?;

        let mut curso = Curso::find_by_id(id)
            .one(&mut txn)
            .await
            .map_err(map_db_err)?
            .ok_or_else(|| AppError::NotFound("Curso no encontrado".into()))?;

        curso.fecha_eliminacion = Some(Utc::now());
        let curso_activo: curso::ActiveModel = curso.into();
        curso_activo.update(&mut txn).await.map_err(map_db_err)?;
        
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

        // Obtener curso para incluirlo en la respuesta
        let curso = Curso::find_by_id(id)
            .one(&db)
            .await
            .map_err(map_db_err)?
            .ok_or_else(|| AppError::NotFound("Curso no encontrado".into()))?;

        // Obtener todos los temas de los módulos que pertenecen al curso dado
        let temas: Vec<TemaModel> = Tema::find()
            .join(
                sea_orm::JoinType::InnerJoin,
                tema::Relation::Modulo.def(),
            )
            .filter(modulo::Column::CursoId.eq(id))
            .order_by(tema::Column::Orden, Order::Asc)
            .all(&db)
            .await
            .map_err(map_db_err)?;

        let tema_ids: Vec<i32> = temas.iter().map(|t| t.id).collect();

        let unidades: Vec<UnidadModel> = if tema_ids.is_empty() {
            Vec::new()
        } else {
            Unidad::find()
                .filter(unidad::Column::TemaId.is_in(tema_ids.clone()))
                .order_by(unidad::Column::Orden, Order::Asc)
                .all(&db)
                .await
                .map_err(map_db_err)?
        };

        let mut unidades_por_tema: std::collections::HashMap<i32, Vec<UnidadAula>> =
            std::collections::HashMap::new();

        for u in unidades {
            let entry = unidades_por_tema.entry(u.tema_id).or_default();
            entry.push(UnidadAula {
                id: u.id,
                nombre: u.nombre,
                descripcion: u.descripcion,
                orden: Some(u.orden),
                tema_id: u.tema_id,
            });
        }

        let temas_aula: Vec<TemaAula> = temas
            .into_iter()
            .map(|t| {
                let unidades = unidades_por_tema.remove(&t.id).unwrap_or_default();
                TemaAula {
                    id: t.id,
                    nombre: t.nombre,
                    descripcion: t.descripcion,
                    orden: Some(t.orden),
                    unidades,
                }
            })
            .collect();

        Ok(AulaCurso { curso, temas: temas_aula })
    }
}

fn map_db_err(err: DbErr) -> AppError {
    AppError::InternalServerError(err.to_string().into())
}
