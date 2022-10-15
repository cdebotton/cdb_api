use axum::{Extension, Json};
use chrono::{
    serde::{ts_milliseconds, ts_milliseconds_option},
    DateTime, Utc,
};
use serde::Serialize;
use sqlx::{FromRow, PgPool};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::Error;

#[derive(Default, Debug, Clone, Serialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UsersResponse {
    #[schema(example = "Uuid::default()")]
    pub id: Uuid,
    #[schema(example = "David")]
    pub first_name: Option<String>,
    #[schema(example = "Bowie")]
    pub last_name: Option<String>,
    #[schema(example = "1665856394804")]
    #[serde(with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    #[schema(example = "1664905980000")]
    #[serde(with = "ts_milliseconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
    #[schema(example = "major.tom@gmail.com")]
    pub email: String,
}

#[utoipa::path(get, path = "/users", responses(
  (status = 200, description = "List all users", body = [UsersResponse]),
  (status = 500, description = "Internal error", body = Error)
))]
pub async fn find_users(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<UsersResponse>>, Error> {
    let users = sqlx::query_as::<_, UsersResponse>(
        // language=PostgreSQL
        r#"
          SELECT u.*, a.email
          FROM app.users AS u
          LEFT JOIN app_private.accounts AS a
          ON a.user_id = u.id

      "#,
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| Error::InvalidToken)?;

    Ok(Json(users))
}
