-- Add up migration script here
CREATE TABLE users (
   id serial PRIMARY KEY,
   username VARCHAR(100) UNIQUE NOT NULL,
   email VARCHAR(100) UNIQUE NOT NULL,
   password VARCHAR(255) NOT NULL,
   status BOOLEAN NOT NULL DEFAULT TRUE
);
