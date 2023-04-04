-- Your SQL goes here
ALTER TABLE r09_transmission_locations ADD CONSTRAINT unique_region_position UNIQUE (reporting_point, region);
