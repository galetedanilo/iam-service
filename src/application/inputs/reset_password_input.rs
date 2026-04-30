use crate::domain::{models::user::UserError, object_values::password::Password};

#[derive(Debug, Clone)]
pub struct ResetPasswordInput {
    pub jwt: String,
    pub new_password: Password,
}

impl ResetPasswordInput {
    pub fn try_new(jwt: String, new_password: String) -> Result<Self, UserError> {
        let new_password = Password::try_new(new_password)?;
        Ok(Self { jwt, new_password })
    }
}
