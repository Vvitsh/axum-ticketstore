use argon2::{
    Argon2, PasswordVerifier,
    password_hash::{PasswordHash, PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::http::StatusCode;

use super::api_error::ApiError;

pub fn argon_hash(password: String) -> Result<String, ApiError> {
    let pw_bytes = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(pw_bytes, &salt)
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Hashing error"))?;

    Ok(hash.to_string())
}

pub fn argon_verify(password: String, hash: &str) -> Result<bool, ApiError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|_err| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal error"));

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash.unwrap())
        .map_err(|_err| ApiError::new(StatusCode::UNAUTHORIZED, "Incorrect password"))?;

    Ok(true)
}
