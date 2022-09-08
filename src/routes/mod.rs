mod accounts;
mod users;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Extension, Json, Router};
use sqlx::PgPool;

pub fn app(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/accounts", accounts::routes())
        .nest("/users", users::routes())
        .fallback(get(not_found))
        .layer(Extension(pool))
}

async fn root() -> Json<&'static str> {
    Json("OK!")
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not found")
}
