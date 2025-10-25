-- Add satellite column to ground stations table

-- Add migration script here
ALTER TABLE ground_stations
    ADD COLUMN satellite TEXT;

CREATE INDEX IF NOT EXISTS idx_ground_stations_satellite ON ground_stations (satellite);