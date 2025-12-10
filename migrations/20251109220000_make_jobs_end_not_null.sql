-- Make the "end" column in jobs table NOT NULL
-- This ensures all jobs must have a defined end time

ALTER TABLE jobs
ALTER COLUMN "end" SET NOT NULL;
