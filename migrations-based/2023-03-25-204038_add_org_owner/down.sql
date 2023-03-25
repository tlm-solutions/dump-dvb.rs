-- This file should undo anything in `up.sql`
ALTER TABLE organization DROP COLUMN owner;

DELETE FROM users WHERE id='00000000-0000-0000-0000-000000000000';

