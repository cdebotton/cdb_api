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

pub fn app(db: PgPool) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/authorize", post(authorize))
        .fallback(get(not_found))
        .layer(Extension(db))
}

async fn root(Extension(db): Extension<PgPool>) -> Json<&'static str> {
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
        .authenticate(&payload.client_id, &payload.client_id)
        .await?;

    let claims = Claims {
        sub: user_id.to_owned(),
        company: role.to_owned(),
        exp: 1000 * 60 * 15,
    };

    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(AuthBody::new(token)))
}
