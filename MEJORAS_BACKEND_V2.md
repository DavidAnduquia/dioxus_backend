# Mejora del Backend para Cursos y Webinars (Versión 2 de la BD)

## 1. Lo que YA tengo (estado actual)

### 1.1 Base de datos `ddl_postgresql_aula_v2.sql`

La versión 2 de la base de datos ya soporta un **ecosistema educativo completo**, centrado en cursos y webinars:

#### 1.1.1 Entidades base

- **Usuarios y roles**
  - `roles`: Coordinador, Profesor, Estudiante (y otros que quieras agregar).
  - `usuarios`: datos personales, correo único, rol, semestre, género, fechas de creación/actualización, estado, soft delete con `fecha_eliminacion`.
- **Áreas y cursos**
  - `areas_conocimiento`: clasifica los cursos por área (Matemáticas, Ciencias, etc.).
  - `cursos`:
    - Fechas de inicio/fin, prerequisitos.
    - `coordinador_id`, `area_conocimiento_id`.
    - Campos académicos: `semestre`, `periodo`, `anio_pensum`.
    - Soft delete: `fecha_eliminacion`.

- **Plantillas y asignación**
  - `plantillas_cursos`: permite tener cursos “base” para clonación.
  - `historial_cursos_estudiantes`: inscripción de estudiantes, estado (`en_progreso`, etc.), nota final, aprobado.
  - `profesores_curso`: profesores asignados a cada curso.

#### 1.1.2 Estructura de contenido del curso

- `modulos`: bloques grandes del curso (pueden ser estructura, taller, evaluación, etc.).
- `temas`: subdivisiones dentro de un módulo.
- `unidades`: subdivisión adicional dentro del módulo (en v2, cuelga de `modulos`).
- `contenidos_unidad`:
  - Tipos de contenido: `texto`, `archivo`, `video`, `quiz`, `actividad_entrega`.
  - Campos para:
    - Contenido textual (`texto_largo`).
    - Archivos (`archivo_url`, `archivo_tipo`).
    - Videos (`video_url`).
    - Vínculo a exámenes (`examen_id`) o entregas (`entrega_id`).

- `actividades_entrega`: tareas con fecha límite (`fecha_limite`) y tipo:
  - `entrega_obligatoria`
  - `entrega_opcional`
- `entregas`: subida de trabajos por parte de estudiantes.

#### 1.1.3 Evaluaciones

- **Sistema unificado (nuevo)**
  - `tipo_evaluacion` (ENUM): `cuestionario`, `examen`, `tarea`, `proyecto`, `discusion`.
  - `evaluaciones`:
    - Asociadas a `id_curso`.
    - Configuración JSON (`configuracion`), fechas de disponibilidad y límite, peso, puntaje máximo, estado (`borrador`, `publicado`, `archivado`).
  - `tipo_pregunta` (ENUM): `opcion_multiple`, `verdadero_falso`, `respuesta_corta`, `ensayo`, `archivo`.
  - `banco_preguntas`: preguntas reutilizables por curso.
  - `preguntas_evaluacion`: instancia de preguntas dentro de una evaluación.
  - `calificaciones_evaluacion`: nota por estudiante, con estado (`pendiente`, `calificado`, `en_progreso`).

- **Sistema legacy (compatibilidad)**
  - `actividades` + `calificaciones`.
  - `examenes` + `preguntas_examen`.
  - `sesiones_curso` + `evaluaciones_sesion` + `evaluaciones_calificacion`.

#### 1.1.4 Webinars

Ya hay soporte completo para webinars:

- `webinars`:
  - `curso_id`, `titulo`, `descripcion`, `progreso` (0–100), `estado` (`no_iniciado`, `en_progreso`, `completado`), `duracion`, `modulos`.
- `webinar_modulos`:
  - Estructura interna del webinar (tipo contenido: `video`, `presentacion`, `actividad`, `quiz`).
- `webinar_progreso_estudiantes`:
  - `progreso_actual`, `modulos_completados`, `tiempo_total_visto`, `ultima_actividad`, `completado`, `fecha_completado`.

- `personalizacion_webinar`:
  - `estilos`, `orden_componentes`, `privacidad_componentes` en JSONB.

#### 1.1.5 Portafolios y contenido transversal

- `contenido_transversal`: recursos ligados a curso/módulo/examen/portafolio.
- `portafolios`: colecciones de contenidos por curso.
- `portafolio_contenidos`: relación N:N entre portafolios y contenidos.
- `personalizacion_portafolio`: ajustes visuales para portafolios.

#### 1.1.6 Gamificación y certificados

- Gamificación:
  - `puntos_estudiante`: puntos totales, nivel, rachas.
  - `logros`: catálogo de logros.
  - `logros_estudiante`: logros obtenidos por estudiante y curso.
  - `eventos_puntos`: histórico de eventos de puntos.

