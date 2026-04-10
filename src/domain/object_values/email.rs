use std::fmt::Display;

use thiserror::Error;

use crate::domain::{models::user::UserError, object_values::EMAIL_REGEX};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Email(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum EmailError {
    #[error("Email cannot be empty")]
    Emtpy,

    #[error("Email is too long (maximum {0} characters")]
    TooLong(usize),

    #[error("Invalid email format (e.g., user@example.com")]
    Invalid,
}

impl Email {
    const MAX_LENGTH: usize = 255;

    pub fn try_new(email: String) -> Result<Self, EmailError> {
        let trimmed = email.trim();

        if trimmed.is_empty() {
            return Err(EmailError::Emtpy);
        }

        if trimmed.len() > Self::MAX_LENGTH {
            return Err(EmailError::TooLong(Self::MAX_LENGTH));
        }

        if !Self::is_valid_email(trimmed) {
            return Err(EmailError::Invalid);
        }

        Ok(Self(trimmed.to_string()))
    }

    pub fn is_valid_email(email: &str) -> bool {
        EMAIL_REGEX.is_match(email)
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for Email {
    type Error = EmailError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl TryFrom<&str> for Email {
    type Error = EmailError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_new(value.to_string())
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<EmailError> for UserError {
    fn from(error: EmailError) -> Self {
        UserError::InvalidData(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_email_empty_should_return_empty_error() {
        let email = Email::try_new("".to_string());
        assert!(matches!(email, Err(EmailError::Emtpy)));
    }

    #[test]
    fn when_email_too_long_should_return_too_long_error() {
        let long_email = "a".repeat(256); // 256 characters
        let email = Email::try_new(long_email);
        assert!(matches!(email, Err(EmailError::TooLong(_))));
    }

    #[test]
    fn when_email_invalid_format_should_return_invalid_error() {
        let email = Email::try_new("invalid-email".to_string());
        assert!(matches!(email, Err(EmailError::Invalid)));
    }

    #[test]
    fn when_email_valid_should_create_email() {
        let email = Email::try_new("user@example.com".to_string()).unwrap();
        assert_eq!(email.as_ref(), "user@example.com");
    }

    #[test]
    fn when_email_valid_should_create_email_from_str() {
        let email = Email::try_from("user@example.com").unwrap();
        assert_eq!(email.as_ref(), "user@example.com");
    }

    #[test]
    fn when_email_valid_should_create_email_from_string() {
        let email = Email::try_from("user@example.com".to_string()).unwrap();
        assert_eq!(email.as_ref(), "user@example.com");
    }
}
