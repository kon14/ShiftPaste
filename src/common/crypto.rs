use bcrypt::{hash, verify, DEFAULT_COST};

use super::error::AppError;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, DEFAULT_COST)
        .map_err(|err| AppError::internal_with_private("Failed to hash password!", err.to_string()))
}

pub fn verify_password(password: &str, hashed: &str) -> Result<bool, AppError> {
    verify(password, hashed).map_err(|err| {
        AppError::internal_with_private("Failed to verify password!", err.to_string())
    })
}
