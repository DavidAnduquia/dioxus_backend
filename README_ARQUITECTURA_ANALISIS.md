# Análisis Arquitectónico y de Implementación del Backend Aula Virtual

## 1. Visión General del Proyecto

- **Lenguaje:** Rust.
- **Framework web:** Axum (REST API).
- **Base de datos:** PostgreSQL (schema `rustdema2`, DDL en `src/database/ddl_postgresql_aula_v2.sql`).
- **ORM / acceso a datos:** SQLx (consultas tipadas y asíncronas).
- **Autenticación:** JWT + bcrypt (seguridad basada en tokens).
- **Configuración:** Variables de entorno con `dotenvy` u otro crate similar.
- **Logging/Tracing:** `tracing` para logs estructurados.
- **Arquitectura:** API REST modularizada por dominios (usuarios, cursos, portafolios, webinars, etc.), con separación entre modelos, servicios y rutas.

Este backend está diseñado como **plataforma educativa genérica**, capaz de soportar:

- Cursos estructurados por módulos/temas/unidades.
- Evaluaciones unificadas (quices, exámenes, tareas, proyectos, discusiones).
- Webinars con progreso por estudiante.
- Portafolios y contenido transversal reutilizable.
- Gamificación (puntos, logros, rachas).
- Certificados digitales.

---

## 2. Organización del Código y Capas

Aunque la estructura exacta puede variar, la organización típica en `backend-aula` sigue un patrón por capas:

- `src/models/`
  - Definiciones de estructuras de datos (modelos) que representan tablas de BD y DTOs de entrada/salida.
  - Ejemplos: `usuario.rs`, `curso.rs`, `webinar.rs`, `portafolio.rs`, `webinar_modulo.rs`, `webinar_progreso_estudiante.rs`, `personalizacion_portafolio.rs`, etc.

- `src/services/`
  - Lógica de negocio y acceso a datos (queries SQLx, transformaciones, validaciones adicionales).
  - Ejemplos: `usuario_service.rs`, `curso_service.rs`, `portafolio_service.rs`, `portafolio_contenido_service.rs`, `personalizacion_portafolio_service.rs`, `webinar_service` (si existe o previsto).

- `src/routes/` o módulo raíz con rutas Axum
  - Definición de endpoints (handlers) que exponen la API REST.
  - Cada handler delega la lógica principal al servicio correspondiente.

- `src/database/`
  - Scripts SQL: `ddl_postgresql_aula_v2.sql` (schema definitivo v2) y posiblemente migraciones.
  - Funciones utilitarias para conexión (pool de SQLx) y ejecución de migraciones.

- `src/main.rs`
  - Punto de entrada de la aplicación.
  - Configuración de:
    - Router Axum (rutas, middlewares, CORS).
    - Pool de conexiones SQLx.
    - Capas de tracing/logging.
    - Carga de configuración desde entorno.

Este diseño sigue una arquitectura **en capas**:

- **Capa de Presentación (HTTP):** handlers Axum.
- **Capa de Aplicación/Servicios:** `*_service.rs`.
- **Capa de Dominio/Modelo:** structs en `models/`.
- **Capa de Infraestructura:** acceso a PostgreSQL vía SQLx, configuración, logging, autenticación.

---

## 3. Diseño de la Base de Datos (v2) y su Impacto en la Arquitectura

El DDL `ddl_postgresql_aula_v2.sql` define el corazón del backend. Conceptualmente, el dominio se puede agrupar en:

### 3.1 Gestión de Usuarios y Roles

- **Tablas clave:** `roles`, `usuarios`.
- **Patrones usados:**
  - Validación a nivel de BD con `CHECK` en campos como `genero`.
  - `UNIQUE` en `correo` y `documento_nit` para garantizar identidad.
  - Soft delete con `fecha_eliminacion`.
  - Índices (`idx_usuarios_correo`, `idx_usuarios_rol_id`, `idx_usuarios_activos`).

- **Implicaciones en código:**
  - Servicios de usuario deben siempre filtrar por `estado = true` y `fecha_eliminacion IS NULL` para evitar traer usuarios desactivados.
  - En autenticación, el lookup por correo está optimizado por índice.

### 3.2 Cursos y Estructura de Contenido

