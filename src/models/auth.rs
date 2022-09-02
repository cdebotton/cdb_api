use sqlx::PgPool;
use uuid::Uuid;

use crate::jwt::AuthError;

use super::user::User;

#[derive(Debug)]
pub struct Auth<'a> {
    pool: &'a PgPool,
    pub user: Option<User>,
}

impl<'a> Auth<'a> {
    pub const fn new(pool: &'a PgPool) -> Self {
        Auth { pool, user: None }
    }

    pub async fn authenticate(
        &mut self,
        email: &String,
        password: &String,
    ) -> Result<(String, Uuid), AuthError> {
        let call = sqlx::query!(
            // language=PostgreSQL
            r#"SELECT role, user_id FROM app.authenticate($1, $2)"#,
            &email,
            &password
        )
        .fetch_one(self.pool)
        .await?;

        match (call.role, call.user_id) {
            (Some(role), Some(user_id)) => Ok((role, user_id)),
            _ => Err(AuthError::WrongCredentials),
        }
    }
}
