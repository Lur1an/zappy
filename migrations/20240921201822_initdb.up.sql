-- Add up migration script here
CREATE TABLE IF NOT EXISTS click (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc(),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    ip_address inet NOT NULL,
    user_agent TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS ip_addr (
    inet PRIMARY KEY,
);
