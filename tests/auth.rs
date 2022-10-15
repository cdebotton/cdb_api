use axum::http::Request;

use cdb_api::{http::routes, test_utils::*};
use eyre::Result;
use serde_json::json;
use sqlx::PgPool;
use std::borrow::BorrowMut;
use tower::ServiceExt;

#[sqlx::test(fixtures("users"))]
async fn test_authorize(pool: PgPool) -> Result<()> {
    let mut app = routes(pool);

    let request = Request::post("/auth/authorize").json(json! {{
        "clientId": "sleepy.g@yahoo.com",
        "clientSecret": "test"
    }});

    let mut res = app.borrow_mut().oneshot(request).await?;
    let json = response_json(&mut res).await;

    assert_eq!(json["tokenType"], "Bearer".to_string());

    Ok(())
}

#[sqlx::test(fixtures("revalidate"))]
async fn test_revalidate(pool: PgPool) -> Result<()> {
    let mut app = routes(pool.clone());

    let token = sqlx::query!(
        // language=PostgresQL
        r#"SELECT refresh_token FROM app_private.accounts WHERE email = 'not.the.clams@gmail.com';"#
    )
    .fetch_one(&pool)
    .await?;

    let token = token.refresh_token.unwrap();

    let request =
        Request::post("/auth/revalidate").json(json! {{ "refreshToken": token.to_string() }});
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
