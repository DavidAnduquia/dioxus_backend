# Rust REST API Backend

Backend REST API construido con Rust, Axum y PostgreSQL.

## Características

- 🚀 **Framework Web**: Axum
- 🗄️ **Base de Datos**: PostgreSQL
- 🧩 **Acceso a datos**: SeaORM (y SQLx en rutas puntuales)
- 🔐 **Autenticación**: JWT tokens con bcrypt
- ✅ **Validación**: Validación de entrada con validator
- 📝 **Logging**: Structured logging con tracing
- 🔧 **Configuración**: Variables de entorno con dotenvy
- 🛡️ **Seguridad**: CORS, middleware de autenticación
- 📊 **Manejo de Errores**: Error handling robusto

## Estructura del Proyecto

```
src/
├── main.rs              # Punto de entrada de la aplicación
├── config/              # Configuración de la aplicación
├── database/            # Configuración y migraciones de BD
├── handlers/            # Controladores de rutas
│   ├── auth.rs         # Autenticación
│   ├── usuarios.rs     # Gestión de usuarios
│   └── ...
├── middleware/          # Middleware personalizado
│   └── auth.rs         # Middleware de autenticación
├── models/             # Modelos (entidades SeaORM) + DTOs integrados por entidad
├── routes/             # Definición de rutas
└── utils/              # Utilidades y manejo de errores
```

## Configuración

1. **Instalar PostgreSQL** y crear una base de datos:
   ```sql
   CREATE DATABASE rust_api_db;
   ```

2. **Copiar variables de entorno**:
   ```bash
   cp .env.example .env
   ```

3. **Configurar `.env`** con tus credenciales de base de datos:
   ```env
   DATABASE_URL=postgresql://username:password@localhost/rust_api_db
   PORT=3000
   JWT_SECRET=tu-clave-secreta-muy-segura
   ENVIRONMENT=development
   ```

## Ejecutar el Proyecto

```bash
# Ejecutar
cargo run

# Validar compilación
cargo check

# Para desarrollo con auto-reload
cargo install cargo-watch
cargo watch -x run
```

## API Endpoints

### Autenticación
- `POST /auth/register` - Registrar nuevo usuario
- `POST /auth/login` - Iniciar sesión
- `POST /auth/validate-token` - Validar token
- `POST /auth/token` - OAuth2 (password grant)

### Usuarios
- `GET /api/usuarios` - Listar usuarios
- `POST /api/usuarios` - Crear usuario
- `GET /api/usuarios/{id}` - Obtener usuario
- `PUT /api/usuarios/{id}` - Actualizar usuario
- `POST /api/usuario/login` - Login alternativo
- `POST /api/usuario/logout/{id}` - Logout

### Swagger / Docs / WS
- `GET /swagger-ui` - Swagger UI
- `GET /api-docs/openapi.json` - Spec OpenAPI
- `GET /ws` - WebSocket

### Utilidad
- `GET /health` - Health check
- `GET /ready` - Readiness
- `GET /live` - Liveness

## Ejemplos de Uso

### Registrar Usuario
```bash
curl -X POST http://localhost:3000/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "password123",
    "name": "John Doe"
  }'
```

### Iniciar Sesión
```bash
curl -X POST http://localhost:3000/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "password123"
  }'
```

## Desarrollo

### Agregar Nuevas Entidades

1. **Modelo**: Agregar en `src/models/mod.rs`
2. **Migración/DDL**: Revisar `src/database/` (DDL/migrator/seeder)
3. **Handlers**: Crear en `src/handlers/`
4. **Rutas**: Agregar en `src/routes/mod.rs`

### Testing
```bash
cargo test
```

### Linting
```bash
cargo clippy
cargo fmt
```

## Producción

1. **Variables de entorno**:
   ```env
   ENVIRONMENT=production
   DATABASE_URL=postgresql://prod_user:prod_pass@prod_host/prod_db
   JWT_SECRET=super-secure-production-secret
   PORT=8080
   ```

2. **Build optimizado**:
   ```bash
   cargo build --release
   ```

3. **Ejecutar**:
   ```bash
   ./target/release/rust-api-backend
   ```

## Tecnologías Utilizadas

- **Axum**: Framework web async
- **SQLx**: Driver de base de datos async
- **PostgreSQL**: Base de datos relacional
- **JWT**: Autenticación stateless
- **BCrypt**: Hashing de contraseñas
- **Serde**: Serialización JSON
- **Tracing**: Logging estructurado
- **Validator**: Validación de datos
- **UUID**: Identificadores únicos
- **Chrono**: Manejo de fechas

## Contribuir

1. Fork el proyecto
2. Crear feature branch (`git checkout -b feature/nueva-funcionalidad`)
3. Commit cambios (`git commit -am 'Agregar nueva funcionalidad'`)
4. Push al branch (`git push origin feature/nueva-funcionalidad`)
5. Crear Pull Request
