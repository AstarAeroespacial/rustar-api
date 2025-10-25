-- Add migration script here

ALTER TABLE jobs
    ADD COLUMN commands TEXT[] NOT NULL;