-- Your SQL goes here
CREATE TABLE note (
	id SERIAL PRIMARY KEY,
	thing_id INT NOT NULL REFERENCES thing(id),
	created TIMESTAMP NOT NULL DEFAULT now(),
	content TEXT
);
