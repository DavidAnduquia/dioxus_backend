-- Verificar estructura actual de la tabla usuarios
\d usuarios

-- Verificar si existe la columna fecha_eliminacion
SELECT column_name, data_type, is_nullable 
FROM information_schema.columns 
WHERE table_name = 'usuarios' 
AND table_schema = 'public'
ORDER BY ordinal_position;

-- Si la columna no existe, agregarla
ALTER TABLE usuarios ADD COLUMN IF NOT EXISTS fecha_eliminacion TIMESTAMPTZ;
