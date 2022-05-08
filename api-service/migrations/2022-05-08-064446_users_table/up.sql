CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  display_name VARCHAR NOT NULL,
  username CHAR(128) NOT NULL UNIQUE,
  password_hash CHAR(256) NOT NULL,
  is_admin BOOLEAN NOT NULL
)
