revoke execute on function app_public.register_account(text, text, text)
from "authenticated";

drop function app_public.register_account(text, text, text);
drop function app_public.authenticate(text, text);

drop type app_public.jwt_token;

drop table app_private.account;
