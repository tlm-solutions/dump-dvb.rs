-- Your SQL goes here

INSERT INTO users (id, name, password, admin, deactivated) VALUES ('00000000-0000-0000-0000-000000000000', 'Dummy McDummyface', 'lol', FALSE, TRUE);

ALTER TABLE organization ADD COLUMN owner UUID REFERENCES users(id);
UPDATE organization SET owner='00000000-0000-0000-0000-000000000000';
ALTER TABLE organization ALTER COLUMN owner SET NOT NULL;
