use crate::errors::Error;
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};

pub fn hash(password: &str) -> Result<String, Error> {
    Ok(Argon2::default()
        .hash_password(password.as_bytes())
        .map_err(|_| Error::Internal)?
        .to_string())
}

pub fn verify(password: &str, hash: &str) -> Result<bool, Error> {
    let parsed = PasswordHash::new(hash).map_err(|_| Error::Internal)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}
