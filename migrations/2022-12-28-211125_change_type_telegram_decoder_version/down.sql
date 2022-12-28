-- step 1: delete new column
ALTER TABLE stations DROP COLUMN telegram_decoder_version;

-- step 2: recreate old table with old datatype
ALTER TABLE stations ADD COLUMN telegram_decoder_version INT[];
