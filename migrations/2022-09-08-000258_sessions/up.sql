-- Your SQL goes here

CREATE TABLE sessions (
		id BIGSERIAL PRIMARY KEY,
		owner UUID REFERENCES users(id) NOT NULL,
		start_time timestamp NOT NULL,
        token VarChar NOT NULL
);

