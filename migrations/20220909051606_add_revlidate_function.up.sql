BEGIN;

CREATE FUNCTION app.revalidate(req_token uuid) RETURNS app.jwt_token AS $$
  DECLARE
    new_jwt app.jwt_token;
  BEGIN
    UPDATE app_private.accounts
    SET
      refresh_token = uuid_generate_v4(),
      refresh_token_expires = NOW() + INTERVAL '5 days'
    WHERE app_private.accounts.refresh_token = req_token
    RETURNING 'admin', user_id, refresh_token, refresh_token_expires
    INTO new_jwt;

    RETURN new_jwt;
  END;
$$ LANGUAGE plpgsql STRICT SECURITY DEFINER;

COMMIT;
