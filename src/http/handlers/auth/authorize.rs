use crate::{http::jwt::Claims, Error, KEYS};
use axum::{Extension, Json};
use jsonwebtoken::{encode, Algorithm, Header};
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;
use sqlx::PgPool;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    pub token_type: &'static str,
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub refresh_token_expires: i64,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthBody {
    #[schema(example = "david.bowie@gmail.com")]
    #[validate(email, length(min = 1))]
    pub client_id: String,
    #[schema(example = "Z1gGy.Pl4y3d!GuI74R")]
    #[validate(length(min = 1))]
    pub client_secret: String,
}

#[utoipa::path(
    post,
    path = "/auth/authorize",
    request_body = AuthBody,
    responses(
        (status = 200, description = "Authorization successful", body = AuthResponse),
        (status = 400, description = "Validation error", body = Error),
        (status = 401, description = "Invalid credentials", body = Error),
        (status = 500, description = "Internal server error", body = Error),
    ),

)]
pub async fn authorize(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<AuthBody>,
) -> Result<Json<AuthResponse>, Error> {
    payload.validate()?;

    let row = sqlx::query!(
        // language=PostgreSQL
        r#"SELECT
                role "role!",
                user_id "user_id!",
                refresh_token "refresh_token!",
                refresh_token_expires "refresh_token_expires!"
            FROM app.authenticate($1, $2)"#,
        &payload.client_id,
        &payload.client_secret
    )
    .fetch_one(&pool)
    .await?;

    let claims = Claims::new(row.user_id, row.role.into());

    let header = Header::new(Algorithm::HS512);
    let access_token = encode(&header, &claims, &KEYS.encoding)?;

    Ok(Json(AuthResponse {
        token_type: "Bearer",
        access_token,
        expires_in: claims.exp,
        refresh_token: row.refresh_token.to_string(),
        refresh_token_expires: row.refresh_token_expires.timestamp_millis(),
    }))
}
