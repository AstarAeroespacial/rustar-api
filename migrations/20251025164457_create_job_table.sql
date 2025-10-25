-- Add migration script here

CREATE TABLE IF NOT EXISTS jobs (
    id TEXT PRIMARY KEY NOT NULL,
    gs_id TEXT NOT NULL,
    sat_id TEXT NOT NULL,
    start_time INTEGER NOT NULL,
    end_time INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_jobs_gs_id ON jobs (gs_id);
CREATE INDEX IF NOT EXISTS idx_jobs_sat_id ON jobs (sat_id);