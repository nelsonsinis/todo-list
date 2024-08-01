create extension if not exists "uuid-ossp";

CREATE TABLE tasks (
  id uuid default uuid_generate_v4() primary key not null,
  title varchar not null,
  description text,
  checked boolean not null default false,
  created_at timestamp not null default now(),
  updated_at timestamp not null default now(),
  deleted_at timestamp
);