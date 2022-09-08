use chrono::{
    serde::{ts_milliseconds, ts_milliseconds_option},
    DateTime, Utc,
};
use serde::Serialize;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
enum UserError {
    #[error("Not Found")]
    NotFound,
    #[error("Server error {0:#?}")]
    SqlxError(#[from] sqlx::Error),
}

#[derive(FromRow, Default, Debug, Clone, Serialize)]
pub struct User {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[serde(with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_milliseconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl User {
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Self, UserError> {
        sqlx::query_as!(User, r#"select * from app.users where id = $1"#, id)
            .fetch_optional(pool)
            .await?
            .ok_or(UserError::NotFound)
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        Ok(sqlx::query_as!(User, r#"SELECT * FROM app.users"#)
            .fetch_all(pool)
            .await?)
    }

    pub fn create() {}

    pub fn destroy() {}

    pub fn update() {}
}
