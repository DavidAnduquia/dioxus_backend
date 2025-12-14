# TODO Backend - Cobertura Completa de BD v2 (Cursos + Webinars)

Objetivo: listar TODO lo que falta en el backend para **usar o exponer todas las tablas** definidas en `ddl_postgresql_aula_v2.sql`, organizado por dominio y con prioridades.

---

## 1. Usuarios y Roles

Tablas: `roles`, `usuarios`.

### 1.1 Roles
- [ ] **Endpoint CRUD de roles (baja prioridad)**
  - `GET /roles`
  - `POST /roles`
  - `PUT /roles/{id}`
  - `DELETE /roles/{id}` (solo si se requiere UI de administración completa).
- [ ] **Validación de uso de roles en todo el backend**
  - Asegurar que los nombre de rol usados en código ("Coordinador", "Profesor", "Estudiante") coinciden con la BD.

### 1.2 Usuarios
- [ ] **Modelo `Usuario` completo** alineado con columnas:
  - Incluir campos: `fecha_ultima_conexion`, `token_primer_ingreso`, `estado`, `fecha_eliminacion`.
- [ ] **Servicio de usuarios**
  - [ ] Función para obtener solo usuarios activos (`estado = true` y `fecha_eliminacion IS NULL`).
  - [ ] Actualizar `fecha_ultima_conexion` al loguearse.
  - [ ] Endpoint para desactivar usuario mediante `fecha_eliminacion` (soft delete).
- [ ] **Autenticación**
  - [ ] Confirmar uso de `correo` + contraseña (bcrypt) y devolución de JWT.
  - [ ] Endpoint `GET /me` para obtener perfil del usuario autenticado.

---

## 2. Cursos y Estructura Académica

Tablas: `areas_conocimiento`, `cursos`, `plantillas_cursos`, `historial_cursos_estudiantes`, `profesores_curso`.

### 2.1 Áreas de conocimiento
- [ ] Endpoint(s):
  - `GET /areas-conocimiento` (lista todas, usando índice `areas_conocimiento_nombre_key`).
  - (Opcional) CRUD completo si se requiere administración por UI.

### 2.2 Cursos
- [ ] **Modelo de curso** con todos los campos:
  - `semestre`, `periodo`, `anio_pensum`, `area_conocimiento_id`, `fecha_eliminacion`, `fecha_actualizacion`.
- [ ] **Endpoints curso**
  - `GET /cursos` con filtros opcionales: área, periodo, semestre, estado (activo/borrado).
  - `GET /cursos/{id}` (detalle simple).
  - `POST /cursos` / `PUT /cursos/{id}` (crear/editar).
  - `DELETE /cursos/{id}` → setear `fecha_eliminacion`.
- [ ] **Endpoint de aula**
  - `GET /cursos/{id}/aula` → devuelve árbol:
    - curso + módulos + temas + unidades + contenidos.

### 2.3 Plantillas de cursos
- [ ] Endpoints:
  - `GET /plantillas-cursos`.
  - `POST /cursos/{id}/plantillas` (crear plantilla desde curso base).
  - `POST /plantillas-cursos/{id}/instanciar` (crear nuevo curso desde plantilla).

### 2.4 Historial de cursos de estudiantes
- [ ] Endpoints:
  - `POST /cursos/{id}/inscribir/{id_estudiante}` → inserta en `historial_cursos_estudiantes`.
  - `PATCH /cursos/{id}/estudiantes/{id_estudiante}` → actualiza `estado`, `calificacion_final`, `aprobado`.
  - `GET /estudiantes/{id_estudiante}/historial` → lista sus cursos, estados y notas.

### 2.5 Profesores por curso
- [ ] Endpoints:
  - `POST /cursos/{id}/profesores/{id_profesor}` → inserta en `profesores_curso`.
  - `GET /cursos/{id}/profesores`.
  - `GET /profesores/{id_profesor}/cursos`.

---

## 3. Estructura de Contenido (Módulos, Temas, Unidades, Contenidos)

Tablas: `modulos`, `temas`, `unidades`, `actividades_entrega`, `entregas`, `contenidos_unidad`.

### 3.1 Módulos
- [ ] Modelo `Modulo` con campos de fechas, `duracion_estimada`, `obligatorio`, `fecha_eliminacion`.
- [ ] Endpoints:
  - `GET /cursos/{id}/modulos` (ordenados por `orden`).
  - `POST /cursos/{id}/modulos`.
  - `PUT /modulos/{id}` (editar nombre, descripción, fechas, tipo, visibilidad, orden).
  - `DELETE /modulos/{id}` → usar `fecha_eliminacion`.

### 3.2 Temas y Unidades
- [ ] Endpoints para `temas`:
  - `GET /modulos/{id}/temas`.
  - `POST /modulos/{id}/temas`.
  - `PUT /temas/{id}` / `DELETE /temas/{id}`.
- [ ] Endpoints para `unidades`:
  - `GET /modulos/{id}/unidades`.
  - `POST /modulos/{id}/unidades`.
  - `PUT /unidades/{id}` / `DELETE /unidades/{id}`.

