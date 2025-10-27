-- Change ground_stations.location to latitude and longitude columns
-- Migration: split location into separate lat/lon coordinates

-- Step 1: Add new columns
ALTER TABLE ground_stations
ADD COLUMN latitude DOUBLE PRECISION,
ADD COLUMN longitude DOUBLE PRECISION;

-- Step 2: If there's existing data in location column and it represents latitude,
-- you could migrate it here. For example, if location was latitude:
-- UPDATE ground_stations SET latitude = location WHERE location IS NOT NULL;
-- If you need to preserve old data, adjust this accordingly

-- Step 3: Drop the old location column
ALTER TABLE ground_stations
DROP COLUMN location;

-- Optional: Add constraints or indexes if needed
-- Example: CREATE INDEX idx_ground_stations_coordinates ON ground_stations(latitude, longitude);
