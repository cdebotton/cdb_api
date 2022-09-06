mod accounts;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Extension, Json, Router};
use sqlx::PgPool;

pub fn app(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/accounts", accounts::routes())
        .fallback(get(not_found))
        .layer(Extension(pool))
}

async fn root() -> Json<&'static str> {
    Json("OK!")
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not found")
}

#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;

    use axum::http::Request;
    use eyre::Result;
    use tower::ServiceExt;

    use super::*;

    use crate::utils::test::RequestBuilderExt;

    #[sqlx::test]
    async fn test_root(pool: PgPool) -> Result<()> {
        let mut app = app(pool);
        let request = Request::get("/").empty_body();
        let res = app.borrow_mut().oneshot(request).await?;

        assert_eq!(res.status(), StatusCode::OK);

        Ok(())
    }
}
