-- DDL PostgreSQL Aula Virtual v3.0
-- Migración completa a PostgreSQL

-- Habilitar extensión UUID si no está habilitada
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

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
CREATE TABLE plantillas_curso (
    id SERIAL PRIMARY KEY,
    curso_id INTEGER REFERENCES cursos(id) ON DELETE CASCADE,
    descripcion TEXT,
    fecha_creacion TIMESTAMPTZ DEFAULT NOW()
);

-- Tabla para estudiantes matriculados y su historial
CREATE TABLE historial_curso_estudiante (
    id SERIAL PRIMARY KEY,
    estudiante_id INTEGER REFERENCES usuarios(id) ON DELETE CASCADE,
    curso_id INTEGER REFERENCES cursos(id) ON DELETE CASCADE,
    semestre VARCHAR(20),
    fecha_matricula TIMESTAMPTZ DEFAULT NOW(),
    calificacion DECIMAL(5,2),
    CONSTRAINT uq_estudiante_curso UNIQUE (estudiante_id, curso_id)
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
    fecha_creacion TIMESTAMPTZ DEFAULT NOW(),
    fecha_modificacion TIMESTAMPTZ DEFAULT NOW()
);

-- Tabla de personalización de examen
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

-- Función para actualizar automáticamente los timestamps
CREATE OR REPLACE FUNCTION update_modified_column() 
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW; 
END;
$$ LANGUAGE plpgsql;

-- Triggers para actualizar automáticamente las fechas de modificación
CREATE TRIGGER update_usuarios_modtime
BEFORE UPDATE ON usuarios
FOR EACH ROW EXECUTE FUNCTION update_modified_column();

CREATE TRIGGER update_areas_conocimiento_modtime
BEFORE UPDATE ON areas_conocimiento
FOR EACH ROW EXECUTE FUNCTION update_modified_column();

-- Insertar datos iniciales
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

-- Función para hashear contraseñas (deberías usar bcrypt en tu aplicación)
CREATE OR REPLACE FUNCTION hash_password()
RETURNS TRIGGER AS $$
BEGIN
    -- En un entorno real, usa bcrypt o similar
    -- NEW.contrasena = crypt(NEW.contrasena, gen_salt('bf'));
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER hash_user_password
BEFORE INSERT OR UPDATE OF contrasena ON usuarios
FOR EACH ROW EXECUTE FUNCTION hash_password();