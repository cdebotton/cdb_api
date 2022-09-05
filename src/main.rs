#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]

mod error;
mod jwt;
mod models;
mod routes;

use std::{env, net::SocketAddr, process::exit};

use axum::Server;

use sqlx::postgres::PgPoolOptions;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use jwt::Keys;
use once_cell::sync::Lazy;

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| panic!("JWT_SECRET not set"));
    Keys::new(secret.as_bytes())
});

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "cdb_api=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let database_url = match dotenvy::var("DATABASE_URL") {
        Ok(str) => str,
        Err(err) => {
            tracing::error!("Failed to read DATABASE_URL from env with error {err:#?}");
            exit(2);
        }
    };

    match PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            if let Err(err) = sqlx::migrate!().run(&pool).await {
                tracing::error!("Migrations failed with err {err:?}");
            }

            match Server::bind(&addr)
                .serve(
                    routes::app(pool)
                        .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
                        .into_make_service(),
                )
                .await
            {
                Ok(_) => tracing::debug!("Listening on {}", addr),
                Err(err) => tracing::error!("Unable to start server, {}", err),
            }
        }
        Err(err) => panic!("Unable to connect to database, failed with error {:?}", err),
    }
}
