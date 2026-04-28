use crate::domain::{models::user::UserError, object_values::email::Email};

#[derive(Debug, Clone)]
pub struct AuthenticationInput {
    pub email: Email,
    pub password: String,
}

impl AuthenticationInput {
    pub fn try_new(email: String, password: String) -> Result<Self, UserError> {
        let email = Email::try_from(email)?;

        Ok(Self { email, password })
    }
}
