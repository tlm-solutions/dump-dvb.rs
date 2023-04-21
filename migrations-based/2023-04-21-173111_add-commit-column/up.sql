-- Your SQL goes here

ALTER TABLE trekkie_runs ADD app_commit VARCHAR(40);
UPDATE trekkie_runs SET app_commit='0000000000000000000000000000000000000000';
ALTER TABLE trekkie_runs ALTER COLUMN app_commit SET NOT NULL;

ALTER TABLE trekkie_runs ADD app_name VARCHAR(40);
UPDATE trekkie_runs SET app_name ='unknown';
ALTER TABLE trekkie_runs ALTER COLUMN app_name SET NOT NULL;


