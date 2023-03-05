-- This file should undo anything in `up.sql`

CREATE TABLE sessions (
		owner UUID REFERENCES users(id) PRIMARY KEY,
		start_time timestamp NOT NULL,
        token VarChar NOT NULL
);

CREATE TABLE internal_stations(
    id UUID PRIMARY KEY REFERENCES stations(id), 
    wireguard_number INT
);

CREATE TABLE station_history (
        id BIGSERIAL PRIMARY KEY,
		changed_time TIMESTAMP NOT NULL,
        station_id UUID REFERENCES stations(id) NOT NULL,
		name TEXT NOT NULL,
		lat DOUBLE PRECISION NOT NULL,
		lon DOUBLE PRECISION NOT NULL,
		region BIGSERIAL REFERENCES regions(id) NOT NULL,
		approved BOOL NOT NULL,
		deactivated BOOL NOT NULL,
		public BOOL NOT NULL,
		radio INT,
		architecture INT,
		device INT,
		elevation DOUBLE PRECISION,
		telegram_decoder_version INT[],
		antenna Int
);