### 3.3 Contenidos de unidad
- [ ] Endpoint(s):
  - `GET /unidades/{id}/contenidos`.
  - `POST /unidades/{id}/contenidos` (crear contenido de tipo texto/archivo/video/quiz/actividad_entrega).
  - `PUT /contenidos-unidad/{id}` (editar título, descripción, orden, URLs, etc.).
  - `DELETE /contenidos-unidad/{id}`.

### 3.4 Actividades de entrega y entregas
- [ ] Endpoints para `actividades_entrega`:
  - `GET /unidades/{id}/actividades-entrega`.
  - `POST /unidades/{id}/actividades-entrega`.
  - `PUT /actividades-entrega/{id}`.
  - `DELETE /actividades-entrega/{id}`.
- [ ] Endpoints para `entregas`:
  - `GET /actividades-entrega/{id}/entregas`.
  - `POST /actividades-entrega/{id}/entregas` (subida de entregas, probablemente con integración a almacenamiento de archivos).
  - `PATCH /entregas/{id}` para calificar (`calificacion`, `comentario_profesor`, `estado`).

---

## 4. Evaluaciones (Unificadas y Legacy)

Tablas: `evaluaciones`, `banco_preguntas`, `preguntas_evaluacion`, `calificaciones_evaluacion` + legacy (`actividades`, `calificaciones`, `examenes`, `preguntas_examen`, `sesiones_curso`, `evaluaciones_sesion`, `evaluaciones_calificacion`).

### 4.1 Definir estrategia de uso
- [ ] Tomar decisión explícita:
  - **Opción recomendada:** usar **solo** `evaluaciones` + `banco_preguntas` + `preguntas_evaluacion` + `calificaciones_evaluacion` en nuevo código.
  - Legacy solo se usa para datos viejos o se migra.

### 4.2 Endpoints evaluaciones (nuevo sistema)
- [ ] `GET /cursos/{id}/evaluaciones`.
- [ ] `POST /cursos/{id}/evaluaciones`.
- [ ] `GET /evaluaciones/{id}` (detalle + preguntas).
- [ ] `PUT /evaluaciones/{id}`.
- [ ] `DELETE /evaluaciones/{id}` → usar `fecha_eliminacion`.

### 4.3 Banco de preguntas
- [ ] `GET /cursos/{id}/banco-preguntas` (con filtros por dificultad, etiquetas).
- [ ] `POST /cursos/{id}/banco-preguntas`.
- [ ] `PUT /banco-preguntas/{id}` / `DELETE /banco-preguntas/{id}`.

### 4.4 Composición de evaluaciones
- [ ] `POST /evaluaciones/{id}/preguntas` (asignar preguntas desde banco o crear ad-hoc).
- [ ] `PATCH /evaluaciones/{id}/preguntas/{id_pregunta}` (orden, puntaje, etc.).

### 4.5 Calificaciones de evaluaciones
- [ ] `POST /evaluaciones/{id}/calificaciones` (registro de intento/calificación).
- [ ] `GET /evaluaciones/{id}/calificaciones`.
- [ ] `GET /estudiantes/{id_estudiante}/evaluaciones` (historial de notas).

### 4.6 Legacy (si se mantiene)
- [ ] Aislar uso de `actividades`, `calificaciones`, `examenes`, `preguntas_examen`, `evaluaciones_sesion`, `evaluaciones_calificacion` en servicios separados o empezar plan de migración.

---

## 5. Webinars

Tablas: `webinars`, `webinar_modulos`, `webinar_progreso_estudiantes`, `personalizacion_webinar`.

### 5.1 Webinars
- [ ] Endpoints:
  - `GET /cursos/{id}/webinars`.
  - `POST /cursos/{id}/webinars`.
  - `GET /webinars/{id}`.
  - `PUT /webinars/{id}`.
  - `DELETE /webinars/{id}`.

### 5.2 Módulos de webinar
- [ ] Endpoints:
  - `GET /webinars/{id}/modulos`.
  - `POST /webinars/{id}/modulos`.
  - `PUT /webinar-modulos/{id}` (título, tipo_contenido, orden, URLs, duración_estimada, obligatorio).
  - `DELETE /webinar-modulos/{id}`.

### 5.3 Progreso de estudiantes en webinars
- [ ] Endpoints:
  - `GET /webinars/{id}/progreso/{id_estudiante}`.
  - `POST /webinars/{id}/progreso/{id_estudiante}` para actualizar:
    - `progreso_actual`, `modulos_completados`, `tiempo_total_visto`, `ultima_actividad`, `completado`, `fecha_completado`.

### 5.4 Personalización de webinar
- [ ] Endpoints:
  - `GET /webinars/{id}/personalizacion`.
  - `PUT /webinars/{id}/personalizacion` (leer/actualizar JSONB de estilos, orden y privacidad).

---

## 6. Portafolios y Contenido Transversal

Tablas: `contenido_transversal`, `portafolios`, `portafolio_contenidos`, `personalizacion_portafolio`.