- **Tablas clave:**
  - `areas_conocimiento`, `cursos`, `plantillas_cursos`, `historial_cursos_estudiantes`, `profesores_curso`.
  - `modulos`, `temas`, `unidades`, `contenidos_unidad`, `actividades_entrega`, `entregas`.

- **Modelo lógico:**
  - Un curso pertenece a un área de conocimiento y a un coordinador.
  - Un curso se organiza en módulos → temas → unidades.
  - Cada unidad tiene uno o varios contenidos (texto, archivo, video, quiz, entrega).
  - Existen actividades de entrega con entregas asociadas por estudiante.

- **Patrones y buenas prácticas:**
  - Relaciones claras 1:N y N:1 con restricciones `ON DELETE CASCADE` para integridad.
  - Índices por claves foráneas (`idx_modulos_curso`, `idx_temas_modulo`, `idx_unidades_modulo`, etc.).
  - Campos `orden` para controlar el flujo de aprendizaje sin depender de IDs.

- **Implicaciones arquitectónicas:**
  - Servicios deben construir respuestas jerárquicas (curso → módulos → temas → unidades → contenidos) para el frontend, pero internamente operar con consultas eficientes por cada nivel.
  - Posible patrón **Facade** desde `curso_service` que orquesta llamadas a otros servicios (módulos, temas, unidades) para devolver el "aula" completa.

### 3.3 Evaluaciones Unificadas

- **Nuevo sistema:** `evaluaciones`, `banco_preguntas`, `preguntas_evaluacion`, `calificaciones_evaluacion`, `tipo_evaluacion`, `tipo_pregunta`.
- **Legacy:** `actividades`, `calificaciones`, `examenes`, `preguntas_examen`, `sesiones_curso`, `evaluaciones_sesion`, `evaluaciones_calificacion`.

- **Estrategia de diseño:**
  - Un solo modelo de evaluación (`evaluaciones`) parametrizable por tipo y configuración JSON (`configuracion`).
  - Banco de preguntas reutilizables por curso y tipificadas por dificultad, etiquetas, etc.
  - Tabla de respuestas/calificaciones con `UNIQUE (id_evaluacion, id_estudiante)` para evitar duplicidades.

- **Recomendación arquitectónica:**
  - Migrar la lógica de negocio hacia el sistema unificado y usar el legacy solo para compatibilidad, reduciendo complejidad.
  - En servicios, aplicar un patrón **Strategy** para manejar diferentes `tipo_evaluacion` si la lógica entre `quiz`, `tarea`, `proyecto` se vuelve muy específica.

### 3.4 Webinars

- **Tablas:** `webinars`, `webinar_modulos`, `webinar_progreso_estudiantes`, `personalizacion_webinar`.

- **Modelo:**
  - Webinars asociados a cursos, con módulos internos y progreso por estudiante.
  - Personalización de layout guardada como JSON (ideal para UI dinámica en frontend).

- **Impacto en el backend:**
  - Servicios de webinars deben manejar tanto estructura (definición del webinar) como estado (progreso de estudiantes).
  - Puede aplicarse un patrón **Repository** o servicios dedicados (`webinar_service`) para aislar la lógica de webinars y mantener el código del curso limpio.

### 3.5 Portafolios y Contenido Transversal

- **Tablas:** `contenido_transversal`, `portafolios`, `portafolio_contenidos`, `personalizacion_portafolio`.

- **Uso típico:**
  - Repositorio central de recursos que pueden apuntar a distintos orígenes (`curso`, `examen`, `modulo`, `portafolio`).
  - Portafolios como colecciones temáticas de evidencias por curso.

- **Patrones:**
  - Modelo similar a un **Tagging System** (N:N) a través de `portafolio_contenidos`.
  - JSONB para personalización flexible sin alterar el schema frecuentemente.

### 3.6 Gamificación y Certificados

- **Tablas:** `puntos_estudiante`, `logros`, `logros_estudiante`, `eventos_puntos`, `plantillas_certificado`, `certificados`.

- **Diseño:**
  - `eventos_puntos` como log append-only que alimenta el estado de `puntos_estudiante`.
  - Función `calcular_nivel(puntos)` que encapsula la lógica de nivel en la BD (posible mezcla de lógica en BD + aplicativa).
  - Múltiples plantillas de certificado con HTML/CSS y variables JSON.

