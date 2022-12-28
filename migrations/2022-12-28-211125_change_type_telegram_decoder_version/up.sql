
-- step 1: delete old column
ALTER TABLE stations DROP COLUMN telegram_decoder_version;

-- step 2: recreate table with new datatype
ALTER TABLE stations ADD COLUMN telegram_decoder_version TEXT;

