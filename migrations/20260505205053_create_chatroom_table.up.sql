-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE chatroom(
    room_id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    room_name VARCHAR(100) UNIQUE DEFAULT NULL,
    description VARCHAR(300),
    is_direct BOOLEAN NOT NULL DEFAULT false,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX is_direct_idx ON chatroom (is_direct);