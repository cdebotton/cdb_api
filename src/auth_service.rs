use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::Error, models::user::User};

#[derive(Debug)]
pub struct AuthService {
    pub user: Option<User>,
}

type Token = (String, Uuid, Uuid, DateTime<Utc>);

impl AuthService {
    pub async fn authenticate(
        pool: &PgPool,
        email: &String,
        password: &String,
    ) -> Result<Token, Error> {
        let call = sqlx::query!(
            // language=PostgreSQL
            r#"SELECT
                    role "role!",
                    user_id "user_id!",
                    refresh_token "refresh_token!",
                    refresh_token_expires "refresh_token_expires!"
                FROM app.authenticate($1, $2)"#,
            &email,
            &password
        )
        .fetch_one(pool)
        .await
        .map_err(|_| Error::WrongCredentials)?;

        Ok((
            call.role,
            call.user_id,
            call.refresh_token,
            call.refresh_token_expires,
        ))
    }

    pub async fn revalidate(pool: &PgPool, token: uuid::Uuid) -> Result<Token, Error> {
        let row = sqlx::query!(
            r#"SELECT
                role "role!",
                user_id "user_id!",
                refresh_token "refresh_token!",
                refresh_token_expires "refresh_token_expires!"
            FROM app.validate_refresh_token($1)"#,
            token
        )
        .fetch_one(pool)
        .await
        .map_err(|_| Error::WrongCredentials)?;

        Ok((
            row.role,
            row.user_id,
            row.refresh_token,
            row.refresh_token_expires,
        ))
    }

    pub async fn register(
        pool: &PgPool,
        first_name: Option<String>,
        last_name: Option<String>,
        email: String,
        password: String,
    ) -> Result<User, Error> {
        let user = sqlx::query_as::<_, User>(
            // language=PostgresQL
            r#"SELECT * FROM app.register_user($1, $2, $3, $4);"#,
        )
        .bind(first_name)
        .bind(last_name)
        .bind(email)
        .bind(password)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }
}
