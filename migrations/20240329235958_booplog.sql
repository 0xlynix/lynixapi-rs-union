CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE booplog (
    /* Identifiers */
    id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),

    token VARCHAR(200),
    event_slug VARCHAR(200),

    booped_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);