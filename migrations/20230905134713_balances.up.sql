-- Add up migration script here
create type btype as enum ('cash', 'card');

create table balances (
    id serial PRIMARY KEY,
    uid INT NOT NULL,
    btype btype NOT NULL,
    name VARCHAR(100) NOT NULL,
    iban VARCHAR(255),
    balance INT NOT NULL DEFAULT 0,
);
