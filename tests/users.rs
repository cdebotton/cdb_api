use std::borrow::BorrowMut;

use axum::http::Request;
use cdb_api::{http::routes, test_utils::*};
use eyre::Result;
use sqlx::PgPool;
use tower::ServiceExt;

#[sqlx::test(fixtures("users"))]
async fn get_users(pool: PgPool) -> Result<()> {
    let mut app = routes(pool);
    let req = Request::get("/users").empty_body();
    let mut res = app.borrow_mut().oneshot(req).await?;
    let json = response_json(&mut res).await;

    let first_user = json.get(0).expect("Expecting a first user");
    let second_user = json.get(1).expect("Expecting a second user");

    assert_eq!(
        first_user
            .get("firstName")
            .expect("Expecting the first user to have a first name"),
        "Sleepy"
    );
    assert_eq!(
        first_user
            .get("lastName")
            .expect("Expecting the first user to have a last name"),
        "Gary"
    );
    assert_eq!(
        second_user
            .get("firstName")
            .expect("Expecting the second user to have a first name"),
        "Kiko"
    );
    assert_eq!(
        second_user
            .get("lastName")
            .expect("Expecting the second user to have a last name"),
        "Bato-de Botton"
    );

    Ok(())
}
