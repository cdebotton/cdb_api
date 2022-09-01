-- https://dba.stackexchange.com/questions/115972/how-do-you-revoke-create-table-from-a-user-on-postgresql-9-4
-- https://www.tangramvision.com/blog/hands-on-with-postgresql-authorization-part-1-roles-and-grants#why-use-postgresql-authorization

CREATE ROLE role_admin;
CREATE ROLE role_public;
CREATE SCHEMA private;

GRANT USAGE ON SCHEMA public TO role_public, role_admin;
GRANT USAGE ON SCHEMA private TO role_admin;

create extension "uuid-ossp" with schema public;
create extension "pgcrypto" with schema public;

revoke all on schema private from role_public;

create function set_updated_at() returns trigger as $$
begin
  new.updated_at := current_timestamp;
  return new;
end;
$$ language plpgsql;

-- revoke all privileges on all tables in schema private from role_public;

-- alter default privileges revoke insert, update, delete from public;
-- grant usage on schema public to role_public, role_admin;
-- grant all on schema auth to "authenticated";
