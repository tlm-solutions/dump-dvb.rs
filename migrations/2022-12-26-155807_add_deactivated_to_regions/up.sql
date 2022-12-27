-- step 1: add column with nullable constraint
ALTER TABLE regions ADD COLUMN deactivated BOOLEAN;

-- step 2: initialize every row with value
UPDATE regions SET deactivated=false;

-- step 3: add not null constraint
ALTER TABLE regions ALTER COLUMN deactivated SET NOT NULL;
