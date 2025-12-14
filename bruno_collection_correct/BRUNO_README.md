# Virtual Room API - Colección Bruno (AUTOMATIZADA) 🤖

Esta colección tiene **AUTOMATIZACIÓN COMPLETA** con scripts de pre-request y post-response para manejo automático de JWT tokens e IDs.

## 🚀 **AUTOMATIZACIÓN IMPLEMENTADA**

### **1. JWT Token Automático** 🔐
- **Script de colección pre-request**: Agrega automáticamente `Authorization: Bearer {{JWT_TOKEN}}` a TODAS las requests que requieren autenticación
- **Script post-login**: Guarda automáticamente el JWT token después del login exitoso
- **Validación automática**: Todas las rutas protegidas requieren JWT válido

### **2. Secuencia de Uso** 📋
```bash
# Opción 1: Login directo (recomendado)
1. Ejecutar "Auth/Login User" → ✅ JWT_TOKEN guardado automáticamente

# Opción 2: OAuth2 Token
1. Ejecutar "Auth/OAuth2 Token" → ✅ JWT_TOKEN guardado automáticamente

2. Todas las requests siguientes incluyen Authorization header automáticamente
3. ¡Todo funciona sin configuración manual!
```

### **3. Métodos de Autenticación** 🔐

#### **Login User** (Recomendado)
- **Endpoint:** `POST /auth/login`
- **Body:** JSON con email/password
- **Respuesta:** `ApiResponse<AuthResponse>` con token en `data.token`

#### **OAuth2 Token** 
- **Endpoint:** `POST /auth/token`
- **Body:** JSON con grant_type, username, password, scope
- **Respuesta:** `OAuth2TokenResponse` con token en `access_token`

### **4. Endpoints que requieren JWT** 🔒
**TODOS** estos endpoints requieren autenticación JWT:

| Categoría | Endpoints | Estado |
|-----------|-----------|--------|
| 👥 **Usuarios** | `GET/POST/PUT /api/usuarios/*` | ✅ Protegido |
| 👔 **Roles** | `GET/POST/PUT/DELETE /api/roles/*` | ✅ Protegido |
| 🏫 **Áreas** | `GET/POST/PUT/DELETE /api/areas-conocimiento/*` | ✅ Protegido |
| 📚 **Cursos** | `GET/POST/PUT/DELETE /api/cursos/*` | ✅ Protegido |
| 📝 **Exámenes** | `GET/POST/PUT/DELETE /api/examenes/*` | ✅ Protegido |
| 📋 **Matrículas** | `GET/POST/DELETE /api/matriculas/*` | ✅ Protegido |
| 📖 **Módulos** | `GET/POST/PUT/DELETE /api/modulos/*` | ✅ Protegido |
| 🎯 **Actividades** | `GET/POST/PUT/DELETE /api/actividades/*` | ✅ Protegido |
| 🔔 **Notificaciones** | `GET/POST/PUT /api/notificaciones/*` | ✅ Protegido |

### **5. Endpoints públicos** 🌐
Estos endpoints **NO** requieren JWT:
- `GET /health` - Health check
- `GET /ready` - Readiness check
- `GET /live` - Liveness check
- `POST /auth/register` - Registro
- `POST /auth/login` - Login
- `POST /auth/token` - OAuth2 token
- `GET /swagger-ui/*` - Documentación

### **6. IDs Automáticos** 🆔
Los siguientes endpoints guardan automáticamente sus IDs creados:

| Endpoint | Variable | Descripción |
|----------|----------|-------------|
| `POST /auth/login` | `JWT_TOKEN` | Token de autenticación |
| `POST /api/usuarios` | `USER_ID` | ID del usuario creado |
| `POST /api/roles` | `ROLE_ID` | ID del rol creado |
| `POST /api/areas-conocimiento` | `AREA_ID` | ID del área creada |
| `POST /api/cursos` | `CURSO_ID` | ID del curso creado |
| `POST /api/modulos` | `MODULO_ID` | ID del módulo creado |
| `POST /api/actividades` | `ACTIVIDAD_ID` | ID de la actividad creada |
| `POST /api/examenes` | `EXAMEN_ID` | ID del examen creado |

### **6. Debugging Automático** 🔍
- **Logs en consola**: Cada request y response se registra automáticamente
- **Manejo de errores**: Detecta automáticamente errores 401, 403, 404
- **Sugerencias**: Te indica qué hacer cuando hay problemas

## 🎯 **FLUJO DE TRABAJO AUTOMATIZADO**

### **Inicio de Sesión:**
```
1. Ejecutar "Auth/Login User"
2. ✅ JWT_TOKEN se guarda automáticamente
3. 🔐 Todas las requests siguientes incluyen Authorization header
```

### **Creación de Contenido:**
```
1. Login → JWT_TOKEN guardado
2. Create Area → AREA_ID guardado
3. Create Curso → CURSO_ID guardado
4. Create Modulo → MODULO_ID guardado
5. Create Actividad → ACTIVIDAD_ID guardado
6. ¡Todo automatizado!
```

### **Testing de Autenticación** 🧪
```bash
# 1. Probar endpoint público (debe funcionar sin token)
GET /health → ✅ 200 OK

# 2. Probar endpoint protegido sin token
GET /api/usuarios → ❌ 401 Unauthorized

# 3. Hacer login
POST /auth/login → ✅ JWT_TOKEN guardado

# 4. Probar endpoint protegido con token
GET /api/usuarios → ✅ 200 OK con datos
```

## 🛠️ **SCRIPTS IMPLEMENTADOS**

### **Script de Colección Pre-request:**
```javascript
// Agrega automáticamente Authorization header si JWT_TOKEN existe
if (bru.getEnvVar("JWT_TOKEN")) {
  req.setHeader("Authorization", "Bearer " + bru.getEnvVar("JWT_TOKEN"));
  console.log("🔐 JWT Token agregado automáticamente");
}
```

### **Script Post-login:**
```javascript
// Extrae y guarda JWT token de respuesta
const responseData = res.getBody();
if (responseData.success && responseData.data.token) {
  bru.setEnvVar("JWT_TOKEN", responseData.data.token);
  console.log("✅ JWT Token guardado");
}
```

## 🔧 **CONFIGURACIÓN TÉCNICA**

- **Base URL**: `http://localhost:3030`
- **Autenticación**: JWT Bearer Token
- **Content-Type**: `application/json`
- **Variables**: Todas manejadas automáticamente

## 🚨 **TROUBLESHOOTING**

### **Error 401 Unauthorized:**
- ✅ Asegúrate de ejecutar "Auth/Login User" primero
- ✅ Verifica que JWT_TOKEN se guardó en consola
- ✅ Confirma que el servidor esté ejecutándose en puerto 3030

### **Error de conexión:**
- ✅ Verifica que el servidor esté ejecutándose: `cargo run --release`
- ✅ Confirma BASE_URL: `http://localhost:3030`
- ✅ Revisa logs del servidor para errores

### **Token expirado:**
- ✅ Vuelve a ejecutar "Auth/Login User"
- ✅ Los tokens expiran en 24 horas

---

**¡Colección Bruno completamente automatizada con JWT!** 🔐🚀

**Características principales:**
- ✅ **JWT automático** - Login guarda token, requests incluyen auth automáticamente
- ✅ **IDs automáticos** - Crear recursos guarda IDs para requests siguientes
- ✅ **Debugging integrado** - Logs y manejo de errores automático
- ✅ **Testing completo** - Todas las rutas protegidas requieren JWT válido
