-- DEV ONLY - Brute Force DROP DB (for local dev and unit test)
-- SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE
--  usename = 'app_user' OR datname = 'app_db';
-- Drop the schema if it exists
DROP SCHEMA IF EXISTS groovy_layers CASCADE;

-- Create the Alpha schema
DROP TABLE IF EXISTS groovy_layers.users CASCADE;
DROP TABLE IF EXISTS groovy_layers.orders CASCADE;
DROP TABLE IF EXISTS groovy_layers.print_stations CASCADE;
DROP TABLE IF EXISTS groovy_layers.material_inventory CASCADE;


CREATE SCHEMA groovy_layers;
