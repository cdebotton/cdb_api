use std::env;

use http::jwt::Keys;
use once_cell::sync::Lazy;

pub mod config;
pub mod http;
pub mod test_utils;

pub use http::error::Error;

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| panic!("JWT_SECRET not set"));
    Keys::new(secret.as_bytes())
});
