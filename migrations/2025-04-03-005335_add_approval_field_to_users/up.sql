-- Your SQL goes here
ALTER TABLE users
ADD COLUMN approved_at TIMESTAMP,
ADD COLUMN approved_by UUID REFERENCES users(id);