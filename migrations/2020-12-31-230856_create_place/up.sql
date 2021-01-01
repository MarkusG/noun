CREATE TABLE place (
	id SERIAL PRIMARY KEY,
	-- I would like to use Postgres' POINT type here, but diesel doesn't
	-- support it as of 1.4
	lat DOUBLE PRECISION,
	long DOUBLE PRECISION,
	address TEXT,
	name TEXT,
	tags TEXT[],
	description TEXT NOT NULL,
	recorded TIMESTAMP NOT NULL DEFAULT now()

	CHECK ((lat IS NOT NULL AND long IS NOT NULL) OR address IS NOT NULL)
)
