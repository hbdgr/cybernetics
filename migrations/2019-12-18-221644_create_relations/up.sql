CREATE TABLE relations (
	hash BYTEA PRIMARY KEY NOT NULL UNIQUE,
	definition BYTEA NOT NULL,
	first_object BYTEA NOT NULL,
	second_object BYTEA NOT NULL
)
