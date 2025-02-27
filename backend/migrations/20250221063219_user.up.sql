-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    first_name varchar(20) NOT NULL,
    family_name varchar(20) NOT NULL,
    email varchar(255) NOT NULL,
    password varchar(255) NOT NULL
);
