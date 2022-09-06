use axum::http::Request;

use cdb_api::{
    routes::app,
    utils::test::{response_json, RequestBuilderExt},
};
use eyre::Result;
use serde_json::json;
use sqlx::PgPool;
use std::borrow::BorrowMut;
use tower::ServiceExt;

#[sqlx::test(fixtures("users"))]
async fn test_authorize(pool: PgPool) -> Result<()> {
    let mut app = app(pool);

    let request = Request::post("/accounts/authorize").json(json! {{
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
    let mut app = app(pool);

    let request = Request::post("/accounts/register").json(json! {{
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
