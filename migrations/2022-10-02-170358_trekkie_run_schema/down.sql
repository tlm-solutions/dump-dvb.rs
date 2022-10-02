-- This file should undo anything in `up.sql`

ALTER TABLE trekkie_runs ALTER COLUMN start_time DROP NOT NULL;
ALTER TABLE trekkie_runs ALTER COLUMN end_time DROP NOT NULL;
ALTER TABLE trekkie_runs ALTER COLUMN line DROP NOT NULL;
ALTER TABLE trekkie_runs ALTER COLUMN run DROP NOT NULL;
ALTER TABLE trekkie_runs ALTER COLUMN gps_file DROP NOT NULL;

