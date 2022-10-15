use axum::http::Request;

use cdb_api::{http::routes, test_utils::*};
use eyre::Result;
use serde_json::json;
use sqlx::PgPool;
use std::borrow::BorrowMut;
use tower::ServiceExt;

#[sqlx::test]
async fn test_register(pool: PgPool) -> Result<()> {
    let mut app = routes(pool);

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
