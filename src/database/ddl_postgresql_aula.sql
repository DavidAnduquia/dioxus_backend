-- DDL PostgreSQL Aula Virtual v3.0
-- Migración completa a PostgreSQL

-- Habilitar extensión UUID si no está habilitada
--CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Tabla para roles (Coordinador, Profesor, Estudiante)
CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(50) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Tabla para usuarios (Coordinador, Profesor, Estudiante)
CREATE TABLE usuarios (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    documento_nit VARCHAR(20) UNIQUE NOT NULL,
    correo VARCHAR(100) UNIQUE NOT NULL,
    contrasena TEXT NOT NULL,
    foto_url TEXT,
    rol_id INTEGER REFERENCES roles(id) ON DELETE SET NULL,
    semestre INTEGER,
    genero VARCHAR(1) NOT NULL,
    fecha_nacimiento DATE NOT NULL,
    fecha_creacion TIMESTAMPTZ DEFAULT NOW(),
    fecha_actualizacion TIMESTAMPTZ DEFAULT NOW(),
    fecha_ultima_conexion TIMESTAMPTZ DEFAULT NOW(),
    token_primer_ingreso TIMESTAMPTZ,
    estado BOOLEAN DEFAULT true,
    CONSTRAINT chk_genero CHECK (genero IN ('M', 'F', 'O'))
);

-- Tabla de áreas de conocimiento (reemplaza materias)
CREATE TABLE areas_conocimiento (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL UNIQUE,
    descripcion TEXT,
    color_etiqueta VARCHAR(20) NOT NULL DEFAULT 'transparent',
    estado BOOLEAN NOT NULL DEFAULT true,
    fecha_creacion TIMESTAMPTZ DEFAULT NOW(),
    fecha_modificacion TIMESTAMPTZ DEFAULT NOW()
);

-- Tabla de cursos (ahora con relación directa a área de conocimiento)
CREATE TABLE cursos (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    descripcion TEXT NOT NULL,
    fecha_inicio DATE NOT NULL,
    fecha_fin DATE NOT NULL,
    prerequisito TEXT,
    coordinador_id INTEGER NOT NULL REFERENCES usuarios(id) ON DELETE CASCADE,
    creado_en TIMESTAMPTZ DEFAULT NOW(),
    plantilla_base_id INTEGER REFERENCES cursos(id) ON DELETE SET NULL,
    semestre INTEGER,
    periodo VARCHAR(20) NOT NULL DEFAULT '',
    anio_pensum INTEGER NOT NULL,
    area_conocimiento_id INTEGER NOT NULL REFERENCES areas_conocimiento(id) ON DELETE CASCADE,
    CONSTRAINT chk_fechas CHECK (fecha_fin >= fecha_inicio)
);

