create schema app_public;
create schema app_private;

create role "anonymous";
create role "authenticated";

grant "anonymous"
to cdb_api;


grant "authenticated"
to cdb_api;

alter default privileges
revoke execute on functions
from public;

grant usage
on schema app_public
to "anonymous", "authenticated";

create function app_private.set_updated_at() returns trigger as $$
begin
  new.updated_at := current_timestamp;
  return new;
end;
$$ language plpgsql;

create extension "uuid-ossp";
create extension "pgcrypto";
