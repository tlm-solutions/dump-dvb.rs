-- Your SQL goes here

INSERT INTO users (id, name, password, admin, deactivated) VALUES ('00000000-0000-0000-0000-000000000000', 'Dummy McDummyface', 'lol', FALSE, TRUE);

ALTER TABLE organizations ADD COLUMN owner UUID REFERENCES users(id);
UPDATE organizations SET owner='00000000-0000-0000-0000-000000000000';
ALTER TABLE organizations ALTER COLUMN owner SET NOT NULL;