### 6.1 Contenido transversal
- [ ] Endpoints:
  - `GET /cursos/{id}/contenido-transversal` (filtrar por origen_tipo si se desea).
  - `POST /cursos/{id}/contenido-transversal`.
  - `DELETE /contenido-transversal/{id}`.

### 6.2 Portafolios
- [ ] Endpoints:
  - `GET /cursos/{id}/portafolios`.
  - `POST /cursos/{id}/portafolios`.
  - `GET /portafolios/{id}`.
  - `DELETE /portafolios/{id}`.

### 6.3 Contenidos de portafolio
- [ ] Endpoints:
  - `GET /portafolios/{id}/contenidos`.
  - `POST /portafolios/{id}/contenidos` (asociar `contenido_transversal`).
  - `DELETE /portafolios/{id}/contenidos/{id_contenido}`.

### 6.4 Personalización de portafolio
- [ ] Endpoints:
  - `GET /portafolios/{id}/personalizacion`.
  - `PUT /portafolios/{id}/personalizacion`.

---

## 7. Gamificación

Tablas: `puntos_estudiante`, `logros`, `logros_estudiante`, `eventos_puntos`.

### 7.1 Catálogo de logros
- [ ] CRUD (opcional o solo admin):
  - `GET /logros`.
  - `POST /logros`.
  - `PUT /logros/{id}`.
  - `DELETE /logros/{id}`.

### 7.2 Puntos por estudiante
- [ ] Servicio de gamificación que:
  - A partir de ciertos eventos (entregar tarea, aprobar evaluación, finalizar webinar/curso) cree registros en `eventos_puntos`.
  - Actualice `puntos_estudiante` usando la función `calcular_nivel` cuando cambian los puntos.
- [ ] Endpoints:
  - `GET /cursos/{id}/puntos/{id_estudiante}`.
  - `GET /estudiantes/{id_estudiante}/puntos` (resumen global).

### 7.3 Logros por estudiante
- [ ] Lógica para conceder logros (`logros_estudiante`) basada en reglas de negocio.
- [ ] Endpoints:
  - `GET /estudiantes/{id_estudiante}/logros`.

---

## 8. Certificados

Tablas: `plantillas_certificado`, `certificados`.

### 8.1 Plantillas de certificado
- [ ] Endpoints (admin):
  - `GET /plantillas-certificado`.
  - `POST /plantillas-certificado`.
  - `PUT /plantillas-certificado/{id}`.
  - `DELETE /plantillas-certificado/{id}`.

### 8.2 Emisión y verificación de certificados
- [ ] Servicio que emite certificados cuando:
  - El estudiante aprueba un curso o cumple ciertas condiciones.
- [ ] Endpoints:
  - `POST /cursos/{id}/certificados/{id_estudiante}`.
  - `GET /certificados/verificar/{codigo_verificacion}`.
  - `GET /estudiantes/{id_estudiante}/certificados`.

---

## 9. Notificaciones y Eventos Programados

Tablas: `notificaciones`, `eventos_programados`.

### 9.1 Notificaciones
- [ ] Endpoints:
  - `GET /usuarios/{id}/notificaciones`.
  - `POST /notificaciones` (para crear desde el sistema/backend).
  - `PATCH /notificaciones/{id}` para marcar como leída (`leida = true`).

### 9.2 Eventos programados
- [ ] Definir uso concreto de `eventos_programados` (cron interno, integración con un scheduler externo, etc.).
- [ ] Endpoint opcional de lectura:
  - `GET /eventos-programados` (debug/administración).

---

## 10. Infraestructura, Rendimiento y Limpieza

### 10.1 Uso coherente de soft delete
- [ ] Revisar en servicios que las consultas:
  - Filtren por `fecha_eliminacion IS NULL` donde aplique.
  - No devuelvan entidades borradas salvo endpoints de administración.

### 10.2 Aprovechar índices
- [ ] Revisar queries más usadas y asegurarse de que utilizan columnas indexadas (`curso_id`, `estudiante_id`, `tipo`, `estado`, etc.).

### 10.3 Modelos Rust vs BD
- [ ] Alinear todos los modelos de `src/models/` con la BD:
  - Campos faltantes.
  - Tipos (`Option<>` solo cuando la columna permite `NULL`).
  - Enums (`tipo_evaluacion`, `tipo_pregunta`, estados) como enums de Rust.

### 10.4 Paginación y filtros
- [ ] Agregar paginación (`limit`/`offset`) en endpoints de listados grandes:
  - Cursos, usuarios, eventos de puntos, notificaciones, etc.

---

## 11. Prioridades Globales

**Alta prioridad:**
- Cobertura completa para: cursos + estructura de aula + usuarios + evaluaciones unificadas + webinars.

**Media prioridad:**
- Portafolios, contenido transversal, gamificación básica.

**Baja prioridad (pero valiosa):
- Logros avanzados, certificados automatizados, notificaciones ricas, paneles de administración completos.

Este documento sirve como mapa de trabajo para ir tachando tareas a medida que el backend vaya utilizando todas las tablas de la BD v2.
