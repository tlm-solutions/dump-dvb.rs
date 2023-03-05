CREATE TABLE gps_points (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    trekkie_run UUID NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    lat FLOAT NOT NULL,
    lon FLOAT NOT NULL,
    elevation FLOAT,
    accuracy FLOAT,
    vertical_accuracy FLOAT,
    bearing FLOAT,
    speed FLOAT
);
