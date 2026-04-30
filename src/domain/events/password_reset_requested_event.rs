use crate::domain::{
    events::event::EventPayload,
    object_values::{email::Email, id::Id},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PasswordResetRequestedEvent {
    user_id: Id,
    email: Email,
    token: String,
}

impl PasswordResetRequestedEvent {
    pub fn new(user_id: Id, email: Email, token: String) -> Self {
        Self {
            user_id,
            email,
            token,
        }
    }

    pub fn user_id(&self) -> &Id {
        &self.user_id
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

impl EventPayload for PasswordResetRequestedEvent {
    fn get_payload(&self) -> String {
        format!(
            "{{\"user_id\": \"{}\", \"email\": \"{}\", \"token\": \"{}\"}}",
            self.user_id, self.email, self.token
        )
    }
}
