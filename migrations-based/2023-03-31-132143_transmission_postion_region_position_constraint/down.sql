-- This file should undo anything in `up.sql`
ALTER TABLE r09_transmission_locations DROP CONSTRAINT unique_region_position;
