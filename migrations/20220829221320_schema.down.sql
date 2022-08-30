drop function app_private.set_updated_at();

alter default privileges
grant execute on functions
to public;

revoke usage
on schema app_public
from "anonymous", "authenticated";

drop role "anonymous";
drop role "authenticated";

drop schema app_public;
drop schema app_private;

drop extension "uuid-ossp";
drop extension "pgcrypto";
