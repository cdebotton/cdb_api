pub mod utils;

use crate::{
    jwt::{AuthBody, AuthError, AuthPayload, Claims},
    models::auth::Auth,
    KEYS,
};
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use jsonwebtoken::{encode, Header};
use sqlx::PgPool;

pub fn app(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/authorize", post(authorize))
        .fallback(get(not_found))
        .layer(Extension(pool))
}

async fn root() -> Json<&'static str> {
    Json("OK!")
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not found")
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

#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;

    use axum::http::Request;
    use eyre::Result;
    use serde_json::json;
    use tower::ServiceExt;

    use super::*;

    use crate::routes::utils::test::{response_json, RequestBuilderExt};

    #[sqlx::test]
    async fn test_root(pool: PgPool) -> Result<()> {
        let mut app = app(pool);
        let request = Request::get("/").empty_body();
        let res = app.borrow_mut().oneshot(request).await?;

        assert_eq!(res.status(), StatusCode::OK);

        Ok(())
    }

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
