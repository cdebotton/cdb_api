use crate::{
    jwt::{AuthError, Claims},
    models::auth::Auth,
    KEYS,
};
use axum::{routing::post, Extension, Json, Router};
use jsonwebtoken::{encode, Header};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use validator::Validate;

pub fn routes() -> Router {
    Router::new()
        .route("/accounts/authorize", post(authorize))
        .route("/accounts/register", post(register))
}

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    pub access_token: String,
    pub token_type: String,
}

impl AuthBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

async fn authorize(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<AuthBody>, AuthError> {
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    let (role, user_id) = Auth::new(&pool)
        .authenticate(&payload.client_id, &payload.client_secret)
        .await?;

    let claims = Claims::new(user_id, role.into());

    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(AuthBody::new(token)))
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
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<RegisterBody>, AuthError> {
    Ok(Json(RegisterBody))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::routes::{
        app,
        utils::test::{response_json, RequestBuilderExt},
    };
    use axum::http::Request;
    use eyre::Result;
    use serde_json::json;
    use std::borrow::BorrowMut;
    use tower::ServiceExt;

    #[sqlx::test(fixtures("users"))]
    async fn test_authorize(pool: PgPool) -> Result<()> {
        let mut app = app(pool);
        let request = Request::post("/authorize").json(json! {{
            "client_id": "sleepy.g@yahoo.com",
            "client_secret": "test"
        }});

        let mut res = app.borrow_mut().oneshot(request).await?;
        let json = response_json(&mut res).await;

        assert_eq!(json["token_type"], "Bearer".to_string());

        Ok(())
    }
}
