-- create table auth.accounts (
--   user_id uuid primary key not null references users(id) on delete cascade,
--   hashed_password text not null
-- );

-- create function register_account(
--   username text,
--   email text,
--   password text
-- ) returns users as $$
--   declare
--     new_user users;
--   begin
--     insert into users(username, email)
--     values(register_account.username, register_account.email)
--     returning * into new_user;
  
--     insert into auth.account(user_id, hashed_password)
--     values(new_user.id, crypt(register_account.password, gen_salt('bf')));

--     return new_user;
--   end;
-- $$ language plpgsql strict security definer;

-- grant execute on function register_account(text, text, text) to "authenticated";

-- create type jwt_token as (
--   role text,
--   user_id uuid
-- );

-- create function authenticate(
--   email text,
--   password text
-- ) returns jwt_token as $$
--   declare
--     token jwt_token;
--   begin
--     select 'authenticated', account.user_id
--     from users
--     left join auth.account
--     on account.user_id = "user".id
--     where "user".email = authenticate.email
--     and account.hashed_password = crypt(authenticate.password, account.hashed_password)
--     into token;

--     update users
--     set last_login = now()
--     where "user".email = authenticate.email;

--     return token;
--   end;
-- $$ language plpgsql strict security definer;

-- grant execute on function authenticate(text, text) to "anonymous";
