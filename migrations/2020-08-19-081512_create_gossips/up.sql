-- Your SQL goes here
CREATE TABLE gossips (
    id INTEGER PRIMARY KEY NOT NULL,
    msg VARCHAR NOT NULL,
    added TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    guild_id BINARY NOT NULL
)
