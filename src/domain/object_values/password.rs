use argon2::{
    Argon2, PasswordVerifier,
    password_hash::{PasswordHash, PasswordHasher, SaltString, rand_core::OsRng},
};
use thiserror::Error;

use crate::domain::models::user::UserError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Password(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum PasswordError {
    #[error("Password cannot be empty")]
    Empty,

    #[error("Password is too short (minimum {0} characters")]
    TooShort(usize),

    #[error("Password is too long (maximum {0} characters")]
    TooLong(usize),

    #[error("Password must contain at least one uppercase letter")]
    MissingUppercase,

    #[error("Password must contain at least one digit")]
    MissingDigit,

    #[error("Password must contain at least one special character")]
    MisssingSpecialChar,

    #[error("Failed to hash password: {0}")]
    HashingError(String),
}

impl Password {
    const MIN_LENGTH: usize = 8;
    const MAX_LENGTH: usize = 15;

    pub fn try_new(plain_text: String) -> Result<Self, PasswordError> {
        if plain_text.is_empty() {
            return Err(PasswordError::Empty);
        }

        if plain_text.len() < Self::MIN_LENGTH {
            return Err(PasswordError::TooShort(Self::MIN_LENGTH));
        }

        if plain_text.len() > Self::MAX_LENGTH {
            return Err(PasswordError::TooLong(Self::MAX_LENGTH));
        }

        if !plain_text.chars().any(|c| c.is_uppercase()) {
            return Err(PasswordError::MissingUppercase);
        }

        if !plain_text.chars().any(|c| c.is_numeric()) {
            return Err(PasswordError::MissingDigit);
        }

        if !plain_text.chars().any(|c| {
            c == '@'
                || c == '#'
                || c == '$'
                || c == '%'
                || c == '&'
                || c == '*'
                || c == '+'
                || c == '!'
        }) {
            return Err(PasswordError::MisssingSpecialChar);
        }

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(plain_text.as_bytes(), &salt)
            .map_err(|e| PasswordError::HashingError(e.to_string()))?
            .to_string();

        Ok(Password(password_hash))
    }

    pub fn from_hash(hash: String) -> Self {
        Self(hash)
    }

    pub fn verify(&self, attempt: &str) -> bool {
        let parsed_hash = PasswordHash::new(&self.0).expect("Invalid hash format in database");

        Argon2::default()
            .verify_password(attempt.as_bytes(), &parsed_hash)
            .is_ok()
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<PasswordError> for UserError {
    fn from(error: PasswordError) -> Self {
        UserError::InvalidData(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_new_valid_password() {
        let password = Password::try_new("ValidPass123!".to_string()).unwrap();
        assert_eq!(password.into_inner().len(), 97); // Argon2 hash length
    }

    #[test]
    fn test_try_new_empty_password() {
        let result = Password::try_new("".to_string());
        assert_eq!(result.unwrap_err(), PasswordError::Empty);
    }

    #[test]
    fn test_try_new_too_short_password() {
        let result = Password::try_new("Short1!".to_string());
        assert_eq!(result.unwrap_err(), PasswordError::TooShort(8));
    }

    #[test]
    fn test_try_new_too_long_password() {
        let result = Password::try_new("ThisIsAVeryLongPassword123!".to_string());
        assert_eq!(result.unwrap_err(), PasswordError::TooLong(15));
    }

    #[test]
    fn test_try_new_missing_uppercase() {
        let result = Password::try_new("lowercase1!".to_string());
        assert_eq!(result.unwrap_err(), PasswordError::MissingUppercase);
    }

    #[test]
    fn test_try_new_missing_digit() {
        let result = Password::try_new("Uppercase1!".to_string());
        assert_eq!(result.unwrap_err(), PasswordError::MissingDigit);
    }

    #[test]
    fn test_try_new_missing_special_char() {
        let result = Password::try_new("Uppercase1!".to_string());
        assert_eq!(result.unwrap_err(), PasswordError::MisssingSpecialChar);
    }
}
