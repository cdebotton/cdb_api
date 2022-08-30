create table app_public."user" (
  id uuid primary key not null default uuid_generate_v4(),
  username text unique not null,
  email text unique not null,
  first_name text,
  last_name text,
  last_login timestamp with time zone,
  created_at timestamp with time zone not null default now(),
  updated_at timestamp with time zone
);

grant
  select
on table app_public."user"
to "anonymous";

grant
  select,
  update,
  insert,
  delete
on table app_public."user"
to "authenticated";

create trigger set_timestamp
before update on app_public."user"
for each row
execute procedure app_private.set_updated_at();
