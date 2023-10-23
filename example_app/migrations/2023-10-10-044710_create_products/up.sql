-- Your SQL goes here
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    cost double precision NOT NULL,
    active BOOLEAN NOT NULL DEFAULT FALSE
)
