-- Your SQL goes here




CREATE TABLE Loggers (
  id SERIAL PRIMARY KEY,
  app_id integer NOT NULL References apps (id),
  logger_type VARCHAR NOT NULL
)