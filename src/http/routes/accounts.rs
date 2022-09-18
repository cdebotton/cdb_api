use axum::{routing::post, Extension, Json, Router};
use chrono::{DateTime, Utc};
use jsonwebtoken::{encode, Algorithm, Header};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;
use validator::Validate;

use crate::{
    http::{error::Error, jwt::Claims},
    KEYS,
};

pub fn routes() -> Router {
    Router::new()
        .route("/authorize", post(authorize))
        .route("/register", post(register))
        .route("/revalidate", post(revalidate))
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuthPayload {
    #[validate(email, length(min = 1))]
    pub client_id: String,
    #[validate(length(min = 1))]
    pub client_secret: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthBody {
    pub token_type: &'static str,
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub refresh_token_expires: i64,
}

async fn authorize(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<AuthBody>, Error> {
    payload.validate().map_err(|_| Error::MissingCredentials)?;

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
    .await
    .map_err(|_| Error::WrongCredentials)?;

    let claims = Claims::new(row.user_id, row.role.into());

    let header = Header::new(Algorithm::HS512);
    let access_token =
        encode(&header, &claims, &KEYS.encoding).map_err(|_| Error::TokenCreation)?;

    Ok(Json(AuthBody {
        token_type: "Bearer",
        access_token,
        expires_in: claims.exp,
        refresh_token: row.refresh_token.to_string(),
        refresh_token_expires: row.refresh_token_expires.timestamp_millis(),
    }))
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RegisterPayload {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(FromRow, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct RegisterResponse {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

async fn register(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<RegisterPayload>,
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

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
struct RevalidatePayload {
    refresh_token: uuid::Uuid,
}

async fn revalidate(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<RevalidatePayload>,
) -> Result<Json<AuthBody>, Error> {
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
    .await
    .map_err(|_| Error::WrongCredentials)?;

    let claims = Claims::new(row.user_id, row.role.into());

    let header = Header::new(Algorithm::HS512);
    let access_token =
        encode(&header, &claims, &KEYS.encoding).map_err(|_| Error::TokenCreation)?;

    tracing::info!("Revalidated token for user with id `{}`", row.user_id);

    Ok(Json(AuthBody {
        token_type: "Bearer",
        access_token,
        expires_in: claims.exp,
        refresh_token: row.refresh_token.to_string(),
        refresh_token_expires: row.refresh_token_expires.timestamp_millis(),
    }))
}
