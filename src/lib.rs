use std::env;

pub use auth_service::AuthService;
use jwt::Keys;
use once_cell::sync::Lazy;

mod auth_service;
mod error;
pub mod jwt;
mod models;
pub mod routes;
pub mod utils;

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| panic!("JWT_SECRET not set"));
    Keys::new(secret.as_bytes())
});
