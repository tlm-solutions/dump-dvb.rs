-- Your SQL goes here
CREATE TABLE tracy_runs (
		id BIGSERIAL PRIMARY KEY,
		start_time timestamp,
        end_time timestamp,
        line INT,
        run INT,
        gps_file TEXT,
		region BIGSERIAL REFERENCES regions(id),
		owner UUID REFERENCES users(id) NOT NULL,
        finished BOOL NOT NULL
);

