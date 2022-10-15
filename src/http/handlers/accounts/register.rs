use axum::{Extension, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::Error;

#[derive(FromRow, Serialize, Debug, Default, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterResponse {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterBody {
    #[schema(example = "Mark")]
    pub first_name: Option<String>,
    #[schema(example = "Ruffalo")]
    pub last_name: Option<String>,
    #[schema(example = "bark.ruffalo@gmail.com")]
    #[validate(email)]
    pub email: String,
    #[schema(example = "Sm4rT.HuLk")]
    #[validate(length(min = 8))]
    pub password: String,
}

#[utoipa::path(
    post,
    path = "/accounts/register",
    request_body = RegisterBody,
    responses(
        (status = 200, description = "Registration successful", body = RegisterResponse),
        (status = 401, description = "Invalid refresh token", body = Error),
        (status = 500, description = "Internal error", body = Error)
    )
)]
pub async fn register(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<RegisterBody>,
) -> Result<Json<RegisterResponse>, Error> {
    payload.validate()?;

    let register_response = sqlx::query_as::<_, RegisterResponse>(
        // language=PostgresQL
        r#"
            SELECT id, first_name, last_name, created_at, updated_at
            FROM app.register_user($1, $2, $3, $4);
        "#,
    )
    .bind(payload.first_name)
    .bind(payload.last_name)
    .bind(payload.email)
    .bind(payload.password)
    .fetch_one(&pool)
    .await?;

    Ok(Json(register_response))
}
