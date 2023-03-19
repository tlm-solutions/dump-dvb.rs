-- Your SQL goes here
-- fix a fucky where r09 types are different between tables
ALTER TABLE regions ALTER COLUMN r09_type TYPE BIGINT;
