-- Your SQL goes here
CREATE TABLE gossipers (
    id INTEGER PRIMARY KEY NOT NULL,
    discord_id BINARY NOT NULL,
    preferred_guild BINARY NULLABLE
)
