-- Your SQL goes here

ALTER TABLE organization ADD COLUMN deactivated BOOLEAN;
UPDATE organization SET deactivated=false;
ALTER TABLE organization ALTER COLUMN deactivated SET NOT NULL;
