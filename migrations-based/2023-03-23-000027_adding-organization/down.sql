-- This file should undo anything in `up.sql`

ALTER TABLE r09_transmission_locations DROP COLUMN ground_truth;

ALTER TABLE users DROP COLUMN admin;

ALTER TABLE stations DROP COLUMN organization;

DROP TABLE org_users_relation;

DROP TABLE organization;

ALTER TABLE users ADD COLUMN role INT;
UPDATE users SET role=6;
ALTER TABLE users ALTER COLUMN role SET NOT NULL;
