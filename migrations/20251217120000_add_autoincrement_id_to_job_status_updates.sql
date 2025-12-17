-- Drop the existing primary key constraint on job_id
ALTER TABLE jobs_status_updates DROP CONSTRAINT IF EXISTS jobs_status_updates_pkey;

-- Add new auto-incrementing id column as primary key
ALTER TABLE jobs_status_updates ADD COLUMN id SERIAL PRIMARY KEY;

-- Note: job_id is now a regular column (not unique, not primary key)
-- Multiple status updates can reference the same job_id
