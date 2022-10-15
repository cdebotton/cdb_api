use crate::config::Config;
use axum::{
    routing::{get, post},
    Extension, Router, Server,
};
use error::Error;
use sqlx::PgPool;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

use self::handlers::{accounts, auth, get_openapi, users};

pub mod error;
pub mod handlers;
pub mod jwt;

pub async fn serve(pool: PgPool, config: Config) -> Result<(), Error> {
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));

    Server::bind(&addr)
        .serve(
            routes(pool)
                .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
                .into_make_service(),
        )
        .await?;

    Ok(())
}

pub fn routes(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(get_openapi))
        .route("/users", get(users::find_users))
        .route("/users/:id", get(users::find_user_by_id))
        .route("/accounts/register", post(accounts::register))
        .route("/auth/authorize", post(auth::authorize))
        .route("/auth/revalidate", post(auth::revalidate))
        .fallback(get(handlers::not_found))
        .layer(Extension(pool))
}
