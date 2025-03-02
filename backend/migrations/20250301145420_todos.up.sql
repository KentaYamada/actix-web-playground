-- Add up migration script here
CREATE TABLE IF NOT EXISTS todos (
    id serial PRIMARY KEY,
    status integer NOT NULL,
    title varchar(256) NOT NULL,
    detail text NULL,
    created_at timestamp NOT NULL,
    modified_at timestamp NOT NULL
);
