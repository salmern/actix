-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE IF NOT EXISTS public.user_profile (
    user_id uuid NOT NULL DEFAULT uuid_generate_v4(),
    name varchar NOT NULL,
    "role" varchar NULL,
    email varchar NOT NULL,
    "password" varchar NOT NULL,
    is_active boolean default true,
    created_date timestamp(0) NULL,
    updated_date timestamp(0) NULL,
    constraint user_table_unique_key unique(email)
);