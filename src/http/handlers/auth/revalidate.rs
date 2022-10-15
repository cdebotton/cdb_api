use axum::{Extension, Json};
use jsonwebtoken::{encode, Algorithm, Header};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use utoipa::ToSchema;

use crate::{http::jwt::Claims, Error, KEYS};

#[derive(Debug, Serialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct RevalidateResponse {
    pub token_type: &'static str,
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub refresh_token_expires: i64,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RevalidateBody {
    refresh_token: uuid::Uuid,
}

#[utoipa::path(
    post,
    path = "/auth/revalidate",
    request_body = RevalidateBody,
    responses(
        (status = 200, description = "Revalidation successful", body = RevalidateResponse),
        (status = 401, description = "Invalid refresh token", body = Error),
        (status = 500, description = "Internal server error", body = Error),
    )
)]
pub async fn revalidate(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<RevalidateBody>,
) -> Result<Json<RevalidateResponse>, Error> {
    let row = sqlx::query!(
        // language=PostgresQL
        r#"SELECT
            role "role!",
            user_id "user_id!",
            refresh_token "refresh_token!",
            refresh_token_expires "refresh_token_expires!"
        FROM app.validate_refresh_token($1)"#,
        payload.refresh_token
    )
    .fetch_one(&pool)
    .await?;

    let claims = Claims::new(row.user_id, row.role.into());

    let header = Header::new(Algorithm::HS512);
    let access_token = encode(&header, &claims, &KEYS.encoding)?;

    tracing::info!("Revalidated token for user with id `{}`", row.user_id);

    Ok(Json(RevalidateResponse {
        token_type: "Bearer",
        access_token,
        expires_in: claims.exp,
        refresh_token: row.refresh_token.to_string(),
        refresh_token_expires: row.refresh_token_expires.timestamp_millis(),
    }))
}
