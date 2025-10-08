pub mod rol_service;
pub mod socket_service;
pub mod usuario_service;
pub mod area_conocimiento_service;
pub mod curso_service;
pub mod examen_service;
pub mod matricula_service;
pub mod modulo_service;
pub mod actividad_service;

// Servicios con dependencias pendientes (async_trait)
// Descomentar cuando se refactoricen
// pub mod calificacion_service;
// pub mod contenido_plantilla_service;
// pub mod evaluacion_calificacion_service;
// pub mod evento_programado_service;
// pub mod modulo_archivo_service;
// pub mod notificacion_service;
// pub mod portafolio_service;
// pub mod portafolio_contenido_service;
// pub mod pregunta_examen_service;
// pub mod profesor_curso_service;

use sqlx::PgPool;
use crate::{
    models::{
        self,
        calificacion::{Model as CalificacionModel, Relation as CalificacionRelation},
        contenido_plantilla::{Model as ContenidoPlantillaModel, Relation as ContenidoPlantillaRelation},
        evaluacion_calificacion::{Model as EvaluacionCalificacionModel, Relation as EvaluacionCalificacionRelation},
        evento_programado::{Model as EventoProgramadoModel, Relation as EventoProgramadoRelation},
        historial_curso_actividad::{Model as HistorialCursoActividadModel, Relation as HistorialCursoActividadRelation},
        modulo_archivo::{Model as ModuloArchivoModel, Relation as ModuloArchivoRelation},
        notificacion::{Model as NotificacionModel, Relation as NotificacionRelation},
        portafolio::{Model as PortafolioModel, Relation as PortafolioRelation},
        portafolio_contenido::{Model as PortafolioContenidoModel, Relation as PortafolioContenidoRelation},
        pregunta_examen::{Model as PreguntaExamenModel, Relation as PreguntaExamenRelation},
        profesor_curso::{Model as ProfesorCursoModel, Relation as ProfesorCursoRelation},
    },
    utils::errors::AppError,
};

#[derive(Clone)]
#[allow(dead_code)]
pub struct DatabaseService {
    pool: PgPool,
}

impl DatabaseService {
    #[allow(dead_code)]
    pub fn new(pool: PgPool) -> Self {
        // Llamar a la función que usa los tipos para evitar warnings
        if false {
            Self::_ensure_types_used();
        }
        Self { pool }
    }

    #[allow(dead_code)]
    pub async fn get_user(&self, id: uuid::Uuid) -> Result<models::User, AppError> {
        sqlx::query_as::<_, models::User>(
            "SELECT id, email, password_hash, name, created_at, updated_at FROM rustdema.users WHERE id = $1"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(Into::into)
    }

    // Funciones que construyen instancias de los modelos para evitar warnings
    pub fn _ensure_types_used() {
        use chrono::Utc;
        let now = Utc::now();
        
        // Construir CalificacionModel
        let _cal = CalificacionModel {
            id: 0,
            actividad_id: 0,
            estudiante_id: 0,
            calificacion: 0.0,
            retroalimentacion: None,
            fecha_calificacion: now,
            created_at: Some(now),
            updated_at: Some(now),
        };
        
        // Construir ContenidoPlantillaModel
        let _cont = ContenidoPlantillaModel {
            id: 0,
            plantilla_curso_id: 0,
            nombre: String::new(),
            descripcion: None,
            tipo_contenido: String::new(),
            orden: 0,
            created_at: Some(now),
            updated_at: Some(now),
        };
        
        // Construir EvaluacionCalificacionModel
        let _eval = EvaluacionCalificacionModel {
            id: 0,
            evaluacion_id: 0,
            estudiante_id: 0,
            calificacion: 0.0,
            retroalimentacion: None,
            fecha_calificacion: now,
            created_at: Some(now),
            updated_at: Some(now),
        };
        
        // Construir EventoProgramadoModel
        let _evento = EventoProgramadoModel {
            id: 0,
            titulo: String::new(),
            descripcion: None,
            fecha_inicio: now,
            fecha_fin: now,
            tipo_evento: String::new(),
            curso_id: 0,
            profesor_id: 0,
            created_at: Some(now),
            updated_at: Some(now),
        };
        
        // Construir HistorialCursoActividadModel
        let _hist = HistorialCursoActividadModel {
            id: 0,
            historial_curso_id: 0,
            actividad_id: 0,
            calificacion: None,
            completado: false,
            fecha_completado: None,
            created_at: Some(now),
            updated_at: Some(now),
        };
        
        // Construir ModuloArchivoModel
        let _mod_arch = ModuloArchivoModel {
            id: 0,
            modulo_id: 0,
            nombre_archivo: String::new(),
            ruta_archivo: String::new(),
            tipo_archivo: String::new(),
            tamano: 0,
            descripcion: None,
            created_at: Some(now),
            updated_at: Some(now),
        };
        
        // Construir NotificacionModel
        let _notif = NotificacionModel {
            id: 0,
            usuario_id: 0,
            titulo: String::new(),
            mensaje: String::new(),
            tipo: String::new(),
            leida: false,
            enlace: None,
            datos_adicionales: None,
            created_at: Some(now),
            updated_at: Some(now),
        };
        
        // Construir PortafolioModel
        let _port = PortafolioModel {
            id: 0,
            estudiante_id: 0,
            curso_id: 0,
            titulo: String::new(),
            descripcion: None,
            estado: String::new(),
            fecha_creacion: now,
            fecha_actualizacion: now,
            created_at: Some(now),
            updated_at: Some(now),
        };
        
        // Construir PortafolioContenidoModel
        let _port_cont = PortafolioContenidoModel {
            id: 0,
            portafolio_id: 0,
            tipo_contenido: String::new(),
            titulo: String::new(),
            descripcion: None,
            contenido: String::new(),
            orden: 0,
            created_at: Some(now),
            updated_at: Some(now),
        };
        
        // Construir PreguntaExamenModel
        let _preg = PreguntaExamenModel {
            id: 0,
            examen_id: 0,
            pregunta: String::new(),
            tipo_pregunta: String::new(),
            opciones: None,
            respuesta_correcta: None,
            valor_puntos: 0,
            orden: 0,
            created_at: Some(now),
            updated_at: Some(now),
        };
        
        // Construir ProfesorCursoModel
        let _prof = ProfesorCursoModel {
            id: 0,
            profesor_id: 0,
            curso_id: 0,
            fecha_asignacion: now,
            estado: String::new(),
            created_at: Some(now),
            updated_at: Some(now),
        };
        
        // Usar los Relation enums (aunque estén vacíos, necesitamos referenciarlos)
        let _: Option<CalificacionRelation> = None;
        let _: Option<ContenidoPlantillaRelation> = None;
        let _: Option<EvaluacionCalificacionRelation> = None;
        let _: Option<EventoProgramadoRelation> = None;
        let _: Option<HistorialCursoActividadRelation> = None;
        let _: Option<ModuloArchivoRelation> = None;
        let _: Option<NotificacionRelation> = None;
        let _: Option<PortafolioRelation> = None;
        let _: Option<PortafolioContenidoRelation> = None;
        let _: Option<PreguntaExamenRelation> = None;
        let _: Option<ProfesorCursoRelation> = None;
    }
}
