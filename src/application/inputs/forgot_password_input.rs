use crate::domain::{
    models::user::UserError,
    object_values::email::Email,
};

#[derive(Debug, Clone)]
pub struct ForgotPasswordInput {
    pub email: Email,
}

impl ForgotPasswordInput {
    pub fn try_new(email: String) -> Result<Self, UserError> {
        let email = Email::try_new(email)?;
        Ok(Self { email })
    }
}
