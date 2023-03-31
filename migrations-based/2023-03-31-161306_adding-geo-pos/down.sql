-- This file should undo anything in `up.sql`

ALTER TABLE regions DROP COLUMN lat;
ALTER TABLE regions DROP COLUMN lon;
ALTER TABLE regions DROP COLUMN zoom;

