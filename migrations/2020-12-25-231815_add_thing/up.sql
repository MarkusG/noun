-- Your SQL goes here
CREATE TABLE thing (
	id SERIAL PRIMARY KEY,
	name TEXT NOT NULL,
	created TIMESTAMP NOT NULL DEFAULT now(),
	tags TEXT[]
);
