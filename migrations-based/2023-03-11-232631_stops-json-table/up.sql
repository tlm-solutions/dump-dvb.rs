-- Your SQL goes here

-- This creates table for transmission locations used in production
CREATE TABLE r09_transmission_locations (
	id BIGSERIAL PRIMARY KEY,
	region BIGSERIAL REFERENCES regions(id) NOT NULL,
	report_location BIGINT NOT NULL,
	lat DOUBLE PRECISION NOT NULL,
	lon DOUBLE PRECISION NOT NULL
);

-- This is contains undeduped locations of transmission positions, to be used
-- internally for data wrangling
CREATE TABLE r09_transmission_locations_raw (
	id BIGSERIAL PRIMARY KEY,
	region BIGSERIAL REFERENCES regions(id) NOT NULL,
	report_location BIGINT NOT NULL,
	lat DOUBLE PRECISION NOT NULL,
	lon DOUBLE PRECISION NOT NULL,
	trekkie_run UUID REFERENCES trekkie_runs(id) NOT NULL,
	run_owner UUID REFERENCES users(id) NOT NULL
);

