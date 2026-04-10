use thiserror::Error;

use crate::domain::{
    events::event::Event,
    models::user::{User, UserError},
    object_values::{email::Email, id::Id},
};

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, UserRepositoryError>;

    async fn find_by_id(&self, id: &Id) -> Result<Option<User>, UserRepositoryError>;

    async fn save(&self, user: &User, event: Box<dyn Event>) -> Result<(), UserRepositoryError>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum UserRepositoryError {
    #[error("User not found")]
    UserNotFound,

    #[error("Unique constraint violation")]
    UniqueViolation,

    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<UserRepositoryError> for UserError {
    fn from(error: UserRepositoryError) -> Self {
        match error {
            UserRepositoryError::UserNotFound => UserError::NotFound("User not found".to_string()),
            UserRepositoryError::UniqueViolation => {
                UserError::AlreadyExists("User already exists".to_string())
            }
            UserRepositoryError::InvalidData(msg) => UserError::InvalidData(msg),
            UserRepositoryError::Unknown(msg) => UserError::Unknown(msg),
        }
    }
}
