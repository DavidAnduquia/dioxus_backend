-- Agregar columna fecha_eliminacion a la tabla usuarios si no existe
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 
        FROM information_schema.columns 
        WHERE table_name='usuarios' 
        AND column_name='fecha_eliminacion'
    ) THEN
        ALTER TABLE usuarios ADD COLUMN fecha_eliminacion TIMESTAMPTZ;
        RAISE NOTICE 'Columna fecha_eliminacion agregada a usuarios';
    ELSE
        RAISE NOTICE 'Columna fecha_eliminacion ya existe en usuarios';
    END IF;
END
$$;
