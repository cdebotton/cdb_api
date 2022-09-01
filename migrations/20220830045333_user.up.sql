-- create table users (
--   id uuid primary key not null default uuid_generate_v4(),
--   username text unique not null,
--   email text unique not null,
--   first_name text,
--   last_name text,
--   last_login timestamp with time zone,
--   created_at timestamp with time zone not null default now(),
--   updated_at timestamp with time zone
-- );

-- create trigger set_timestamp
-- before update on users
-- for each row
-- execute procedure set_updated_at();



