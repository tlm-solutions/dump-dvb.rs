-- This file should undo anything in `up.sql`


ALTER TABLE trekkie_runs ADD COLUMN gps_file UUID NOT NULL;
