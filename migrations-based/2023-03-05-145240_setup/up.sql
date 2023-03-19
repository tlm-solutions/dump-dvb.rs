-- Your SQL goes here

CREATE TABLE users (
    id uuid PRIMARY KEY,
    name TEXT,
    email TEXT,
    password VARCHAR(100) NOT NULL,
    role INT NOT NULL,
    email_setting INT,
    deactivated BOOLEAN NOT NULL
);

CREATE TABLE regions (
    id BIGSERIAL PRIMARY KEY,
    name text NOT NULL,
    transport_company TEXT NOT NULL,
    regional_company TEXT,
    frequency BIGINT,
    r09_type INT,
    encoding INT,
    deactivated BOOLEAN NOT NULL
);

CREATE TABLE stations (
    id UUID PRIMARY KEY,
    token VARCHAR(36),
    name TEXT NOT NULL,
    lat DOUBLE PRECISION NOT NULL,
    lon DOUBLE PRECISION NOT NULL,
    region BIGSERIAL REFERENCES regions(id) NOT NULL,
    owner UUID REFERENCES users(id) NOT NULL,
    approved BOOL NOT NULL,
    deactivated BOOL NOT NULL,
    public BOOL NOT NULL,
    radio INT,
    architecture INT,
    device INT,
    elevation DOUBLE PRECISION,
    antenna INT,
    telegram_decoder_version TEXT,
    notes TEXT
);

CREATE TABLE trekkie_runs (
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    line INT NOT NULL,
    run INT NOT NULL,
    region BIGSERIAL REFERENCES regions(id) NOT NULL,
    owner UUID REFERENCES users(id) NOT NULL,
    finished BOOLEAN NOT NULL,
    id UUID PRIMARY KEY
);

CREATE TABLE gps_points (
    id BIGSERIAL PRIMARY KEY,
    trekkie_run UUID REFERENCES trekkie_runs(id) NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    lat DOUBLE PRECISION NOT NULL,
    lon DOUBLE PRECISION NOT NULL,
    elevation DOUBLE PRECISION,
    accuracy DOUBLE PRECISION,
    vertical_accuracy DOUBLE PRECISION,
    bearing DOUBLE PRECISION,
    speed DOUBLE PRECISION
);

CREATE TABLE raw_telegrams (
    id BIGSERIAL PRIMARY KEY,
    time TIMESTAMP NOT NULL,
    station UUID REFERENCES stations(id) NOT NULL,
    telegram_type BIGINT NOT NULL,
    data BYTEA NOT NULL
);

CREATE TABLE r09_telegrams (
    id BIGSERIAL PRIMARY KEY,
    time TIMESTAMP NOT NULL,
    station UUID REFERENCES stations(id) NOT NULL,
    telegram_type BIGINT NOT NULL,
    delay INT,
    reporting_point INT NOT NULL,
    junction INT NOT NULL,
    direction SMALLINT NOT NULL,
    request_status SMALLINT NOT NULL,
    priority SMALLINT,
    direction_request SMALLINT,
    line INT,
    run_number INT,
    destination_number INT,
    train_length INT,
    vehicle_number INT,
    operator SMALLINT,
    region BIGSERIAL REFERENCES regions(id) NOT NULL
);

