# Virtual Room API - Colección Bruno (AUTOMATIZADA) 🤖

Esta colección tiene **AUTOMATIZACIÓN COMPLETA** con scripts de pre-request y post-response para manejo automático de JWT tokens e IDs.

## 🚀 **AUTOMATIZACIÓN IMPLEMENTADA**

### **1. JWT Token Automático** 🔐
- **Pre-request script**: Agrega automáticamente `Authorization: Bearer {{JWT_TOKEN}}` a todas las requests
- **Post-login script**: Guarda automáticamente el JWT token después del login
- **Post-logout script**: Limpia automáticamente el token

### **2. IDs Automáticos** 🆔
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

### **3. Debugging Automático** 🔍
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

## 🛠️ **SCRIPTS IMPLEMENTADOS**

```javascript
vars:pre-request {
  // Agrega Authorization header automáticamente
  if (bru.getEnvVar("JWT_TOKEN")) {
    req.setHeader("Authorization", "Bearer " + bru.getEnvVar("JWT_TOKEN"));
    console.log("🔐 Header Authorization agregado automáticamente");
  } else {
    console.log("⚠️  No hay JWT_TOKEN configurado");
  }

  // Log de la request para debugging
  console.log("📤 Enviando request a:", req.getUrl());
}

vars:post-response {
  // Manejo automático de errores
  console.log("📥 Response status:", res.getStatusCode());

  if (res.getStatusCode() === 401) {
    console.log("🚫 Error 401: Token expirado - Haz login nuevamente");
  }
  if (res.getStatusCode() === 403) {
    console.log("🚫 Error 403: No tienes permisos");
  }
  if (res.getStatusCode() === 404) {
    console.log("🚫 Error 404: Recurso no encontrado");
  }
}
```

### **Scripts por Endpoint**
Cada endpoint de creación tiene su script `post-response` que guarda automáticamente el ID creado.

## 📋 **ENDPOINTS CON AUTOMATIZACIÓN**

### **🔐 Autenticación (100% Automatizada)**
- ✅ Login User → Guarda JWT_TOKEN
- ✅ Login User Alternative → Guarda JWT_TOKEN
- ✅ Logout User → Limpia JWT_TOKEN

### **👥 Gestión de Usuarios (90% Automatizada)**
- ✅ Create User → Guarda USER_ID
- ❌ Get User by ID → Usa {{USER_ID}}
- ❌ Update User → Usa {{USER_ID}}
- ❌ Logout User → Usa {{USER_ID}}

### **🏷️ Roles (100% Automatizada)**
- ✅ Create Role → Guarda ROLE_ID
- ❌ Get/Update/Delete Role → Usan {{ROLE_ID}}

### **📚 Áreas (100% Automatizada)**
- ✅ Create Area → Guarda AREA_ID
- ❌ Resto de operaciones → Usan {{AREA_ID}}

### **🎓 Cursos (100% Automatizada)**
- ✅ Create Curso → Guarda CURSO_ID
- ❌ Resto de operaciones → Usan {{CURSO_ID}}

### **📝 Exámenes (100% Automatizada)**
- ✅ Create Examen → Guarda EXAMEN_ID
- ❌ Resto de operaciones → Usan {{EXAMEN_ID}}

### **📋 Matrículas (Manual)**
- ❌ Todas requieren {{ESTUDIANTE_ID}} y {{CURSO_ID}}

### **📚 Módulos (100% Automatizada)**
- ✅ Create Modulo → Guarda MODULO_ID
- ❌ Resto de operaciones → Usan {{MODULO_ID}}

### **🎯 Actividades (100% Automatizada)**
- ✅ Create Actividad → Guarda ACTIVIDAD_ID
- ❌ Resto de operaciones → Usan {{ACTIVIDAD_ID}}

### **🔔 Notificaciones (Manual)**
- ❌ Requiere {{USER_ID}}

### **📊 Métricas (Públicas)**
- ✅ No requieren autenticación

## 🚀 **¿CÓMO USAR?**

### **Configuración Inicial:**
1. Abrir colección en Bruno
2. Seleccionar entorno "Local Development"
3. Ejecutar "Auth/Login User"
4. ✅ ¡Todo está automatizado!

