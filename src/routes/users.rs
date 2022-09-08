use axum::{routing::get, Extension, Json, Router};
use sqlx::PgPool;

use crate::{error::Error, models::user::User};

pub fn routes() -> Router {
    Router::new().route("/", get(get_users))
}

async fn get_users(Extension(pool): Extension<PgPool>) -> Result<Json<Vec<User>>, Error> {
    let users = User::find_all(&pool).await?;

    Ok(Json(users))
}
