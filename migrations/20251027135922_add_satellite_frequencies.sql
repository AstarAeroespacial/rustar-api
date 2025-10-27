-- Add downlink and uplink frequency columns to satellites table
ALTER TABLE satellites
ADD COLUMN downlink_frequency DOUBLE PRECISION NOT NULL DEFAULT 0,
ADD COLUMN uplink_frequency DOUBLE PRECISION NOT NULL DEFAULT 0;
