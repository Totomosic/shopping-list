-- Your SQL goes here

CREATE TABLE shopping_item (
  id SERIAL PRIMARY KEY,
  name VARCHAR(128) NOT NULL,
  description VARCHAR(512),
  image_url VARCHAR(256),
  default_unit_type VARCHAR(32) NOT NULL
)