- **Impacto en arquitectura:**
  - Buen candidato para un **servicio de dominio independiente** (por ejemplo, `gamificacion_service.rs` y `certificados_service.rs`).
  - Puede integrarse con eventos de otras partes del sistema (finalizar curso, aprobar evaluación, completar webinar).

---

## 4. Patrones de Diseño y Estrategias de Arquitectura

### 4.1 Patrón en capas (Layered Architecture)

- **Capa HTTP (Presentación):** Rutas Axum (handlers) que hacen:
  - Parse de requests, validación básica.
  - Manejo de autenticación/autorización con middlewares (JWT, roles).
  - Devolución de respuestas JSON estándar.

- **Capa de Servicios (Aplicación):**
  - Encapsula la lógica de negocio por dominio: usuarios, cursos, evaluaciones, webinars, portafolios, gamificación.
  - Centraliza las reglas (por ejemplo: cuándo se emite un certificado, cómo se calcula el progreso de un curso).

- **Capa de Dominio/Modelos:**
  - Modelos Rust que representan entidades de negocio y DTOs para la API.
  - Se busca que esta capa no dependa de Axum directamente (separación de preocupaciones).

- **Capa de Infraestructura:**
  - Conexión a PostgreSQL (SQLx), logging, configuración, seguridad.

Este patrón favorece **testabilidad**, **mantenibilidad** y un acoplamiento bajo entre web framework y lógica de negocio.

### 4.2 Uso de patrones específicos

- **Singleton / Global state controlado:**
  - El pool de conexiones SQLx y la configuración global se exponen normalmente como estado compartido (por ejemplo en `Extension` de Axum). 
  - Funciona como un "Singleton controlado": una única instancia de pool/configuración, pero inyectada vía DI (dependency injection) en los handlers.

- **Repository / Service:**
  - Los servicios (`*_service.rs`) actúan como Repositories + capa de aplicación.
  - Aislando queries SQLx allí, el resto del código no necesita conocer detalles de SQL o de la BD.

- **Strategy (potencial):**
  - Para diferentes tipos de evaluación (`tipo_evaluacion`) o actividades de gamificación, se pueden implementar estrategias concretas (por ejemplo, cálculo de puntuación, criterios de aprobación).

- **Facade (potencial):**
  - Un servicio de "aula" que construya toda la estructura de curso, combinando varios servicios internos (módulos, unidades, evaluaciones, webinars).

### 4.3 Eficiencia y Rendimiento

La arquitectura ya incorpora varias decisiones que mejoran el rendimiento:

- Índices en casi todas las claves foráneas y campos de filtrado frecuentes.
- Índices parciales para entidades activas (por ejemplo `idx_usuarios_activos`, `idx_cursos_activos`).
- Triggers automáticos que mantienen `fecha_actualizacion` sin tener que gestionarlo siempre en código.

**Recomendaciones adicionales:**

- Usar queries SQLx que aprovechen los índices:
  - Filtrar explícitamente por campos indexados (`estado`, `fecha_eliminacion`, `curso_id`, etc.).
- Evitar N+1 queries en servicios que construyen estructuras grandes (por ejemplo, cargar módulos y temas):
  - Usar `JOIN` o consultas por lote cuando corresponda.
  - O bien cargar por nivel, pero con caching razonable si es necesario.
- Revisar el uso de `SELECT *` y preferir seleccionar sólo columnas necesarias en endpoints críticos.

### 4.4 Optimización de memoria y uso de variables

- **Inicialización ligera:**
  - En `main.rs`, sólo se debe inicializar:
    - Configuración.
    - Pool de BD.
    - Router Axum y middlewares.
  - Evitar cargar grandes estructuras en memoria global en el arranque (por ejemplo, catálogos masivos) salvo que estén muy justificados.

- **Uso de tipos eficientes:**
  - Reutilizar estructuras compartidas (por ejemplo, `Arc` para configuraciones compartidas y clientes), en lugar de clonarlas excesivamente.
  - En modelos, usar tipos adecuados (`i32`/`i64`, `Option<T>` sólo cuando realmente pueda ser `NULL`, etc.) para evitar overhead.

- **Streaming y paginación:**
  - Para listados grandes (cursos, estudiantes, log de eventos de puntos), usar:
    - Paginación (`limit`/`offset`).
    - O cursores para evitar cargar todo en memoria.

---

