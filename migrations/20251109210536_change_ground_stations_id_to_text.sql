-- Change ground_stations.id from bigint to text (string) and make it unique
-- This migration handles the conversion and updates foreign key references

-- Step 1: Add a temporary text column for the new ID
ALTER TABLE ground_stations ADD COLUMN id_text TEXT;

-- Step 2: Populate the new column with string versions of the old IDs
UPDATE ground_stations SET id_text = id::TEXT;

-- Step 3: Add temporary columns to tables that reference ground_stations
ALTER TABLE telemetry ADD COLUMN gs_id_text TEXT;
ALTER TABLE jobs ADD COLUMN gs_id_text TEXT;

-- Step 4: Populate the temporary columns with string versions
UPDATE telemetry SET gs_id_text = gs_id::TEXT;
UPDATE jobs SET gs_id_text = gs_id::TEXT;

-- Step 5: Drop foreign key constraints (if any exist)
ALTER TABLE telemetry DROP CONSTRAINT IF EXISTS telemetry_gs_id_fkey;
ALTER TABLE jobs DROP CONSTRAINT IF EXISTS jobs_gs_id_fkey;

-- Step 6: Drop the old numeric columns
ALTER TABLE telemetry DROP COLUMN gs_id;
ALTER TABLE jobs DROP COLUMN gs_id;
ALTER TABLE ground_stations DROP COLUMN id;

-- Step 7: Rename the text columns to the original names
ALTER TABLE ground_stations RENAME COLUMN id_text TO id;
ALTER TABLE telemetry RENAME COLUMN gs_id_text TO gs_id;
ALTER TABLE jobs RENAME COLUMN gs_id_text TO gs_id;

-- Step 8: Set the new id column as PRIMARY KEY and make it UNIQUE
ALTER TABLE ground_stations ADD PRIMARY KEY (id);

-- Step 9: Add NOT NULL constraints
ALTER TABLE ground_stations ALTER COLUMN id SET NOT NULL;
ALTER TABLE telemetry ALTER COLUMN gs_id SET NOT NULL;
ALTER TABLE jobs ALTER COLUMN gs_id SET NOT NULL;

-- Step 10: Re-add foreign key constraints
ALTER TABLE telemetry ADD CONSTRAINT telemetry_gs_id_fkey 
    FOREIGN KEY (gs_id) REFERENCES ground_stations(id) ON DELETE CASCADE;

ALTER TABLE jobs ADD CONSTRAINT jobs_gs_id_fkey 
    FOREIGN KEY (gs_id) REFERENCES ground_stations(id) ON DELETE CASCADE;

-- Step 11: Drop the old sequence since we're using text IDs now
DROP SEQUENCE IF EXISTS ground_stations_id_seq;
