use axum::{extract::Path, Extension, Json};
use serde::Serialize;
use sqlx::{FromRow, PgPool};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::Error;

#[derive(Default, Debug, Clone, Serialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    #[schema(example = "a00c9bc7-92ca-413a-97ec-66204314bbca")]
    pub id: Uuid,
    #[schema(example = "David")]
    pub first_name: Option<String>,
    #[schema(example = "Bowie")]
    pub last_name: Option<String>,
    #[schema(example = "major.tom@gmail.com")]
    pub email: String,
}

#[utoipa::path(
  get,
  path = "/users/{id}",
  responses(
      (status = 200, description = "Get a user", body = UserResponse),
      (status = 404, description = "User not found", body = Error),
      (status = 500, description = "Internal Error", body = Error)
  ),
  params(
      ("id" = Uuid, Path, description = "The user's id")
  )
)]
pub async fn find_user_by_id(
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
    .await
    .map_err(|_| Error::InvalidToken)?
    .ok_or(Error::NotFound)?;

    Ok(Json(user))
}
