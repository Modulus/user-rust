-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    comment VARCHAR NULL,
    active BOOLEAN NOT NULL DEFAULT 'f',
    pass_hash VARCHAR NOT NULL
);