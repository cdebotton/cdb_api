REVOKE USAGE ON SCHEMA public FROM role_public, role_admin;
REVOKE USAGE ON SCHEMA private FROM role_admin;

DROP SCHEMA private;

DROP ROLE role_admin;
DROP ROLE role_public;

DROP EXTENSION "uuid-ossp";
DROP EXTENSION "pgcrypto";

DROP FUNCTION set_updated_at();

-- alter default privileges grant execute on functions to public;

-- revoke usage on schema public from "anonymous", "authenticated";
-- revoke all on schema auth from "authenticated";

-- DROP ROLE "anonymous";
-- drop role "authenticated";

-- drop schema auth;

