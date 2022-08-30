use std::fmt::{self, Display};

use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::KEYS;

use super::AuthError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub company: String,
    pub exp: usize,
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
        write!(f, "Email: {}\nCompany: {}", self.sub, self.company)
    }
}
