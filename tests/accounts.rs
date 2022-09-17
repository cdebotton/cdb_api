use axum::http::Request;

use cdb_api::{http::routes::app, test_utils::*};
use eyre::Result;
use serde_json::json;
use sqlx::PgPool;
use std::borrow::BorrowMut;
use tower::ServiceExt;

#[sqlx::test(fixtures("users"))]
async fn test_authorize(pool: PgPool) -> Result<()> {
    let mut app = app(pool);

    let request = Request::post("/accounts/authorize").json(json! {{
        "clientId": "sleepy.g@yahoo.com",
        "clientSecret": "test"
    }});

    let mut res = app.borrow_mut().oneshot(request).await?;
    let json = response_json(&mut res).await;

    assert_eq!(json["tokenType"], "Bearer".to_string());

    Ok(())
}

#[sqlx::test]
async fn test_register(pool: PgPool) -> Result<()> {
    let mut app = app(pool);

    let request = Request::post("/accounts/register").json(json! {{
        "firstName": "Sleepy",
        "lastName": "Gary",
        "email": "sleepy.g@yahoo.com",
        "password": "thisIsMyPassword"
    }});

    let mut res = app.borrow_mut().oneshot(request).await?;
    let json = response_json(&mut res).await;

    json.get("createdAt").expect("Expecting created timestamp");

    assert_eq!(
        json.get("updatedAt").expect("Expecting updated timestamp"),
        &serde_json::json!(null)
    );

    json.get("id").expect("Expecting ID");

    assert_eq!(
        json.get("firstName").expect("Expecting first name"),
        "Sleepy"
    );
    assert_eq!(json.get("lastName").expect("Expecting last name"), "Gary");

    Ok(())
}

#[sqlx::test(fixtures("revalidate"))]
async fn test_revalidate(pool: PgPool) -> Result<()> {
    let mut app = app(pool.clone());

    let token = sqlx::query!(
        // language=PostgresQL
        r#"SELECT refresh_token FROM app_private.accounts WHERE email = 'not.the.clams@gmail.com';"#
    )
    .fetch_one(&pool)
    .await?;

    let token = token.refresh_token.unwrap();

    let request =
        Request::post("/accounts/revalidate").json(json! {{ "refreshToken": token.to_string() }});
    let mut response = app.borrow_mut().oneshot(request).await?;
    let json = response_json(&mut response).await;

    json.get("accessToken").expect("Expecting access token");
    json.get("expiresIn")
        .expect("Expecting expiration timestamp");
    json.get("refreshToken")
        .expect("Expecting expiration timestamp for refresh token");

    assert_eq!(
        json.get("tokenType").expect("Expecting token type"),
        "Bearer"
    );

    Ok(())
}
