use axum::{
    extract::{FromRef, Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;

use crate::{
    middleware::auth::AuthUser,
    models::{
        curso::Model as CursoModel,
        AppState,
    },
    services::curso_service::{
        ActualizarCurso,
        AulaCurso,
        CursoDetallado,
        CursoService,
        NuevoCurso,
    },
    utils::errors::AppError,
};

#[derive(Deserialize)]
pub struct CursoPeriodoQuery {
    pub periodo: String,
}

pub async fn listar_cursos(
    _auth_user: AuthUser,  // Validar JWT automáticamente
    State(state): State<AppState>,
) -> Result<Json<Vec<CursoDetallado>>, AppError> {
    let service = CursoService::from_ref(&state);
    let cursos = service.obtener_cursos().await?;
    Ok(Json(cursos))
}

pub async fn obtener_curso(
    _auth_user: AuthUser,  // Validar JWT automáticamente
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<CursoDetallado>, AppError> {
    let service = CursoService::from_ref(&state);
    let curso = service.obtener_curso_por_id(id).await?;
    Ok(Json(curso))
}

pub async fn crear_curso(
    _auth_user: AuthUser,  // Validar JWT automáticamente
    State(state): State<AppState>,
    Json(payload): Json<NuevoCurso>,
) -> Result<(StatusCode, Json<CursoModel>), AppError> {
    let service = CursoService::from_ref(&state);
    let curso = service.crear_curso(payload).await?;
    Ok((StatusCode::CREATED, Json(curso)))
}

pub async fn actualizar_curso(
    _auth_user: AuthUser,  // Validar JWT automáticamente
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<ActualizarCurso>,
) -> Result<Json<CursoModel>, AppError> {
    let service = CursoService::from_ref(&state);
    let curso = service.editar_curso(id, payload).await?;
    Ok(Json(curso))
}

pub async fn eliminar_curso(
    _auth_user: AuthUser,  // Validar JWT automáticamente
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let service = CursoService::from_ref(&state);
    service.eliminar_curso(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn cursos_por_plantilla(
    _auth_user: AuthUser,  // Validar JWT automáticamente
    State(state): State<AppState>,
    Path(plantilla_id): Path<i32>,
) -> Result<Json<Vec<CursoModel>>, AppError> {
    let service = CursoService::from_ref(&state);
    let cursos = service.obtener_cursos_por_plantilla(plantilla_id).await?;
    Ok(Json(cursos))
}

pub async fn cursos_por_area_y_periodo(
    _auth_user: AuthUser,  // Validar JWT automáticamente
    State(state): State<AppState>,
    Path(area_id): Path<i32>,
    Query(params): Query<CursoPeriodoQuery>,
) -> Result<Json<Vec<CursoDetallado>>, AppError> {
    let service = CursoService::from_ref(&state);
    let cursos = service
        .obtener_cursos_por_area_y_periodo(area_id, &params.periodo)
        .await?;
    Ok(Json(cursos))
}

pub async fn aula_por_curso(
    _auth_user: AuthUser,  // Validar JWT automáticamente
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<AulaCurso>, AppError> {
    let service = CursoService::from_ref(&state);
    let aula = service.obtener_aula_por_curso_id(id).await?;
    Ok(Json(aula))
}