## 5. Funcionalidades Programadas (Resumen por Dominios)

> Basado en el schema y la estructura de código, estos son los dominios funcionales que el backend está preparado para manejar. Algunos pueden estar parcialmente implementados y otros listos a nivel de BD.

### 5.1 Usuarios y Autenticación

- Registro y gestión de usuarios (`usuarios`).
- Roles (`roles`) para separación de permisos: Coordinador, Profesor, Estudiante.
- Autenticación vía JWT (inicio de sesión, verificación de token).
- Protección de rutas mediante middlewares de autenticación/autorización.

### 5.2 Gestión de Cursos

- Creación y actualización de cursos con:
  - Nombre, descripción, fechas, coordinador, área, periodo, semestre.
- Asignación de profesores a cursos (`profesores_curso`).
- Inscripción de estudiantes (`historial_cursos_estudiantes`), con estado y nota final.

### 5.3 Estructura de Aula

- Definición de módulos, temas y unidades por curso.
- Asociación de contenidos a unidades (`contenidos_unidad`): texto, archivos, videos, evaluaciones/entregas.
- Actividades de entrega y gestión de entregas de estudiantes.

### 5.4 Evaluaciones

- Sistema unificado de evaluaciones:
  - Creación de evaluaciones (`evaluaciones`) con tipo, configuración y fechas.
  - Banco de preguntas (`banco_preguntas`).
  - Composición de evaluaciones (`preguntas_evaluacion`).
  - Registro de calificaciones (`calificaciones_evaluacion`).

- Sistema legacy (si aún se usa): actividades, exámenes por separado y evaluaciones por sesión.

### 5.5 Webinars

- Definición de webinars por curso, con título, descripción, estado y progreso.
- Módulos de webinar con tipos de contenido variados.
- Registro de progreso del estudiante en cada webinar (`webinar_progreso_estudiantes`).
- Personalización visual de webinars (`personalizacion_webinar`).

### 5.6 Portafolios y Contenido Transversal

- Contenido reutilizable (`contenido_transversal`) asociado a cursos/módulos/exámenes/portafolios.
- Creación y mantenimiento de portafolios (`portafolios`).
- Asociación de contenidos a portafolios (`portafolio_contenidos`).
- Personalización de portafolios (`personalizacion_portafolio`).

### 5.7 Gamificación

- Registro de puntos por estudiante y curso (`puntos_estudiante`).
- Logros disponibles (`logros`) y logros alcanzados (`logros_estudiante`).
- Eventos de puntos (`eventos_puntos`) que permiten auditar cómo se acumulan puntos.
- Función de cálculo de nivel en la BD.

### 5.8 Certificados

- Definición de plantillas de certificados (`plantillas_certificado`).
- Emisión de certificados para estudiantes (`certificados`) con:
  - `codigo_verificacion` único.
  - `numero_certificado` único.
  - Metadatos y URL de PDF.

### 5.9 Notificaciones y Eventos Programados

- Notificaciones a usuarios (`notificaciones`) por acciones del sistema.
- Eventos programados (`eventos_programados`) para tareas automáticas (cron jobs u orquestación externa).

---

## 6. Conclusión y Madurez del Diseño

- La **base de datos v2** está diseñada como una plataforma robusta y extensible para entornos educativos complejos.
- El backend en Rust con Axum y SQLx es una elección muy sólida para:
  - Alto rendimiento.
  - Buen control de memoria y concurrencia.
  - Seguridad fuerte (tipos, lifetimes, ownership).
- A nivel de arquitectura:
  - La separación por modelos, servicios y rutas facilita el crecimiento del proyecto.
  - El uso de JSONB y ENUMs en PostgreSQL permite flexibilidad sin perder integridad.

Los siguientes pasos recomendables son:

- Consolidar el uso del sistema de evaluaciones unificado.
- Completar servicios y endpoints para webinars y estructura de aula (curso completo).
- Introducir pruebas automatizadas (unitarias e integración) sobre servicios clave.
- Monitorizar consultas pesadas y optimizarlas usando índices y estrategias de carga adecuadas.

Con estas acciones, el backend estará listo para soportar de forma eficiente la expansión funcional del frontend (aula, gestión de cursos, dashboards de webinars y progreso de estudiantes) manteniendo un alto nivel de calidad, rendimiento y mantenibilidad.
