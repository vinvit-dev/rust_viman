-- Your SQL goes here
CREATE TABLE users (
   id SERIAL PRIMARY KEY,
   username VARCHAR(100) NOT NULL,
   email VARCHAR(100) NOT NULL,
   password VARCHAR(255) NOT NULL,
   status BOOLEAN NOT NULL DEFAULT TRUE
);