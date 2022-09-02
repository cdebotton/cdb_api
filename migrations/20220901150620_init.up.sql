-- Initial setup for the database

BEGIN;

-- Create the two schemas that the application will run within.
-- We want to avoid adding anything to the public schema.

CREATE SCHEMA app;
CREATE SCHEMA app_private;

-- Install pgcypto for hashing passwords

CREATE EXTENSION "pgcrypto";

-- Install uuid to use for unique IDs instead of serial keys.

CREATE EXTENSION "uuid-ossp";

-- Create the users table the stores all non-sensitive inofrmation.

CREATE TABLE app.users (
  id          uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
  first_name  TEXT,
  last_name   TEXT,
  created_at  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  updated_at  TIMESTAMP WITH TIME ZONE
);

COMMENT ON TABLE app.users IS 'A authenticatable user of the application.';
COMMENT ON COLUMN app.users.id IS 'The primary unique identifier for a user.';
COMMENT ON COLUMN app.users.first_name IS 'The user’s first name.';
COMMENT ON COLUMN app.users.last_name IS 'The user’s last name.';
COMMENT ON COLUMN app.users.created_at IS 'The the time that the user was created.';
COMMENT ON COLUMN app.users.updated_at IS 'The last time the user was updated.';

-- Don't allow function execution by default from the public table from here on out.

ALTER DEFAULT PRIVILEGES REVOKE EXECUTE ON FUNCTIONS FROM public;

CREATE FUNCTION app_private.set_updated_at() RETURNS TRIGGER AS $$
BEGIN
  new.updated_at := CURRENT_TIMESTAMP;
  return new;
END;
$$ LANGUAGE plpgsql;


-- Add the updated_at trigger to the users table.

CREATE TRIGGER user_updated_at BEFORE UPDATE ON app.users
FOR EACH ROW EXECUTE PROCEDURE app_private.set_updated_at();

-- Create the accounts table with data that is private and should be inaccessible to unauthenticated users.

CREATE TABLE app_private.accounts (
  user_id           uuid PRIMARY KEY REFERENCES app.users(id) NOT NULL,
  last_login        TIMESTAMP WITH TIME ZONE,
  hashed_password   TEXT NOT NULL
);

COMMIT;
