-- Your SQL goes here
create table users (
    id uuid default uuid_generate_v4() primary key not null,
    name varchar not null,
    email varchar not null,
    username varchar,
    password varchar not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now(),
    deleted_at timestamp
);