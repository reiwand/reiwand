DROP TABLE sync_blobs;
DROP TABLE mailbox;
DROP TABLE sync_records;
DROP TABLE sync_heads;
ALTER TABLE devices DROP COLUMN enc_key;
ALTER TABLE devices DROP COLUMN sig_key;
ALTER TABLE device_codes DROP COLUMN enc_key;
ALTER TABLE device_codes DROP COLUMN sig_key;
UPDATE tokens SET scopes = trim(replace(' ' || scopes || ' ', ' sync ', ' account '));
