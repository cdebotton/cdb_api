mod accounts;
mod users;

use accounts::{AuthBody, AuthPayload, RevalidatePayload};
use axum::{http::StatusCode, response::IntoResponse, routing::get, Extension, Router};
use sqlx::PgPool;
use users::{Account, UserResponse, UsersResponse};
use utoipa::OpenApi;

pub fn app(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/accounts", accounts::routes())
        .nest("/users", users::routes())
        .fallback(get(not_found))
        .layer(Extension(pool))
}

#[derive(OpenApi)]
#[openapi(
    paths(
        users::get_users,
        users::get_user,
        accounts::authorize,
        accounts::revalidate
    ),
    components(schemas(
        UsersResponse,
        UserResponse,
        Account,
        AuthPayload,
        AuthBody,
        RevalidatePayload
    ))
)]
struct ApiDoc;

async fn root() -> String {
    ApiDoc::openapi().to_json().unwrap()
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not found")
}
