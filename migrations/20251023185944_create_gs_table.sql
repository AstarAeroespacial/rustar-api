-- Create table for ground stations
-- Add migration script here
CREATE TABLE IF NOT EXISTS ground_stations (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    latitude REAL NOT NULL,
    longitude REAL NOT NULL,
    altitude INTEGER NOT NULL
);
