use std::env;

use jwt::Keys;
use once_cell::sync::Lazy;

mod error;
pub mod jwt;
mod models;
pub mod opts;
pub mod routes;
pub mod utils;

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| panic!("JWT_SECRET not set"));
    Keys::new(secret.as_bytes())
});
