-- Your SQL goes here

CREATE TABLE organizations (
	    id UUID PRIMARY KEY,
	    name TEXT NOT NULL,
	    public BOOLEAN NOT NULL
);

CREATE TABLE org_users_relations (
	    id UUID PRIMARY KEY,
	    organization UUID REFERENCES organizations(id) NOT NULL,
	    user_id UUID REFERENCES users(id) NOT NULL,
	    ROLE INT NOT NULL
);

INSERT INTO organizations (id, name, public) VALUES ('53e643d7-c300-4de7-ab48-540d08a0cbc6', 'Community Organization', true);

ALTER TABLE stations ADD COLUMN organization UUID REFERENCES organizations(id);
UPDATE stations SET organization='53e643d7-c300-4de7-ab48-540d08a0cbc6';
ALTER TABLE stations ALTER COLUMN organization SET NOT NULL;

ALTER TABLE users ADD COLUMN admin BOOLEAN;
UPDATE users SET admin=false;
ALTER TABLE users ALTER COLUMN admin SET NOT NULL;

ALTER TABLE r09_transmission_locations ADD COLUMN ground_truth BOOLEAN;
UPDATE r09_transmission_locations SET ground_truth=false;
ALTER TABLE r09_transmission_locations ALTER COLUMN ground_truth SET NOT NULL;

ALTER TABLE users DROP COLUMN role;

