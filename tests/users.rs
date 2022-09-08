use std::borrow::BorrowMut;

use axum::http::Request;
use cdb_api::{
    routes::app,
    utils::test::{response_json, RequestBuilderExt},
};
use eyre::Result;
use sqlx::PgPool;
use tower::ServiceExt;

#[sqlx::test(fixtures("users"))]
async fn get_users(pool: PgPool) -> Result<()> {
    let mut app = app(pool);
    let req = Request::get("/users").empty_body();
    let mut res = app.borrow_mut().oneshot(req).await?;
    let json = response_json(&mut res).await;

    assert_eq!(json[0]["first_name"], "Sleepy");
    assert_eq!(json[0]["last_name"], "Gary");
    assert_eq!(json[1]["first_name"], "Kiko");
    assert_eq!(json[1]["last_name"], "Bato-de Botton");

    Ok(())
}