- Certificados:
  - `plantillas_certificado`: HTML, imagen de fondo, configuraciones.
  - `certificados`: `id` UUID, `codigo_verificacion`, `numero_certificado`, fechas y estado.

#### 1.1.7 Infraestructura transversal

- `notificaciones`: mensajes por usuario.
- `eventos_programados`: para tareas automáticas.
- Funciones:
  - `actualizar_fecha_modificacion()` + triggers automáticos en muchas tablas.
  - `calcular_nivel(puntos)` para gamificación.
- Índices optimizados para:
  - Usuarios, cursos, módulos, actividades, evaluaciones, webinars, puntos, certificados, historial, etc.

---

## 2. Lo que TENGO QUE AGREGAR (faltantes a nivel de backend/código)

> La BD ya está muy completa. Lo que falta, sobre todo, es **código backend** (Rust) que exponga y utilice todo esto.

### 2.1 API de cursos y estructura de aula

- **Endpoints REST recomendados:**
  - `GET /cursos`
    - Filtros opcionales: `area_conocimiento`, `periodo`, `semestre`, `estado`.
  - `GET /cursos/{id}`
    - Devuelve datos básicos del curso + área + coordinador.
  - `GET /cursos/{id}/aula`
    - Devuelve **árbol completo**:
      - Curso
      - Módulos (ordenados)
      - Temas (por módulo, ordenados)
      - Unidades (por módulo, ordenadas)
      - Contenidos de cada unidad (ordenados).

  - `POST /cursos` / `PUT /cursos/{id}`
    - Crear/editar curso con campos académicos completos.
  - `POST /cursos/{id}/plantilla`
    - Crear/actualizar `plantillas_cursos` desde un curso base.

- **Inscripciones y progreso de curso:**
  - `POST /cursos/{id}/inscribir/{id_estudiante}`
    - Inserta en `historial_cursos_estudiantes`.
  - `PATCH /cursos/{id}/estudiantes/{id_estudiante}`
    - Actualiza:
      - `estado`
      - `calificacion_final`
      - `aprobado`.

### 2.2 API de módulos/temas/unidades/contenidos

- CRUD para:
  - `modulos`
  - `temas`
  - `unidades`
  - `contenidos_unidad`

- Operaciones útiles:
  - Cambiar orden (`orden`) dentro de:
    - Módulos de un curso.
    - Temas de un módulo.
    - Unidades de un módulo.
    - Contenidos de una unidad.

### 2.3 API de evaluaciones unificadas

- Endpoints recomendados:
  - `GET /cursos/{id}/evaluaciones`
  - `POST /cursos/{id}/evaluaciones`
  - `PUT /evaluaciones/{id}`
  - `DELETE (lógico) /evaluaciones/{id}` usando `fecha_eliminacion`.

- Banco de preguntas:
  - `GET /cursos/{id}/banco-preguntas`
  - `POST /cursos/{id}/banco-preguntas`
  - `PUT /banco-preguntas/{id}`

- Composición de evaluaciones:
  - `POST /evaluaciones/{id}/preguntas` (crea `preguntas_evaluacion`).
  - `PATCH /evaluaciones/{id}/preguntas/{id_pregunta}` (cambia orden, puntaje, etc.).

- Calificaciones:
  - `POST /evaluaciones/{id}/calificaciones`
  - `GET /evaluaciones/{id}/calificaciones`
  - Respetando `UNIQUE(id_evaluacion, id_estudiante)`.

### 2.4 API de webinars

- Endpoints clave:
  - `GET /cursos/{id}/webinars`
  - `POST /cursos/{id}/webinars`
  - `PUT /webinars/{id}`
  - `DELETE /webinars/{id}` (opcionalmente lógico).

- Módulos del webinar:
  - `GET /webinars/{id}/modulos`
  - `POST /webinars/{id}/modulos`
  - `PATCH /webinar-modulos/{id}` (título, tipo, orden, etc.).

- Progreso de estudiantes:
  - `GET /webinars/{id}/progreso/{id_estudiante}`
  - `POST /webinars/{id}/progreso/{id_estudiante}`
    - Actualizar:
      - `progreso_actual`
      - `modulos_completados`
      - `tiempo_total_visto`
      - `ultima_actividad`
      - marcar `completado` + `fecha_completado`.

- Personalización:
  - `GET /webinars/{id}/personalizacion`
  - `PUT /webinars/{id}/personalizacion`
    - Operar sobre `personalizacion_webinar` (JSONB).

### 2.5 API de portafolios y contenido transversal (opcional pero útil)