-- Tabla para plantillas de cursos
CREATE TABLE IF NOT EXISTS plantillas_cursos (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(150) NOT NULL,
    descripcion TEXT,
    activa BOOLEAN NOT NULL DEFAULT true,
    curso_id INTEGER REFERENCES cursos(id) ON DELETE CASCADE,
    fecha_creacion TIMESTAMPTZ DEFAULT NOW(),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Tabla para estudiantes matriculados y su historial
CREATE TABLE IF NOT EXISTS historial_cursos_estudiantes (
    id SERIAL PRIMARY KEY,
    curso_id INTEGER NOT NULL REFERENCES cursos(id) ON DELETE CASCADE,
    estudiante_id INTEGER NOT NULL REFERENCES usuarios(id) ON DELETE CASCADE,
    fecha_inscripcion TIMESTAMPTZ DEFAULT NOW(),
    estado VARCHAR(30) NOT NULL DEFAULT 'en_progreso',
    calificacion_final DOUBLE PRECISION,
    aprobado BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    CONSTRAINT uq_historial_curso_estudiante UNIQUE (curso_id, estudiante_id)
);

-- Tabla para profesores asignados a los cursos
CREATE TABLE profesores_curso (
    id SERIAL PRIMARY KEY,
    profesor_id INTEGER REFERENCES usuarios(id) ON DELETE CASCADE,
    curso_id INTEGER REFERENCES cursos(id) ON DELETE CASCADE,
    fecha_asignacion TIMESTAMPTZ DEFAULT NOW(),
    CONSTRAINT uq_profesor_curso UNIQUE (profesor_id, curso_id)
);

-- Tabla para contenido transversal
CREATE TABLE contenido_transversal (
    id SERIAL PRIMARY KEY,
    curso_id INTEGER REFERENCES cursos(id) ON DELETE CASCADE,
    origen_tipo VARCHAR(20) NOT NULL CHECK (origen_tipo IN ('curso', 'examen', 'modulo', 'portafolio')),
    origen_id INTEGER NOT NULL,
    profesor_id INTEGER REFERENCES usuarios(id) ON DELETE CASCADE,
    tipo_contenido VARCHAR(50) NOT NULL,
    ruta_archivo TEXT,
    enlace_video TEXT,
    fecha_subida TIMESTAMPTZ DEFAULT NOW(),
    privacidad VARCHAR(20),
    descripcion TEXT
);

-- Tabla para actividades (tareas, exámenes) dentro de un curso
CREATE TABLE actividades (
    id SERIAL PRIMARY KEY,
    curso_id INTEGER REFERENCES cursos(id) ON DELETE CASCADE,
    profesor_id INTEGER REFERENCES usuarios(id) ON DELETE CASCADE,
    nombre VARCHAR(200) NOT NULL,
    descripcion TEXT,
    privacidad VARCHAR(20),
    fecha_inicio TIMESTAMPTZ,
    fecha_fin TIMESTAMPTZ,
    tipo_actividad VARCHAR(50) NOT NULL
);

-- Tabla para registrar las calificaciones de los estudiantes
CREATE TABLE calificaciones (
    id SERIAL PRIMARY KEY,
    actividad_id INTEGER REFERENCES actividades(id) ON DELETE CASCADE,
    estudiante_id INTEGER REFERENCES usuarios(id) ON DELETE CASCADE,
    calificacion DECIMAL(5,2),
    fecha_registro TIMESTAMPTZ DEFAULT NOW(),
    CONSTRAINT uq_actividad_estudiante UNIQUE (actividad_id, estudiante_id)
);

-- Tabla para sesiones de curso
CREATE TABLE sesiones_curso (
    id SERIAL PRIMARY KEY,
    curso_id INTEGER REFERENCES cursos(id) ON DELETE CASCADE,
    nombre_sesion VARCHAR(200),
    descripcion TEXT,
    fecha_inicio TIMESTAMPTZ,
    fecha_fin TIMESTAMPTZ,
    orden INTEGER NOT NULL
);

-- Tabla para almacenar evaluaciones de cada sesión
CREATE TABLE evaluaciones_sesion (
    id SERIAL PRIMARY KEY,
    sesion_id INTEGER REFERENCES sesiones_curso(id) ON DELETE CASCADE,
    nombre_evaluacion VARCHAR(200),
    tipo_actividad VARCHAR(50) NOT NULL,
    fecha_inicio TIMESTAMPTZ,
    fecha_fin TIMESTAMPTZ
);

-- Tabla para registrar las calificaciones de evaluaciones
CREATE TABLE evaluaciones_calificacion (
    id SERIAL PRIMARY KEY,
    evaluacion_id INTEGER REFERENCES evaluaciones_sesion(id) ON DELETE CASCADE,
    estudiante_id INTEGER REFERENCES usuarios(id) ON DELETE CASCADE,
    calificacion DECIMAL(5,2),
    fecha_registro TIMESTAMPTZ DEFAULT NOW(),
    CONSTRAINT uq_evaluacion_estudiante UNIQUE (evaluacion_id, estudiante_id)
);

-- Tabla para la relación entre cursos y actividades
CREATE TABLE historial_curso_actividad (
    curso_id INTEGER REFERENCES cursos(id) ON DELETE CASCADE,
    actividad_id INTEGER REFERENCES actividades(id) ON DELETE CASCADE,
    fecha_fin TIMESTAMPTZ,
    estado VARCHAR(20) NOT NULL,
    PRIMARY KEY (curso_id, actividad_id)
);

-- Tabla de exámenes
CREATE TABLE examenes (
    id SERIAL PRIMARY KEY,
    curso_id INTEGER REFERENCES cursos(id) ON DELETE CASCADE,
    nombre VARCHAR(200) NOT NULL,
    descripcion TEXT,
    fecha_inicio TIMESTAMPTZ,
    fecha_fin TIMESTAMPTZ
);

-- Tabla de preguntas de examen
CREATE TABLE preguntas_examen (
    id SERIAL PRIMARY KEY,
    examen_id INTEGER REFERENCES examenes(id) ON DELETE CASCADE,
    enunciado TEXT NOT NULL,
    tipo VARCHAR(20) NOT NULL CHECK (tipo IN ('abierta', 'seleccion_unica', 'seleccion_multiple')),
    opciones JSONB,
    respuesta_correcta TEXT,
    fecha_inicio_pregunta TIMESTAMPTZ,
    fecha_fin_pregunta TIMESTAMPTZ,
    porcentaje DECIMAL(5,2) DEFAULT 0
);

-- Tabla de portafolios
CREATE TABLE portafolios (
    id SERIAL PRIMARY KEY,
    curso_id INTEGER REFERENCES cursos(id) ON DELETE CASCADE,
    nombre VARCHAR(200) NOT NULL,
    descripcion TEXT
);

-- Relación portafolio-contenido
CREATE TABLE portafolio_contenidos (
    id SERIAL PRIMARY KEY,
    portafolio_id INTEGER REFERENCES portafolios(id) ON DELETE CASCADE,
    contenido_id INTEGER REFERENCES contenido_transversal(id) ON DELETE CASCADE,
    CONSTRAINT uq_portafolio_contenido UNIQUE (portafolio_id, contenido_id)
);

CREATE TABLE modulos (
    id SERIAL PRIMARY KEY,
    curso_id INTEGER REFERENCES cursos(id) ON DELETE CASCADE,
    nombre VARCHAR(200) NOT NULL,
    descripcion TEXT,
    orden INTEGER NOT NULL DEFAULT 0,
    tipo VARCHAR(50) NOT NULL DEFAULT 'estructura_contenido', -- 'estructura_contenido', 'taller', 'evaluacion', etc.
    visible BOOLEAN NOT NULL DEFAULT true,
    fecha_inicio TIMESTAMPTZ,
    fecha_fin TIMESTAMPTZ,
    duracion_estimada INTEGER, -- en minutos
    obligatorio BOOLEAN NOT NULL DEFAULT true,
    fecha_creacion TIMESTAMPTZ DEFAULT NOW(),
    fecha_modificacion TIMESTAMPTZ DEFAULT NOW()
);

-- Tabla de temas (dentro de módulos)
CREATE TABLE temas (
    id SERIAL PRIMARY KEY,
    modulo_id INTEGER REFERENCES modulos(id) ON DELETE CASCADE,
    nombre VARCHAR(200) NOT NULL,
    descripcion TEXT,
    orden INTEGER NOT NULL DEFAULT 0,
    visible BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Tabla de unidades (dentro de temas)
CREATE TABLE unidades (
    id SERIAL PRIMARY KEY,
    modulo_id INTEGER REFERENCES modulos(id) ON DELETE CASCADE,
    nombre VARCHAR(200) NOT NULL,
    descripcion TEXT,
    orden INTEGER NOT NULL DEFAULT 0,
    visible BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Tabla de actividades de entrega (dentro de unidades)
CREATE TABLE actividades_entrega (
    id SERIAL PRIMARY KEY,
    unidad_id INTEGER REFERENCES unidades(id) ON DELETE CASCADE,
    nombre VARCHAR(200) NOT NULL,
    descripcion TEXT,
    fecha_limite TIMESTAMPTZ NOT NULL,
    tipo_actividad VARCHAR(50) NOT NULL CHECK (tipo_actividad IN ('entrega_obligatoria', 'entrega_opcional')),
    activo BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Tabla de entregas de estudiantes
CREATE TABLE entregas (
    id SERIAL PRIMARY KEY,
    actividad_entrega_id INTEGER REFERENCES actividades_entrega(id) ON DELETE CASCADE,
    estudiante_id INTEGER REFERENCES usuarios(id) ON DELETE CASCADE,
    documento_nombre VARCHAR(255) NOT NULL,
    documento_tipo VARCHAR(100) NOT NULL,
    documento_tamanio BIGINT NOT NULL,
    documento_url TEXT NOT NULL,
    fecha_entrega TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    calificacion REAL,
    comentario_profesor TEXT,
    fecha_calificacion TIMESTAMPTZ,
    estado VARCHAR(50) NOT NULL DEFAULT 'pendiente' CHECK (estado IN ('pendiente', 'calificado', 'rechazado')),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Tabla de contenidos por unidad (texto, archivo, video, quiz, actividad_entrega)
CREATE TABLE contenidos_unidad (
    id SERIAL PRIMARY KEY,
    unidad_id INTEGER NOT NULL REFERENCES unidades(id) ON DELETE CASCADE,

    tipo VARCHAR(50) NOT NULL CHECK (
        tipo IN ('texto', 'archivo', 'video', 'quiz', 'actividad_entrega')
    ),

    titulo VARCHAR(200) NOT NULL,
    descripcion TEXT,
    orden INTEGER NOT NULL DEFAULT 0,

    -- Campos directos
    texto_largo TEXT,
    archivo_url TEXT,
    archivo_tipo VARCHAR(100),
    video_url TEXT,

    -- Enlaces a otras tablas segun tipo
    examen_id INTEGER REFERENCES examenes(id) ON DELETE CASCADE,
    entrega_id INTEGER REFERENCES entregas(id) ON DELETE CASCADE,

    activo BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Tabla de webinars (aprendizaje interactivo)
CREATE TABLE webinars (
    id SERIAL PRIMARY KEY,
    curso_id INTEGER REFERENCES cursos(id) ON DELETE CASCADE,
    titulo VARCHAR(200) NOT NULL,
    descripcion TEXT,
    progreso INTEGER NOT NULL DEFAULT 0 CHECK (progreso >= 0 AND progreso <= 100),
    estado VARCHAR(50) NOT NULL DEFAULT 'no_iniciado' CHECK (estado IN ('no_iniciado', 'en_progreso', 'completado')),
    duracion VARCHAR(50), -- ej: "45 min", "1.5 horas"
    modulos INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Tabla de módulos de webinar
CREATE TABLE webinar_modulos (
    id SERIAL PRIMARY KEY,
    webinar_id INTEGER REFERENCES webinars(id) ON DELETE CASCADE,
    titulo VARCHAR(200) NOT NULL,
    descripcion TEXT,
    orden INTEGER NOT NULL DEFAULT 0,
    tipo_contenido VARCHAR(50) NOT NULL DEFAULT 'video' CHECK (tipo_contenido IN ('video', 'presentacion', 'actividad', 'quiz')),
    contenido_url TEXT,
    duracion_estimada INTEGER, -- en minutos
    obligatorio BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Tabla de progreso de estudiantes en webinars
CREATE TABLE webinar_progreso_estudiantes (
    webinar_id INTEGER REFERENCES webinars(id) ON DELETE CASCADE,
    estudiante_id INTEGER REFERENCES usuarios(id) ON DELETE CASCADE,
    progreso_actual INTEGER NOT NULL DEFAULT 0 CHECK (progreso_actual >= 0 AND progreso_actual <= 100),
    modulos_completados INTEGER NOT NULL DEFAULT 0,
    tiempo_total_visto INTEGER NOT NULL DEFAULT 0, -- en minutos
    ultima_actividad TIMESTAMPTZ,
    completado BOOLEAN NOT NULL DEFAULT false,
    fecha_completado TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (webinar_id, estudiante_id)
);

-- Tabla de personalización de webinar
CREATE TABLE personalizacion_webinar (
    id SERIAL PRIMARY KEY,
    webinar_id INTEGER REFERENCES webinars(id) ON DELETE CASCADE,
    estilos JSONB,
    orden_componentes JSONB,
    privacidad_componentes JSONB,
    CONSTRAINT uq_personalizacion_webinar UNIQUE (webinar_id)
);
CREATE TABLE personalizacion_examen (
    id SERIAL PRIMARY KEY,
    examen_id INTEGER REFERENCES examenes(id) ON DELETE CASCADE,
    estilos JSONB,
    orden_componentes JSONB,
    privacidad_componentes JSONB,
    CONSTRAINT uq_personalizacion_examen UNIQUE (examen_id)
);

-- Tabla de personalización de portafolio
CREATE TABLE personalizacion_portafolio (
    id SERIAL PRIMARY KEY,
    portafolio_id INTEGER REFERENCES portafolios(id) ON DELETE CASCADE,
    estilos JSONB,
    orden_componentes JSONB,
    privacidad_componentes JSONB,
    CONSTRAINT uq_personalizacion_portafolio UNIQUE (portafolio_id)
);

-- Tabla de personalización de módulo
CREATE TABLE personalizacion_modulo (
    id SERIAL PRIMARY KEY,
    modulo_id INTEGER REFERENCES modulos(id) ON DELETE CASCADE,
    estilos JSONB,
    orden_componentes JSONB,
    privacidad_componentes JSONB,
    CONSTRAINT uq_personalizacion_modulo UNIQUE (modulo_id)
);

-- Tabla para eventos programados (cron backend)
CREATE TABLE eventos_programados (
    id SERIAL PRIMARY KEY,
    descripcion TEXT,
    fecha_ejecucion TIMESTAMPTZ DEFAULT NOW(),
    estado VARCHAR(20) DEFAULT 'pendiente'
);

-- Índices para mejorar el rendimiento
CREATE INDEX idx_usuarios_rol ON usuarios(rol_id);
CREATE INDEX idx_cursos_area ON cursos(area_conocimiento_id);
CREATE INDEX idx_actividades_curso ON actividades(curso_id);
CREATE INDEX idx_calificaciones_actividad ON calificaciones(actividad_id);
CREATE INDEX idx_calificaciones_estudiante ON calificaciones(estudiante_id);

-- Índices para las nuevas tablas
CREATE INDEX idx_modulos_curso ON modulos(curso_id);
CREATE INDEX idx_modulos_tipo ON modulos(tipo);
CREATE INDEX idx_temas_modulo ON temas(modulo_id);
CREATE INDEX idx_unidades_modulo ON unidades(modulo_id);
CREATE INDEX idx_actividades_entrega_unidad ON actividades_entrega(unidad_id);
CREATE INDEX idx_entregas_actividad ON entregas(actividad_entrega_id);
CREATE INDEX idx_entregas_estudiante ON entregas(estudiante_id);
CREATE INDEX idx_contenidos_unidad_unidad ON contenidos_unidad(unidad_id);
CREATE INDEX idx_webinars_curso ON webinars(curso_id);
CREATE INDEX idx_webinar_modulos_webinar ON webinar_modulos(webinar_id);
CREATE INDEX idx_webinar_progreso_estudiante ON webinar_progreso_estudiantes(estudiante_id);

-- Insertar roles básicos
INSERT INTO roles (nombre) VALUES
    ('Coordinador'),
    ('Profesor'),
    ('Estudiante')
ON CONFLICT (nombre) DO NOTHING;

-- Insertar áreas de conocimiento de ejemplo
INSERT INTO areas_conocimiento (nombre, descripcion, color_etiqueta, estado) VALUES
    ('Matemáticas', 'Área de conocimiento enfocada en ciencias exactas y lógica matemática', '#FF6B6B', true),
    ('Ciencias Naturales', 'Comprende física, química, biología y ciencias de la tierra', '#4ECDC4', true),
    ('Ciencias Sociales', 'Historia, geografía, cívica y estudios sociales', '#45B7D1', true),
    ('Lenguaje y Literatura', 'Comunicación, lectura comprensiva y expresión escrita', '#96CEB4', true),
    ('Tecnología e Informática', 'Herramientas digitales y pensamiento computacional', '#FFEAA7', true),
    ('Artes', 'Expresión artística, música, danza y teatro', '#DDA0DD', true),
    ('Educación Física', 'Desarrollo corporal, deportes y actividad física', '#FFB74D', true)
ON CONFLICT (nombre) DO NOTHING;

-- Insertar usuarios de ejemplo
INSERT INTO usuarios (
    nombre,
    documento_nit,
    correo,
    contrasena,
    foto_url,
    rol_id,
    semestre,
    genero,
    fecha_nacimiento,
    estado
) VALUES
    ('David A. Anduquia', '1234567890', 'david.anduquia@aulavirtual.com', 'admin123', 'https://randomuser.me/api/portraits/men/1.jpg', 1, NULL, 'M', '1980-01-01', true),
    ('María García Profesora', 'PROF456', 'maria.garcia@aulavirtual.com', 'prof123', 'https://randomuser.me/api/portraits/women/1.jpg', 2, NULL, 'F', '1985-05-15', true),
    ('Carlos López Estudiante', 'EST789', 'carlos.lopez@aulavirtual.com', 'est123', 'https://randomuser.me/api/portraits/men/2.jpg', 3, 5, 'M', '2000-10-20', true),
    ('Ana Rodríguez Estudiante', 'EST790', 'ana.rodriguez@aulavirtual.com', 'est123', 'https://randomuser.me/api/portraits/women/2.jpg', 3, 5, 'F', '2001-03-15', true)
ON CONFLICT (documento_nit) DO NOTHING;

-- Insertar cursos de ejemplo
INSERT INTO cursos (
    nombre,
    descripcion,
    fecha_inicio,
    fecha_fin,
    prerequisito,
    coordinador_id,
    semestre,
    periodo,
    anio_pensum,
    area_conocimiento_id
) VALUES
    ('Álgebra Básica', 'Fundamentos de álgebra para educación media', '2025-02-01', '2025-06-30', NULL, 1, 5, '2025-1', 2025, 1),
    ('Física Mecánica', 'Introducción a la física mecánica clásica', '2025-02-01', '2025-06-30', 'Álgebra Básica', 1, 6, '2025-1', 2025, 2)
ON CONFLICT DO NOTHING;

-- Tabla de notificaciones
CREATE TABLE notificaciones (
    id SERIAL PRIMARY KEY,
    usuario_id INTEGER NOT NULL REFERENCES usuarios(id) ON DELETE CASCADE,
    titulo VARCHAR(255) NOT NULL,
    mensaje TEXT NOT NULL,
    tipo VARCHAR(50) NOT NULL,
    leida BOOLEAN DEFAULT false,
    enlace VARCHAR(500),
    datos_adicionales JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Crear índices para notificaciones
CREATE INDEX idx_notificaciones_usuario_id ON notificaciones(usuario_id);
CREATE INDEX idx_notificaciones_tipo ON notificaciones(tipo);
CREATE INDEX idx_notificaciones_leida ON notificaciones(leida);

-- Insertar módulos de ejemplo para el curso de Álgebra Básica
INSERT INTO modulos (
    curso_id, nombre, descripcion, orden, tipo, visible, obligatorio, duracion_estimada
) VALUES
    (1, 'Estructura del Curso', 'Organización y navegación del contenido del curso', 1, 'estructura_contenido', true, true, 30),
    (1, 'Taller de Ejercicios', 'Práctica intensiva con ejercicios resueltos', 2, 'taller', true, false, 120),
    (1, 'Evaluación Final', 'Examen final del módulo de álgebra básica', 3, 'evaluacion', true, true, 90)
ON CONFLICT DO NOTHING;

-- Insertar temas de ejemplo
INSERT INTO temas (
    modulo_id, nombre, descripcion, orden, visible
) VALUES
    (1, 'Introducción a la Programación', 'Conceptos básicos y fundamentos de la programación', 1, true),
    (1, 'Estructuras de Control', 'Control del flujo de ejecución', 2, true)
ON CONFLICT DO NOTHING;

-- Insertar unidades de ejemplo
INSERT INTO unidades (
    tema_id, nombre, descripcion, orden, visible
) VALUES
    (1, 'Algoritmos y Lógica', 'Entendiendo el pensamiento algorítmico', 1, true),
    (1, 'Variables y Tipos de Datos', 'Conceptos fundamentales de datos', 2, true),
    (2, 'Condicionales', 'Estructuras if-else y switch', 1, true)
ON CONFLICT DO NOTHING;

-- Insertar webinars de ejemplo
INSERT INTO webinars (
    curso_id, titulo, descripcion, progreso, estado, duracion, modulos
) VALUES
    (1, 'Introducción a la Programación Web', 'Aprende los fundamentos del desarrollo web con ejemplos prácticos', 75, 'en_progreso', '45 min', 8),
    (2, 'Bases de Datos Relacionales', 'Explora el diseño y gestión de bases de datos con SQL', 100, 'completado', '60 min', 12)
ON CONFLICT DO NOTHING;