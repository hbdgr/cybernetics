CREATE TABLE relations (
	id BIGSERIAL PRIMARY KEY,
	object_definition_id BIGSERIAL NOT NULL,
	first_object_id BIGSERIAL NOT NULL,
	second_object_id BIGSERIAL NOT NULL
)
