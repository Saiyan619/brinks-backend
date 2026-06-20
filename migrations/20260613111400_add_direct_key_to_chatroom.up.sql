-- Add up migration script here
ALTER TABLE chatroom ADD COLUMN direct_key TEXT UNIQUE;