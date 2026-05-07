use crate::domain::{
    events::event::EventPayload,
    object_values::{email::Email, id::Id},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PasswordResetRequestedEvent {
    user_id: Id,
    email: Email,
}

impl PasswordResetRequestedEvent {
    pub fn new(user_id: Id, email: Email) -> Self {
        Self { user_id, email }
    }
}

impl EventPayload for PasswordResetRequestedEvent {
    fn get_payload(&self) -> String {
        format!(
            "{{\"user_id\": \"{}\", \"email\": \"{}\"}}",
            self.user_id, self.email
        )
    }
}
