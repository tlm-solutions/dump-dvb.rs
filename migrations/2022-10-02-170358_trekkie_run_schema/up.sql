-- Your SQL goes here

DELETE FROM trekkie_runs WHERE start_time IS NULL;
DELETE FROM trekkie_runs WHERE end_time IS NULL;
DELETE FROM trekkie_runs WHERE run IS NULL;
DELETE FROM trekkie_runs WHERE line IS NULL;
DELETE FROM trekkie_runs WHERE gps_file IS NULL;

ALTER TABLE trekkie_runs ALTER COLUMN start_time SET NOT NULL;
ALTER TABLE trekkie_runs ALTER COLUMN end_time SET NOT NULL;
ALTER TABLE trekkie_runs ALTER COLUMN run SET NOT NULL;
ALTER TABLE trekkie_runs ALTER COLUMN line SET NOT NULL;
ALTER TABLE trekkie_runs ALTER COLUMN gps_file SET NOT NULL;

