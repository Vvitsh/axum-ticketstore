use axum::http::StatusCode;
use chrono::{Duration, Utc};
use dotenvy_macro::dotenv;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use super::api_error::ApiError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    iat: usize,
    exp: usize,
}

pub fn create_jwt() -> Result<String, ApiError> {
    let mut now = Utc::now();
    let iat = now.timestamp() as usize;
    let expiry = Duration::hours(2);
    now += expiry;
    let exp = now.timestamp() as usize;

    let new_claim = Claims { iat, exp };

    let secret: &'static str = dotenv!("JWT_SECRET");
    let key = EncodingKey::from_secret(secret.as_bytes());

    encode(&Header::default(), &new_claim, &key)
        .map_err(|_err| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error generating token"))
}

pub fn is_valid(token: &str) -> Result<bool, ApiError> {
    let secret: &'static str = dotenv!("JWT_SECRET");
    let key = DecodingKey::from_secret(secret.as_bytes());

    decode::<Claims>(
        token,
        &key,
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )
    .map_err(|err| match err.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => ApiError::new(
            StatusCode::UNAUTHORIZED,
            "Your session has expired, please log in",
        ),
        _ => ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Token denied, please log in",
        ),
    })?;

    Ok(true)
}
