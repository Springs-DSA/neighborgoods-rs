ALTER TABLE item_transfers
ADD COLUMN steward_confirmed_at TIMESTAMP,
ADD COLUMN prev_steward_confirmed_at TIMESTAMP;