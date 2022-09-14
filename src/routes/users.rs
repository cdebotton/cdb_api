use axum::{extract::Path, routing::get, Extension, Json, Router};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::Error, models::user::User};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_users))
        .route("/:id", get(get_user))
}

async fn get_users(Extension(pool): Extension<PgPool>) -> Result<Json<Vec<User>>, Error> {
    let users = sqlx::query_as!(User, r#"SELECT * FROM app.users"#)
        .fetch_all(&pool)
        .await?;

    Ok(Json(users))
}

async fn get_user(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, Error> {
    let user = sqlx::query_as!(User, r#"select * from app.users where id = $1"#, id)
        .fetch_optional(&pool)
        .await?
        .ok_or(Error::NotFound)?;

    Ok(Json(user))
}
