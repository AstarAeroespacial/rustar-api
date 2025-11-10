-- Change satellites.id from bigint to text (string) and make it unique
-- This migration handles the conversion and updates foreign key references

-- Step 1: Add a temporary text column for the new ID
ALTER TABLE satellites ADD COLUMN id_text TEXT;

-- Step 2: Populate the new column with string versions of the old IDs
UPDATE satellites SET id_text = id::TEXT;

-- Step 3: Add temporary columns to tables that reference satellites
ALTER TABLE telemetry ADD COLUMN sat_id_text TEXT;
ALTER TABLE jobs ADD COLUMN sat_id_text TEXT;

-- Step 4: Populate the temporary columns with string versions
UPDATE telemetry SET sat_id_text = sat_id::TEXT;
UPDATE jobs SET sat_id_text = sat_id::TEXT;

-- Step 5: Drop foreign key constraints (if any exist)
-- Note: The original schema doesn't have explicit FK constraints, so this is precautionary
ALTER TABLE telemetry DROP CONSTRAINT IF EXISTS telemetry_sat_id_fkey;
ALTER TABLE jobs DROP CONSTRAINT IF EXISTS jobs_sat_id_fkey;

-- Step 6: Drop the old numeric columns
ALTER TABLE telemetry DROP COLUMN sat_id;
ALTER TABLE jobs DROP COLUMN sat_id;
ALTER TABLE satellites DROP COLUMN id;

-- Step 7: Rename the text columns to the original names
ALTER TABLE satellites RENAME COLUMN id_text TO id;
ALTER TABLE telemetry RENAME COLUMN sat_id_text TO sat_id;
ALTER TABLE jobs RENAME COLUMN sat_id_text TO sat_id;

-- Step 8: Set the new id column as PRIMARY KEY and make it UNIQUE
ALTER TABLE satellites ADD PRIMARY KEY (id);
-- Note: PRIMARY KEY already implies UNIQUE, but we're being explicit

-- Step 9: Add NOT NULL constraints
ALTER TABLE satellites ALTER COLUMN id SET NOT NULL;
ALTER TABLE telemetry ALTER COLUMN sat_id SET NOT NULL;
ALTER TABLE jobs ALTER COLUMN sat_id SET NOT NULL;

-- Step 10: Re-add foreign key constraints
ALTER TABLE telemetry ADD CONSTRAINT telemetry_sat_id_fkey 
    FOREIGN KEY (sat_id) REFERENCES satellites(id) ON DELETE CASCADE;

ALTER TABLE jobs ADD CONSTRAINT jobs_sat_id_fkey 
    FOREIGN KEY (sat_id) REFERENCES satellites(id) ON DELETE CASCADE;

-- Step 11: Drop the old sequence since we're using text IDs now
DROP SEQUENCE IF EXISTS satellites_id_seq;
