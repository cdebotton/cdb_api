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
async fn main() -> Result<(), Error> {
    tracing_subscriber::registry()
        .with(EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "cdb_api=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    std::panic::set_hook(Box::new(|panic| {
        if let Some(location) = panic.location() {
            tracing::error!(
                message = %panic,
                panic.file = location.file(),
                panic.line = location.line(),
                panic.column = location.column()
            );
        } else {
            tracing::error!(message = %panic);
        }
    }));

    let database_url = dotenvy::var("DATABASE_URL")
        .expect("Failed to read DATABASE_URL from env with error {0:#?}");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Unable to connect to database with error {0:#?}");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Unable to run migrations, cannot start application");

    let config = Config::parse();

    http::serve(pool, config).await?;

    Ok(())
}
