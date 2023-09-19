-- Add up migration script here
CREATE TYPE btype as enum ('cash', 'card');

CREATE TABLE balances (
    id serial PRIMARY KEY,
    uid INT NOT NULL,
    balance_type btype NOT NULL,
    name VARCHAR(100) NOT NULL,
    iban VARCHAR(255),
    balance INT NOT NULL DEFAULT 0
);
