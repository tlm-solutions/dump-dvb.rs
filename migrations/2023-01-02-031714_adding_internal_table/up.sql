-- Your SQL goes here

CREATE TABLE internal_stations(id UUID PRIMARY KEY REFERENCES stations(id), wireguard_number INT);

ALTER TABLE stations ADD COLUMN notes TEXT;

