-- This file should undo anything in `up.sql`

ALTER TABLE trekkie_runs DROP COLUMN app_name;
ALTER TABLE trekkie_runs DROP COLUMN app_commit;


