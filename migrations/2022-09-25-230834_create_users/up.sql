-- Your SQL goes here
CREATE TABLE Users (
  id SERIAL PRIMARY KEY,
  display_name VARCHAR(140),
  source VARCHAR(140),
  external_id VARCHAR(140)
)