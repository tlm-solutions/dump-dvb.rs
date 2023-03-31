-- Your SQL goes here

ALTER TABLE regions ADD COLUMN lat FLOAT;
ALTER TABLE regions ADD COLUMN lon FLOAT;
ALTER TABLE regions ADD COLUMN zoom FLOAT;
ALTER TABLE regions ADD COLUMN work_in_progress BOOLEAN;

UPDATE regions SET lat=0.0, lon=0.0, zoom=11.0, work_in_progress=false;

ALTER TABLE regions ALTER COLUMN lat SET NOT NULL;
ALTER TABLE regions ALTER COLUMN lon SET NOT NULL;
ALTER TABLE regions ALTER COLUMN zoom SET NOT NULL;
ALTER TABLE regions ALTER COLUMN work_in_progress SET NOT NULL;

