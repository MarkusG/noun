ALTER TABLE place
DROP CONSTRAINT place_check;

ALTER TABLE place
ADD CONSTRAINT has_location
CHECK (
	(lat IS NOT NULL AND long IS NOT NULL)
	OR address IS NOT NULL
)
