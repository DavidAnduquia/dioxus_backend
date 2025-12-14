# Observaciones del Backend (backend-aula)

## 1. Stack real del proyecto

- **Framework HTTP:** Axum
- **Base de datos:** PostgreSQL
- **Acceso a datos:** SeaORM como ORM principal (y SQLx en rutas puntuales)
- **Auth:** JWT + bcrypt
- **Configuración:** `dotenvy` (variables de entorno)
- **Docs:** Swagger UI expuesto por rutas del backend
- **WebSocket:** endpoint `/ws`

## 2. Puertos y URLs locales

- **Puerto por defecto:** `3000` (variable `PORT`, default en `src/config/mod.rs`)
- **Servidor:** `http://127.0.0.1:3000`
- **Swagger UI:** `http://127.0.0.1:3000/swagger-ui`
- **OpenAPI JSON:** `http://127.0.0.1:3000/api-docs/openapi.json`
- **WebSocket:** `ws://127.0.0.1:3000/ws`
- **CORS (dev):** permitido para frontend en `http://localhost:8080` y `http://127.0.0.1:8080` (ver `src/main.rs`)

## 3. Rutas (endpoints) reales

Rutas públicas principales (ver `src/routes/mod.rs`):

- `GET /health`
- `GET /ready`
- `GET /live`
- `POST /auth/register`
- `POST /auth/login`
- `POST /auth/validate-token`
- `POST /auth/token`
- `GET /ws`
- `GET /metrics/memory`
- `POST /metrics/optimize`

Rutas bajo `/api/*` (ver `src/routes/*.rs`):

- **Usuarios:**
  - `GET /api/usuarios`
  - `POST /api/usuarios`
  - `GET /api/usuarios/{id}`
  - `PUT /api/usuarios/{id}`
  - `POST /api/usuario/login`
  - `POST /api/usuario/logout/{id}`
- **Roles:**
  - `GET /api/roles`
  - `POST /api/roles`
  - `GET /api/roles/{id}`
  - `PUT /api/roles/{id}`
  - `DELETE /api/roles/{id}`
- **Cursos:**
  - `GET /api/cursos`
  - `POST /api/cursos`
  - `GET /api/cursos/{id}`
  - `PUT /api/cursos/{id}`
  - `DELETE /api/cursos/{id}`
  - `GET /api/cursos/{id}/aula`
  - `GET /api/plantillas/{plantilla_id}/cursos`
  - `GET /api/areas-conocimiento/{area_id}/cursos`

(Existen más rutas por entidad en `src/routes/`.)

## 4. Convención de modelos y DTOs (refactor aplicado)

- **Regla:** `src/services/` no debe definir DTOs.
- **DTOs integrados:** los structs de request/update (por ejemplo `Nuevo*`, `Nueva*`, `Actualizar*`, `*Request`, `*Response`) viven dentro del módulo base en `src/models/<entidad>.rs`.
- **Importación/re-export:** los services deben consumir DTOs desde `crate::models::<entidad>::{...}`.

Verificación recomendada:

- `cargo check` debe compilar.
- En `src/services`, un grep de `pub struct (Nuevo|Nueva|Actualizar)` no debe retornar coincidencias.

## 5. `target/` no es documentación del proyecto

- Los `.md` dentro de `target/` son archivos generados por dependencias/build.
- No se deben editar ni tomar como documentación oficial.

## 6. Comandos de trabajo recomendados

- Compilación:
  - `cargo check`
- Ejecución:
  - `cargo run`

## 7. Observaciones técnicas (pendientes/alertas)

- Hay warnings de Rust del tipo "never constructed" / "never used" en varios modelos y services; no bloquean compilación pero ensucian el build.
- Si se requiere limpieza completa, la siguiente etapa es decidir:
  - qué endpoints/servicios están realmente en uso,
  - y eliminar o integrar lo que quedó como código muerto.
