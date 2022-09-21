#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]

use std::env;

use cdb_api::{
    config::Config,
    http::{self, error::Error},
};
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "cdb_api=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    std::panic::set_hook(Box::new(|panic| {
        panic.location().map_or_else(
            || {
                tracing::error!(message = %panic);
            },
            |location| {
                tracing::error!(
                    message = %panic,
                    panic.file = location.file(),
                    panic.line = location.line(),
                    panic.column = location.column()
                );
            },
        );
    }));

    let config = Config::parse();

    let database_url = dotenvy::var("DATABASE_URL")
        .ok()
        .unwrap_or_else(|| panic!("Unable to read env DATABASE_URL"));

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .ok()
        .unwrap_or_else(|| panic!("Unable to connect to the database"));

    if let Err(err) = sqlx::migrate!().run(&pool).await {
        tracing::error!("Unable to run database migrations: {}", err);
    }

    if let Err(err) = http::serve(pool, config).await {
        tracing::error!("Unable to start server: {}", err);
    }
}
