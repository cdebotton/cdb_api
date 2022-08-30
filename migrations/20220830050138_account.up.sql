create table app_private.account (
  user_id uuid primary key not null references app_public."user"(id) on delete cascade,
  hashed_password text not null
);


create function app_public.register_account(
  username text,
  email text,
  password text
) returns app_public."user" as $$
  declare
    new_user app_public."user";
  begin
    insert into app_public."user"(username, email)
    values(register_account.username, register_account.email)
    returning * into new_user;
  
    insert into app_private.account(user_id, hashed_password)
    values(new_user.id, "public".crypt(register_account.password, "public".gen_salt('bf')));

    return new_user;
  end;
$$ language plpgsql strict security definer;

grant execute on function app_public.register_account(text, text, text)
to "authenticated";

create type app_public.jwt_token as (
  role text,
  user_id uuid
);

create function app_public.authenticate(
  email text,
  password text
) returns app_public.jwt_token as $$
  declare
    token app_public.jwt_token;
  begin
    select 'authenticated', account.user_id
    from app_public."user"
    left join app_private.account
    on account.user_id = "user".id
    where "user".email = authenticate.email
    and account.hashed_password = "public".crypt(authenticate.password, account.hashed_password)
    into token;

    update app_public."user"
    set last_login = now()
    where "user".email = authenticate.email;

    return token;
  end;
$$ language plpgsql strict security definer;

grant execute on function app_public.authenticate(text, text) to "anonymous";
