CREATE TABLE IF NOT EXISTS guilds (
    id BIGINT PRIMARY KEY NOT NULL,
    prefix TEXT
);


CREATE TABLE IF NOT EXISTS notes (
    guild_id BIGINT PRIMARY KEY,
    note_id SERIAL PRIMARY KEY NOT NULL,
    name TEXT,
    content TEXT

);