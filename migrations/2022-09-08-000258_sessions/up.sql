-- Your SQL goes here

CREATE TABLE sessions (
		owner UUID REFERENCES users(id) PRIMARY KEY,
		start_time timestamp NOT NULL,
        token VarChar NOT NULL
);

