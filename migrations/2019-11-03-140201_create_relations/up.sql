CREATE TABLE relations (
	id BIGSERIAL PRIMARY KEY,
	relation_object_id BIGSERIAL NOT NULL,
	first_object_id BIGSERIAL NOT NULL,
	second_object_id BIGSERIAL NOT NULL
)
