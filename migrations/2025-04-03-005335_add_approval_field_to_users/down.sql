-- This file should undo anything in `up.sql`
ALTER TABLE users
DROP COLUMN approved_at,
DROP COLUMN approved_by;