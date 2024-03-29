CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE article (
    /* Identifiers */
    id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    slug VARCHAR(200) NOT NULL,
    
    /* Content */
    title VARCHAR(100) NOT NULL,
    content TEXT NOT NULL,
    author VARCHAR(100) NOT NULL,

    /* Meta */
    cover_image TEXT NOT NULL,
    content_desc TEXT NOT NULL,
    
    featured BOOLEAN NOT NULL,
    published BOOLEAN NOT NULL,
    is_furry BOOLEAN NOT NULL DEFAULT false,

    /* Date */
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);