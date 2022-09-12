use axum::{routing::post, Extension, Json, Router};
use chrono::{DateTime, Utc};
use jsonwebtoken::{encode, Algorithm, Header};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{error::Error, jwt::Claims, models::user::User, AuthService, KEYS};

pub fn routes() -> Router {
    Router::new()
        .route("/authorize", post(authorize))
        .route("/register", post(register))
        .route("/revalidate", post(revalidate))
}

#[derive(Debug, Deserialize, Validate)]
pub struct AuthPayload {
    #[validate(email, length(min = 1))]
    pub client_id: String,
    #[validate(length(min = 1))]
    pub client_secret: String,
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    pub token_type: String,
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub refresh_token_expires: i64,
}

impl AuthBody {
    pub fn new(
        access_token: String,
        expires_in: i64,
        refresh_token: Uuid,
        refresh_token_expires: DateTime<Utc>,
    ) -> Self {
        Self {
            token_type: "Bearer".to_string(),
            access_token,
            expires_in,
            refresh_token: refresh_token.to_string(),
            refresh_token_expires: refresh_token_expires.timestamp_millis(),
        }
    }
}

async fn authorize(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<AuthBody>, Error> {
    payload.validate().map_err(|_| Error::MissingCredentials)?;

    let (role, user_id, refresh_token, refresh_token_expires) =
        AuthService::authenticate(&pool, &payload.client_id, &payload.client_secret).await?;

    let claims = Claims::new(user_id, role.into());

    let header = Header::new(Algorithm::HS512);
    let token = encode(&header, &claims, &KEYS.encoding).map_err(|_| Error::TokenCreation)?;

    Ok(Json(AuthBody::new(
        token,
        claims.exp,
        refresh_token,
        refresh_token_expires,
    )))
}

#[derive(Debug, Deserialize)]
struct RegisterBody;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterPayload {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

async fn register(
    Extension(pool): Extension<PgPool>,
    Json(request): Json<RegisterPayload>,
) -> Result<Json<User>, Error> {
    request.validate()?;

    let user = AuthService::register(
        &pool,
        request.first_name,
        request.last_name,
        request.email,
        request.password,
    )
    .await?;

    Ok(Json(user))
}

#[derive(Debug, Deserialize, Validate)]
struct RevalidatePayload {
    refresh_token: uuid::Uuid,
}

async fn revalidate(
    Extension(pool): Extension<PgPool>,
    Json(body): Json<RevalidatePayload>,
) -> Result<Json<AuthBody>, Error> {
    let (role, user_id, refresh_token, refresh_token_expires) =
        AuthService::revalidate(&pool, body.refresh_token).await?;

    let claims = Claims::new(user_id, role.into());

    let header = Header::new(Algorithm::HS512);
    let token = encode(&header, &claims, &KEYS.encoding).map_err(|_| Error::TokenCreation)?;

    Ok(Json(AuthBody::new(
        token,
        claims.exp,
        refresh_token,
        refresh_token_expires,
    )))
}
