-- Verificar qué schemas existen
SELECT schema_name FROM information_schema.schemata WHERE schema_name NOT LIKE 'pg_%' AND schema_name != 'information_schema';

-- Verificar en qué schema está la tabla usuarios
SELECT table_schema, table_name FROM information_schema.tables WHERE table_name = 'usuarios';

-- Verificar estructura completa de la tabla usuarios en todos los schemas
SELECT 
    table_schema,
    column_name, 
    data_type, 
    is_nullable,
    column_default
FROM information_schema.columns 
WHERE table_name = 'usuarios'
ORDER BY table_schema, ordinal_position;

-- Verificar si la tabla usuarios está vacía o tiene datos
SELECT 'usuarios' as table_name, COUNT(*) as row_count, table_schema
FROM information_schema.tables t
LEFT JOIN information_schema.columns c ON t.table_name = c.table_name AND t.table_schema = c.table_schema
WHERE t.table_name = 'usuarios' AND t.table_schema NOT LIKE 'pg_%'
GROUP BY table_schema, t.table_name;
