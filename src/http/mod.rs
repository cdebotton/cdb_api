use crate::config::Config;
use axum::Server;
use error::Error;
use sqlx::PgPool;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

pub mod error;
pub mod jwt;
pub mod routes;

pub async fn serve(pool: PgPool, config: Config) -> Result<(), Error> {
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));

    Server::bind(&addr)
        .serve(
            routes::app(pool)
                .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
                .into_make_service(),
        )
        .await?;

    Ok(())
}
