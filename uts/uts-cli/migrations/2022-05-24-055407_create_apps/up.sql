-- Your SQL goes here

CREATE TABLE Apps (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  telegram_chat_id TEXT,
  token VARCHAR
)