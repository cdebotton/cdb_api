use std::{
    fmt::{self, Display},
    ops::Add,
};

use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::KEYS;

use super::AuthError;

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    Admin,
    Anonymous,
}

impl From<String> for Role {
    fn from(role: String) -> Self {
        match role.as_str() {
            "admin" => Self::Admin,
            "anonymous" => Self::Anonymous,
            _ => {
                tracing::error!("Invalid role {role:?}");
                Self::Anonymous
            }
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub role: Role,
    pub exp: i64,
}

impl Claims {
    pub fn new(sub: Uuid, role: Role) -> Self {
        let exp = Utc::now().add(Duration::minutes(15)).timestamp_millis();

        Claims { sub, role, exp }
    }
}

#[async_trait]
impl<B> FromRequest<B> for Claims
where
    B: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request(req: &'_ mut RequestParts<B>) -> Result<Self, Self::Rejection>
    where
        Self: Send + Sync,
    {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| AuthError::InvalidToken)?;

        let token_data = decode(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

impl Display for Claims {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "sub: {}\nrole: {}\nexp:{}",
            self.sub, self.role, self.exp
        )
    }
}
