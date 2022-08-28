use crate::{
    jwt::{AuthBody, AuthError, AuthPayload, Claims},
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

pub fn app(db: PgPool) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/authorize", post(authorize))
        .fallback(get(not_found))
        .layer(Extension(db))
}

async fn root(Extension(db): Extension<PgPool>) -> Json<&'static str> {
    println!("{db:#?}");
    Json("OK!")
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not found")
}

async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    if payload.client_id != "foo" || payload.client_secret != "bar" {
        return Err(AuthError::WrongCredentials);
    }

    let claims = Claims {
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        exp: 2000000,
    };

    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(AuthBody::new(token)))
}
