use crate::domain::{
    models::user::UserError,
    object_values::{email::Email, password::Password},
};

pub struct RegisterUserInput {
    pub email: Email,
    pub password: Password,
}

impl RegisterUserInput {
    pub fn try_new(email: String, password: String) -> Result<Self, UserError> {
        let email = Email::try_new(email)?;
        let password = Password::try_new(password)?;

        Ok(Self { email, password })
    }
}
