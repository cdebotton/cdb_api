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
    let users = User::find_all(&pool).await?;

    Ok(Json(users))
}

async fn get_user(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, Error> {
    let user = User::find_by_id(&pool, id).await?;

    Ok(Json(user))
}
