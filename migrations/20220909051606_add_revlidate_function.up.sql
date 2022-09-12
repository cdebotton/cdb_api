BEGIN;

-- Make all JWT fields non nullable
ALTER TYPE app.jwt_token ALTER ATTRIBUTE role TYPE TEXT;
ALTER TYPE app.jwt_token ALTER ATTRIBUTE user_id TYPE uuid;
ALTER TYPE app.jwt_token ALTER ATTRIBUTE refresh_token TYPE uuid;
ALTER TYPE app.jwt_token ALTER ATTRIBUTE  refresh_token_expires TYPE TIMESTAMP WITH TIME ZONE;


-- Add function to validate a refresh token.
CREATE FUNCTION app.validate_refresh_token(req_token uuid)
RETURNS app.jwt_token AS
$BODY$
DECLARE
  new_jwt app.jwt_token;
BEGIN
  UPDATE app_private.accounts
  SET
    refresh_token = uuid_generate_v4(),
    refresh_token_expires = NOW() + INTERVAL '5 days'
  WHERE
    app_private.accounts.refresh_token = req_token
  RETURNING 'admin', user_id, refresh_token, refresh_token_expires
  INTO new_jwt;

  IF NOT FOUND THEN
    RETURN NULL;
  ELSE
    RETURN new_jwt;
  END IF;
END;
$BODY$
  LANGUAGE plpgsql
  STRICT SECURITY DEFINER;

COMMENT ON FUNCTION app.validate_refresh_token(uuid) IS 'Function that validates a refresh token, updating the stored refresh token and setting a new expiration date.';

COMMIT;
