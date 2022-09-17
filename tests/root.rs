use std::borrow::BorrowMut;

use axum::http::{Request, StatusCode};
use cdb_api::{http::routes::app, test_utils::*};
use eyre::Result;
use sqlx::PgPool;
use tower::ServiceExt;

#[sqlx::test]
async fn test_root(pool: PgPool) -> Result<()> {
    let mut app = app(pool);
    let request = Request::get("/").empty_body();
    let res = app.borrow_mut().oneshot(request).await?;

    assert_eq!(res.status(), StatusCode::OK);

    Ok(())
}
