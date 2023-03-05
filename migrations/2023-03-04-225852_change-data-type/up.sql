-- Your SQL goes here

CREATE EXTENSION "uuid-ossp";

-- Add Column
ALTER TABLE trekkie_runs ADD COLUMN new_id UUID;
-- Fill Column
UPDATE trekkie_runs SET new_id = uuid_generate_v4();

-- Change Constraints 
ALTER TABLE trekkie_runs DROP CONSTRAINT tracy_runs_pkey CASCADE;
ALTER TABLE trekkie_runs ADD CONSTRAINT trekkie_runs_pkey PRIMARY KEY (new_id);

-- DROP old column
ALTER TABLE trekkie_runs DROP COLUMN id;

-- Rename Column
ALTER TABLE trekkie_runs RENAME COLUMN new_id TO id;

ALTER TABLE gps_points ADD CONSTRAINT gps_points_trekkie_run_fk FOREIGN KEY (trekkie_run) REFERENCES trekkie_runs (id);

DROP EXTENSION "uuid-ossp";

