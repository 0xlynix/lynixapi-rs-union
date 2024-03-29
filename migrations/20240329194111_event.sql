CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE event (
    /* Identifiers */
 	id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    slug VARCHAR(200) NOT NULL,

    /* Content */
    event_name VARCHAR(255) NOT NULL,
    event_image TEXT NOT NULL,
    event_description TEXT NOT NULL,
    event_location VARCHAR(255) NOT NULL,
    event_start_date TIMESTAMP WITH TIME ZONE NOT NULL,
    event_end_date TIMESTAMP WITH TIME ZONE NOT NULL, 

    /* Meta */
    featured BOOLEAN NOT NULL,
    is_furry BOOLEAN NOT NULL DEFAULT false,
    is_furry_convention BOOLEAN NOT NULL DEFAULT false,
    is_started BOOLEAN NOT NULL DEFAULT false,
    is_ended BOOLEAN NOT NULL DEFAULT false,
    is_canceled BOOLEAN NOT NULL DEFAULT false,
    is_visible BOOLEAN NOT NULL DEFAULT true,

    /* Date */
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);