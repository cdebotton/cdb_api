BEGIN;

DROP FUNCTION app.validate_refresh_token(uuid);

ALTER TYPE app.jwt_token ALTER ATTRIBUTE role TYPE TEXT;
ALTER TYPE app.jwt_token ALTER ATTRIBUTE user_id TYPE uuid;
ALTER TYPE app.jwt_token ALTER ATTRIBUTE refresh_token TYPE uuid;
ALTER TYPE app.jwt_token ALTER ATTRIBUTE  refresh_token_expires TYPE TIMESTAMP WITH TIME ZONE;

COMMIT;
