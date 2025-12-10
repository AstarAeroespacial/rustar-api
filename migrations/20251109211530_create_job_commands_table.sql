-- Create job_commands table to store commands for each job
-- This normalizes the commands array into a proper relational structure

CREATE TABLE job_commands (
    job_id BIGINT NOT NULL,
    command TEXT NOT NULL,
    CONSTRAINT job_commands_job_id_fkey 
        FOREIGN KEY (job_id) 
        REFERENCES jobs(id) 
        ON DELETE CASCADE
);

-- Create index for faster lookups by job_id
CREATE INDEX idx_job_commands_job_id ON job_commands(job_id);