CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id                          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username                    VARCHAR(50) NOT NULL UNIQUE,
    email                       VARCHAR(255) NOT NULL UNIQUE,
    password_hash               TEXT NOT NULL,
    is_verified                 BOOLEAN NOT NULL DEFAULT false,
    verification_token          TEXT,
    verification_token_expires  TIMESTAMP WITH TIME ZONE,
    reset_token                 TEXT,
    reset_token_expires         TIMESTAMP WITH TIME ZONE,
    last_seen                   TIMESTAMP WITH TIME ZONE,
    created_at                  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at                  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX users_email_idx ON users (email);
CREATE INDEX users_username_idx ON users (username);