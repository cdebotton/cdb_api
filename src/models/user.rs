use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
enum UserError {
    #[error("Not Found")]
    NotFound,
    #[error("Server error {0:#?}")]
    SqlxError(#[from] sqlx::Error),
}

#[derive(FromRow, Default, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub last_login: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl User {
    async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Self, UserError> {
        sqlx::query_as!(User, r#"select * from app_public."user" where id = $1"#, id)
            .fetch_optional(pool)
            .await?
            .ok_or(UserError::NotFound)
    }

    pub fn find_all() {}

    pub fn create() {}

    pub fn destroy() {}

    pub fn update() {}
}
