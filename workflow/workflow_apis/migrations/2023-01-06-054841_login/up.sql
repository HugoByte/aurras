CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table userss(
    id uuid default uuid_generate_v4() primary key,
    username varchar not null unique,
    email varchar not null unique,
    password_hash varchar not null unique,
    full_name varchar not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);