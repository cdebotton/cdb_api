#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]

use std::{env, process::exit};

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

    let database_url = match dotenvy::var("DATABASE_URL") {
        Ok(str) => str,
        Err(err) => {
            tracing::error!("Failed to read DATABASE_URL from env with error {err:#?}");
            exit(2);
        }
    };

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Unable to run migrations, cannot start application");

    let config = Config::parse();

    http::serve(pool, config).await?;

    Ok(())
}
