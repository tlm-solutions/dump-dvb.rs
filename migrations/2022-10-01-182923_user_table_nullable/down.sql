-- This file should undo anything in `up.sql`

DELETE FROM users WHERE name IS NULL;
DELETE FROM users WHERE email IS NULL;
DELETE FROM users WHERE password IS NULL;
DELETE FROM users WHERE email_setting IS NULL;

ALTER TABLE users ALTER COLUMN name SET NOT NULL;
ALTER TABLE users ALTER COLUMN email SET NOT NULL;
ALTER TABLE users ALTER COLUMN password SET NOT NULL;
ALTER TABLE users ALTER COLUMN email_setting SET NOT NULL;

