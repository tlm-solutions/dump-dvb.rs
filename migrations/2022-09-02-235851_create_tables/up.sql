-- Your SQL goes here
CREATE TABLE users (
		id UUID PRIMARY KEY,
		name TEXT NOT NULL,
		email TEXT NOT NULL,
		password VARCHAR(100) NOT NULL,
		role INT NOT NULL,
		email_setting INT NOT NULL,
		deactivated BOOL NOT NULL
);

CREATE TABLE regions (
		id BIGSERIAL PRIMARY KEY,
		name TEXT NOT NULL,
		transport_company TEXT NOT NULL,
		regional_company TEXT,
		frequency BIGINT,
		r09_type INT,
		encoding INT
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
		telegram_decoder_version INT[],
		antenna Int
);

create table r09_telegrams (
		id BIGSERIAL primary key,
		time timestamp not null,
		station UUID REFERENCES stations(id) NOT NULL,
		telegram_type int8 not null,
		delay int,
		reporting_point int not null,
		junction int not null,
		direction int2 not null,
		request_status int2 not null,
		priority int2,
		direction_request int2,
		line int,
		run_number int,
		destination_number int,
		train_length int2,
		vehicle_number int,
		operator int2
);

create table raw_telegrams (
		id BIGSERIAL primary key,
		time timestamp not null,
		station UUID REFERENCES stations(id) NOT NULL,
		telegram_type int8 not null,
		data bytea not null
);
