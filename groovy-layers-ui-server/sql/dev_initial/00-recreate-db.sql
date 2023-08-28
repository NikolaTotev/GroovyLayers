-- DEV ONLY - Brute Force DROP DB (for local dev and unit test)
-- SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE
--  usename = 'app_user' OR datname = 'app_db';
-- Drop the schema if it exists
DROP SCHEMA IF EXISTS groovy_layers CASCADE;
DROP TABLE IF EXISTS groovy_layers.user CASCADE;
DROP TABLE IF EXISTS groovy_layers.task CASCADE;

-- Create the Alpha schema
CREATE SCHEMA groovy_layers;