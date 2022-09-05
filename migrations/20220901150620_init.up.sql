-- Initial setup for the database

BEGIN;

-- Create the two schemas that the application will run within.
-- We want to avoid adding anything to the public schema.

CREATE SCHEMA app;
CREATE SCHEMA app_private;

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

CREATE TRIGGER _100_user_updated_at BEFORE UPDATE ON app.users
FOR EACH ROW EXECUTE PROCEDURE app_private.set_updated_at();

-- Create the accounts table with data that is private and should be inaccessible to unauthenticated users.

CREATE TABLE app_private.accounts (
  user_id               uuid PRIMARY KEY REFERENCES app.users(id) ON DELETE CASCADE NOT NULL,
  email                 TEXT UNIQUE NOT NULL,
  last_login            TIMESTAMP WITH TIME ZONE,
  refresh_token         TEXT UNIQUE,
  refresh_token_expires TIMESTAMP WITH TIME ZONE,
  hashed_password       TEXT NOT NULL
);


COMMENT ON TABLE app_private.accounts IS 'A table containing sensitive account data.';
COMMENT ON COLUMN app_private.accounts.user_id IS 'The primary key that references the id of the user it belongs to.';
COMMENT ON COLUMN app_private.accounts.last_login IS 'The last time the account has been logged in.';
COMMENT ON COLUMN app_private.accounts.refresh_token IS 'A randomly generated token to refresh to current JSON Web Token.';
COMMENT ON COLUMN app_private.accounts.refresh_token_expires IS 'The date when the refresh token expires.';
COMMENT ON COLUMN app_private.accounts.hashed_password IS 'The encrypted password.';

-- Install pgcypto for hashing passwords

CREATE EXTENSION "pgcrypto";

-- Create the function to simulatenously insert a new user and register the account with an ecrypted password.

CREATE FUNCTION app.register_user(
  first_name TEXT,
  last_name TEXT,
  email TEXT,
  password TEXT
) RETURNS app.users AS $$
  DECLARE
    new_user app.users;
  BEGIN
    INSERT INTO app.users (first_name, last_name)
    VALUES (first_name, last_name)
    RETURNING * INTO new_user;

    INSERT INTO app_private.accounts (user_id, email, hashed_password, refresh_token, refresh_token_expires)
    VALUES (new_user.id, email, crypt(password, gen_salt('bf')), uuid_generate_v4(), NOW() + INTERVAL '5 days');

    RETURN new_user;
  END;
$$ LANGUAGE plpgsql STRICT SECURITY DEFINER;

COMMENT ON FUNCTION app.register_user(TEXT, TEXT, TEXT, TEXT) IS 'Register a new user to the application';

-- Create a token type

CREATE TYPE app.jwt_token as (
  role TEXT,
  user_id uuid,
  refresh_token uuid,
  refresh_token_expires TIMESTAMP WITH TIME ZONE
);

-- Create function to verify that a user is providing the correct credentials.

CREATE FUNCTION app.authenticate(
  email TEXT,
  password TEXT
) RETURNS app.jwt_token as $$
  SELECT ('admin', user_id, refresh_token, refresh_token_expires)::app.jwt_token
  FROM app_private.accounts
  WHERE accounts.email = $1
  AND accounts.hashed_password = crypt($2, accounts.hashed_password);
$$ LANGUAGE sql STRICT SECURITY DEFINER;

COMMENT ON FUNCTION app.authenticate(TEXT, TEXT) IS 'Create a JWT token to identify a user and provide permissions.';

-- Create roles for authenticated and unauthenticated users.

-- CREATE ROLE app_anonymous;
-- GRANT app_anonymous TO web_app;

-- CREATE ROLE app_user;
-- GRANT app_user TO web_app;

-- GRANT usage ON SCHEMA app to app_anonymous, app_user;
-- GRANT SELECT ON TABLE app.users TO app_anonymous, app_user;
-- GRANT INSERT, UPDATE, DELETE ON TABLE app.users TO app_user;

-- GRANT EXECUTE ON FUNCTION app.authenticate(TEXT, TEXT) TO app_anonymous, app_user;
-- GRANT EXECUTE ON FUNCTION app.register_user(TEXT, TEXT, TEXT, TEXT) TO app_anonymous;

COMMIT;
