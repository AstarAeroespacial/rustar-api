-- Create enum type for job status
CREATE TYPE job_status AS ENUM ('Sent', 'Received', 'Started', 'Completed', 'Error');

-- Alter the jobs_status_updates table to use the enum
ALTER TABLE jobs_status_updates
ALTER COLUMN status TYPE job_status USING status::job_status;
