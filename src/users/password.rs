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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_and_verify_correct_password() {
        let hash = hash("my-password").unwrap();
        assert!(verify("my-password", &hash).unwrap());
    }

    #[test]
    fn verify_rejects_wrong_password() {
        let hash = hash("my-password").unwrap();
        assert!(!verify("not-my-password", &hash).unwrap());
    }

    #[test]
    fn same_password_produces_different_hashes() {
        let first_hash = hash("my-password").unwrap();
        let second_hash = hash("my-password").unwrap();
        assert_ne!(first_hash, second_hash);
    }
}
