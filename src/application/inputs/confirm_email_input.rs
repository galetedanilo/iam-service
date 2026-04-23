use crate::domain::{models::user::UserError, object_values::id::Id};

#[derive(Debug, Clone)]
pub struct ConfirmEmailInput {
    pub user_id: Id,
    pub token: String,
}

impl ConfirmEmailInput {
    pub fn try_new(user_id: String, token: String) -> Result<Self, UserError> {
        let user_id = Id::try_from(user_id)?;

        Ok(Self { user_id, token })
    }
}
