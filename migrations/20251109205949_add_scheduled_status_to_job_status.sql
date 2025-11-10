-- Add 'Scheduled' status to job_status enum
-- This must be done by adding the new value to the existing enum type

ALTER TYPE job_status ADD VALUE 'Scheduled' AFTER 'Received';
