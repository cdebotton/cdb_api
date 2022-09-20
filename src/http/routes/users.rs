use axum::{extract::Path, routing::get, Extension, Json, Router};
use chrono::{
    serde::{ts_milliseconds, ts_milliseconds_option},
    DateTime, Utc,
};
use serde::Serialize;
use sqlx::{postgres::PgRow, FromRow, PgPool, Row};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::http::error::Error;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_users))
        .route("/:id", get(get_user))
}

#[derive(Serialize, Debug, Clone, Default, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    #[schema(example = "major.tom@gmail.com")]
    pub email: String,
}

#[derive(Default, Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UsersResponse {
    #[schema(example = "Uuid::default()")]
    pub id: Uuid,
    #[schema(example = "David")]
    pub first_name: Option<String>,
    #[schema(example = "Bowie")]
    pub last_name: Option<String>,
    #[serde(with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_milliseconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
    pub account: Account,
}

impl<'r> FromRow<'r, PgRow> for UsersResponse {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let account = Account {
            email: row.get("email"),
        };

        let users = Self {
            id: row.get("id"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            account,
        };

        Ok(users)
    }
}

#[utoipa::path(get, path = "/users", responses(
    (status = 200, description = "List all users", body = [UsersResponse])
))]
pub async fn get_users(
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
    .await?;

    Ok(Json(users))
}

#[derive(Default, Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub account: Account,
}

impl<'r> FromRow<'r, PgRow> for UserResponse {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let account = Account {
            email: row.get("email"),
        };

        let user = Self {
            id: row.get("id"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            account,
        };

        Ok(user)
    }
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    responses(
        (status = 200, description = "Get a user", body = UserResponse),
        (status = 404, description = "User not found")
    ),
    params(
        ("id" = Uuid, Path, description = "The user's id")
    )
)]
pub async fn get_user(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, Error> {
    let user = sqlx::query_as::<_, UserResponse>(
        // language=PostgreSQL
        r#"
            SELECT u.id, u.first_name, u.last_name, a.email
            FROM app.users AS u
            LEFT JOIN app_private.accounts AS a
            ON a.user_id = u.id
            WHERE id = $1
        "#,
    )
    .bind(&id)
    .fetch_optional(&pool)
    .await?
    .ok_or(Error::NotFound)?;

    Ok(Json(user))
}
