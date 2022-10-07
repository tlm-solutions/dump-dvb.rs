-- Your SQL goes here
DELETE FROM users WHERE password IS NULL;

ALTER TABLE users ALTER COLUMN password SET NOT NULL;

