use crate::{AuthError, AuthResult};
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PasswordHashString(String);

impl PasswordHashString {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub fn hash_password(password: &str) -> AuthResult<PasswordHashString> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| PasswordHashString(hash.to_string()))
        .map_err(|_| AuthError::PasswordHashFailed)
}

pub fn verify_password(password: &str, hash: &PasswordHashString) -> AuthResult<()> {
    let parsed = PasswordHash::new(hash.as_str()).map_err(|_| AuthError::MalformedPasswordHash)?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .map_err(|_| AuthError::PasswordRejected)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verifies_valid_password() {
        let hash = hash_password("secret").unwrap();
        assert!(verify_password("secret", &hash).is_ok());
    }

    #[test]
    fn rejects_wrong_password() {
        let hash = hash_password("secret").unwrap();
        assert_eq!(
            verify_password("wrong", &hash),
            Err(AuthError::PasswordRejected)
        );
    }

    #[test]
    fn salts_are_unique() {
        assert_ne!(
            hash_password("secret").unwrap(),
            hash_password("secret").unwrap()
        );
    }
}