- Portafolios:
  - `GET /cursos/{id}/portafolios`
  - `POST /cursos/{id}/portafolios`
  - `GET /portafolios/{id}/contenidos`

- Contenido transversal:
  - `POST /cursos/{id}/contenido-transversal`
  - `GET /cursos/{id}/contenido-transversal`

### 2.6 Servicios de gamificación y certificados

- Gamificación:
  - Servicio que, en ciertas operaciones (entrega, completar webinar, aprobar evaluación):
    - Inserte en `eventos_puntos`.
    - Actualice `puntos_estudiante`.
    - Use `calcular_nivel(puntos)` para ajustar `nivel`.
  - Endpoints:
    - `GET /cursos/{id}/puntos/{id_estudiante}`
    - `GET /cursos/{id}/logros/{id_estudiante}`

- Certificados:
  - Servicio que:
    - Verifique condiciones (curso aprobado, etc.).
    - Inserte en `certificados` con `codigo_verificacion` y `numero_certificado`.
  - Endpoints:
    - `POST /cursos/{id}/certificados/{id_estudiante}`
    - `GET /certificados/verificar/{codigo_verificacion}`

---

## 3. Lo que TENGO QUE CORREGIR / UNIFICAR

### 3.1 Duplicidad entre sistemas de evaluación

Actualmente conviven:

- **Nuevo sistema**: `evaluaciones`, `banco_preguntas`, `preguntas_evaluacion`, `calificaciones_evaluacion`.
- **Legacy**: `actividades`, `calificaciones`, `examenes`, `preguntas_examen`, `evaluaciones_sesion`, `evaluaciones_calificacion`.

**Correcciones recomendadas:**

- Definir una regla clara:
  - **Nuevo desarrollo** debe usar solo el sistema unificado (`evaluaciones` + `banco_preguntas`).
  - Legacy solo se mantiene si hay datos antiguos que no puedas migrar aún.
- En el código:
  - Evitar mezclar `actividades` / `examenes` con `evaluaciones` en nuevas APIs.
  - Planear, a futuro, una migración de datos legacy a la estructura nueva.

### 3.2 Normalizar uso de soft delete

Hay campos como `fecha_eliminacion` y campos `estado` (activo/inactivo).

**Correcciones:**

- Acordar criterios de borrado:
  - Cursos y módulos: preferible usar `fecha_eliminacion` en vez de borrar físicamente.
  - Endpoints de borrado deben:
    - Actualizar `fecha_eliminacion`.
    - Ajustar consultas para filtrar `WHERE fecha_eliminacion IS NULL`.

### 3.3 Coherencia de nombres y tipos entre BD y código

Ejemplos típicos:

- En Rust, asegurarse de que:
  - Campos ENUM (`tipo_evaluacion`, `tipo_pregunta`) se modelen como enums nativos.
  - Campos JSONB (`configuracion`, `estilos`, `orden_componentes`, etc.) tengan structs/`serde_json::Value` bien definidos.
- Revisar que los modelos actuales (`curso.rs`, `webinar.rs`, etc.):
  - Incluyan todos los campos relevantes de la BD.
  - No usen nombres desactualizados de columnas.

### 3.4 Uso consistente de índices y triggers

- Ya existen índices e triggers en la BD, pero el código:
  - No siempre aprovecha filtrado por campos indexados (`estado`, `fecha_eliminacion`, etc.).
- Corrección:
  - Ajustar queries para usar filtros alineados con los índices creados (por ejemplo, filtrar por `estado` y `fecha_eliminacion` donde haya índices parciales).

---

## 4. Resumen final

- **Lo que ya tengo:**  
  Una BD v2 muy completa que soporta:
  - Cursos semestrales con estructura rica (módulos, temas, unidades, contenidos).
  - Evaluaciones unificadas.
  - Webinars y progreso de estudiantes.
  - Portafolios, gamificación y certificados.
  - Notificaciones y eventos programados.

- **Lo que tengo que agregar (backend):**
  - Endpoints REST claros para:
    - Cursos + estructura de aula.
    - Inscripciones y progreso.
    - Evaluaciones unificadas.
    - Webinars (estructura + progreso).
    - Portafolios, puntos y certificados (según prioridad).
  - Modelos y servicios en Rust que usen todas estas tablas.

- **Lo que tengo que corregir/unificar:**
  - Decidir y usar solo el sistema de evaluaciones unificado en nuevo código.
  - Normalizar soft delete y filtrado de registros.
  - Alinear completamente modelos Rust con columnas y tipos de la BD.
  - Aprovechar índices y triggers existentes en las consultas.

Con este documento puedo planear, por módulos, los cambios en el backend para que el frontend (aula + webinars) consuma una API consistente y totalmente alineada con la versión 2 de la base de datos.
