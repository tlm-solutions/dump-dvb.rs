-- Your SQL goes here

ALTER TABLE organizations ADD COLUMN deactivated BOOLEAN;
UPDATE organizations SET deactivated=false;
ALTER TABLE organizations ALTER COLUMN deactivated SET NOT NULL;
