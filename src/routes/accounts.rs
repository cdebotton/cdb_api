use crate::{
    jwt::{AuthError, Claims},
    models::{auth::Auth, user::User},
    KEYS,
};
use axum::{routing::post, Extension, Json, Router};
use chrono::{DateTime, Utc};
use jsonwebtoken::{encode, Algorithm, Header};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

pub fn routes() -> Router {
    Router::new()
        .route("/authorize", post(authorize))
        .route("/register", post(register))
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
) -> Result<Json<AuthBody>, AuthError> {
    payload
        .validate()
        .map_err(|_| AuthError::MissingCredentials)?;

    let (role, user_id, refresh_token, refresh_token_expires) =
        Auth::authenticate(&pool, &payload.client_id, &payload.client_secret).await?;

    let claims = Claims::new(user_id, role.into());

    let header = Header::new(Algorithm::HS512);
    let token = encode(&header, &claims, &KEYS.encoding).map_err(|_| AuthError::TokenCreation)?;

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
) -> Result<Json<User>, AuthError> {
    request.validate()?;

    let user = Auth::register(
        &pool,
        request.first_name,
        request.last_name,
        request.email,
        request.password,
    )
    .await?;

    Ok(Json(user))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::routes::utils::test::{response_json, RequestBuilderExt};
    use axum::http::Request;
    use eyre::Result;
    use serde_json::json;
    use std::borrow::BorrowMut;
    use tower::ServiceExt;

    #[sqlx::test(fixtures("users"))]
    async fn test_authorize(pool: PgPool) -> Result<()> {
        let mut app = Router::new().merge(routes()).layer(Extension(pool));

        let request = Request::post("/authorize").json(json! {{
            "client_id": "sleepy.g@yahoo.com",
            "client_secret": "test"
        }});

        let mut res = app.borrow_mut().oneshot(request).await?;
        let json = response_json(&mut res).await;

        assert_eq!(json["token_type"], "Bearer".to_string());

        Ok(())
    }

    #[sqlx::test]
    async fn test_register(pool: PgPool) -> Result<()> {
        let mut app = Router::new().merge(routes()).layer(Extension(pool));

        let request = Request::post("/register").json(json! {{
            "first_name": "Sleepy",
            "last_name": "Gary",
            "email": "sleepy.g@yahoo.com",
            "password": "thisIsMyPassword"
        }});

        let mut res = app.borrow_mut().oneshot(request).await?;
        let json = response_json(&mut res).await;
        println!("{json}");

        assert_eq!(json["first_name"], "Sleepy".to_string());
        assert_eq!(json["last_name"], "Gary".to_string());

        Ok(())
    }
}
