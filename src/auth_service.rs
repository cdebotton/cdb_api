use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::Error, models::user::User};

#[derive(Debug)]
pub struct AuthService {
    pub user: Option<User>,
}

impl AuthService {
    pub async fn authenticate(
        pool: &PgPool,
        email: &String,
        password: &String,
    ) -> Result<(String, Uuid, Uuid, DateTime<Utc>), Error> {
        let call = sqlx::query!(
            // language=PostgreSQL
            r#"SELECT role, user_id, refresh_token, refresh_token_expires FROM app.authenticate($1, $2)"#,
            &email,
            &password
        )
        .fetch_one(pool)
        .await?;

        match (
            call.role,
            call.user_id,
            call.refresh_token,
            call.refresh_token_expires,
        ) {
            (Some(role), Some(user_id), Some(refresh_token), Some(refresh_token_expires)) => {
                Ok((role, user_id, refresh_token, refresh_token_expires))
            }
            _ => Err(Error::WrongCredentials),
        }
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
