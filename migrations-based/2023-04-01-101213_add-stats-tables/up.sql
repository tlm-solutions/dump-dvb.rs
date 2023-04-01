-- Your SQL goes here

CREATE TABLE region_statistics (
	id BIGINT PRIMARY KEY REFERENCES regions(id),
	total_telegrams BIGINT NOT NULL,
	month_telegrams BIGINT NOT NULL,
	week_telegrams BIGINT NOT NULL,
	day_telegrams BIGINT NOT NULL,
	total_gps BIGINT NOT NULL,
	month_gps BIGINT NOT NULL,
	week_gps BIGINT NOT NULL,
	day_gps BIGINT NOT NULL
);

CREATE TABLE station_statistics (
	id UUID PRIMARY KEY REFERENCES stations(id),
	total_telegrams BIGINT NOT NULL,
	month_telegrams BIGINT NOT NULL,
	week_telegrams BIGINT NOT NULL,
	day_telegrams BIGINT NOT NULL
);

CREATE TABLE user_statistics (
	id UUID PRIMARY KEY REFERENCES users(id),
	total_gps BIGINT NOT NULL,
	month_gps BIGINT NOT NULL,
	week_gps BIGINT NOT NULL,
	day_gps BIGINT NOT NULL
);


