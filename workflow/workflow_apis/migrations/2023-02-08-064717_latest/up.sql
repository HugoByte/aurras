-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table userss(
    id uuid default uuid_generate_v4() primary key,
    username varchar not null unique,
    email varchar not null unique,
    password_hash varchar not null unique,
    full_name varchar not null,
    actions text[] NOT NULL, 
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);

create table action_details(
    id  SERIAL primary key,
    rule varchar unique not null,
    action varchar not null,
    trigger varchar not null,
    active_status BOOLEAN NOT NULL DEFAULT 'f',
    url varchar not null,
    auth varchar not null,
    namespace varchar not null,
    user_id uuid  not null,
    foreign key (user_id) references userss(id)
);