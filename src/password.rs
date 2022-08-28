use argon2::{
    password_hash::{self, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use eyre::Result;
use tokio::task;

pub async fn hash(password: String) -> Result<String> {
    task::spawn_blocking(move || {
        let salt = SaltString::generate(rand::thread_rng());
        let hashed_password = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| eyre::eyre!(e))?
            .to_string();

        Ok(hashed_password)
    })
    .await?
}

pub async fn verify(password: String, hash: String) -> Result<bool> {
    task::spawn_blocking(move || {
        let hash = PasswordHash::new(&hash).map_err(|e| eyre::eyre!(e))?;
        let res = Argon2::default().verify_password(password.as_bytes(), &hash);

        match res {
            Ok(()) => Ok(true),
            Err(password_hash::Error::Password) => Ok(false),
            Err(e) => Err(eyre::eyre!(e)),
        }
    })
    .await?
}