### **Creación de Contenido Completo:**
```bash
Login → Create Area → Create Curso → Create Modulo → Create Actividad
✅ Todos los IDs se guardan automáticamente
```

## 🌍 **Variables de Entorno Completas**

### **Entornos Disponibles:**
- **Local Development:** Variables en MAYÚSCULAS `{{BASE_URL}}`
- **Local Development (lowercase):** Variables en minúsculas `{{base_url}}`
- **Production:** Variables en MAYÚSCULAS para producción

### **18 Variables por Entorno:**

#### **🔗 Configuración de Conexión:**
- `BASE_URL` - URL completa (ej: `http://localhost:3030`)
- `PROTOCOL` - Protocolo HTTP/HTTPS (ej: `http`, `https`)
- `HOST` - Nombre del host (ej: `localhost`, `api.example.com`)
- `PORT` - Puerto del servidor (ej: `3030`, vacío para HTTPS)

#### **🔐 Autenticación:**
- `JWT_TOKEN` - Token JWT (autoguardado después del login)

#### **👥 IDs de Usuarios:**
- `USER_ID` - ID de usuario (autoguardado al crear usuario)
- `ESTUDIANTE_ID` - ID de estudiante (manual)
- `PROFESSOR_ID` - ID de profesor (manual)

#### **🏷️ Gestión de Roles:**
- `ROLE_ID` - ID de rol (autoguardado al crear rol)

#### **📚 Contenido Educativo:**
- `AREA_ID` - ID de área de conocimiento (autoguardado)
- `CURSO_ID` - ID de curso (autoguardado)
- `MODULO_ID` - ID de módulo (autoguardado)
- `ACTIVIDAD_ID` - ID de actividad (autoguardado)
- `EXAMEN_ID` - ID de examen (autoguardado)

#### **🔔 Comunicación:**
- `NOTIFICACION_ID` - ID de notificación (manual)

#### **📋 Plantillas:**
- `PLANTILLA_ID` - ID de plantilla (manual)
- `TEMPLATE_ID` - Alias de PLANTILLA_ID (manual)

### **💡 Ejemplos de Uso:**

#### **URLs Combinadas:**
```bru
url: {{PROTOCOL}}://{{HOST}}:{{PORT}}/api/users
url: {{BASE_URL}}/api/users  # Equivalente
```

#### **Headers Dinámicos:**
```javascript
// En scripts
const fullUrl = bru.getEnvVar("PROTOCOL") + "://" +
                bru.getEnvVar("HOST") + ":" +
                bru.getEnvVar("PORT") + "/api/endpoint";
```

#### **Configuración por Entorno:**
```json
// Local Development
{
  "PROTOCOL": "http",
  "HOST": "localhost",
  "PORT": "3030"
}

// Production
{
  "PROTOCOL": "https",
  "HOST": "api.example.com",
  "PORT": ""
}
```

## 📋 **Referencia de Variables**

**📄 Archivo: `collection.bru`** - Documentación completa en formato JavaScript object
**📄 Archivo: `VARIABLES_REFERENCE.bru`** - Referencia rápida para copiar y pegar

### **Formato de Variables:**
```javascript
const variables = {
  BASE_URL: "http://localhost:3030",
  PROTOCOL: "http",
  HOST: "localhost",
  PORT: "3030",
  JWT_TOKEN: "",
  // ... todas las variables
};
```

## 📈 **ESTADÍSTICAS DE AUTOMATIZACIÓN**

- **🔐 Autenticación**: 100% automatizada
- **👥 Usuarios**: 90% automatizada
- **🏷️ Roles**: 100% automatizada
- **📚 Áreas**: 100% automatizada
- **🎓 Cursos**: 100% automatizada
- **📝 Exámenes**: 100% automatizada
- **📋 Matrículas**: Manual (requiere IDs previos)
- **📚 Módulos**: 100% automatizada
- **🎯 Actividades**: 100% automatizada
- **🔔 Notificaciones**: Manual
- **📊 Métricas**: 100% (públicas)

---

**¡Colección Bruno con automatización de nivel PRO!** 🚀🤖

**Variables se guardan automáticamente con pre-request y post-request scripts.**usar!** 🎉
