-- Make ground_stations.latitude, ground_stations.longitude and ground_stations.altitude NOT NULL
-- and set safe defaults for existing rows.
-- This migration is safe to run on databases that already have the columns.

BEGIN;

-- Ensure no NULL values remain (set sensible defaults)
UPDATE public.ground_stations SET latitude = 0.0 WHERE latitude IS NULL;
UPDATE public.ground_stations SET longitude = 0.0 WHERE longitude IS NULL;
UPDATE public.ground_stations SET altitude = 0 WHERE altitude IS NULL;

-- Set defaults so new rows get values if omitted
ALTER TABLE public.ground_stations
  ALTER COLUMN latitude SET DEFAULT 0.0,
  ALTER COLUMN longitude SET DEFAULT 0.0,
  ALTER COLUMN altitude SET DEFAULT 0;

-- Now apply NOT NULL constraints
ALTER TABLE public.ground_stations
  ALTER COLUMN latitude SET NOT NULL,
  ALTER COLUMN longitude SET NOT NULL,
  ALTER COLUMN altitude SET NOT NULL;


COMMIT;
