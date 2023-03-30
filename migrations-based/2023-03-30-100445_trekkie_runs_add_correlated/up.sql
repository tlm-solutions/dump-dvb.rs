-- Your SQL goes here
ALTER TABLE trekkie_runs ADD COLUMN correlated BOOLEAN;
UPDATE trekkie_runs SET correlated=false;
ALTER TABLE trekkie_runs ALTER COLUMN correlated SET NOT NULL;
