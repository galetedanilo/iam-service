use chrono::{DateTime, Utc};

use crate::domain::{
    events::event::Event,
    object_values::{email::Email, id::Id},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserRegisteredEvent {
    user_id: Id,
    email: Email,
    occurred_at: DateTime<Utc>,
}

impl UserRegisteredEvent {
    pub fn new(user_id: Id, email: Email) -> Self {
        Self {
            user_id,
            email,
            occurred_at: Utc::now(),
        }
    }

    pub fn user_id(&self) -> &Id {
        &self.user_id
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn occurred_at(&self) -> &DateTime<Utc> {
        &self.occurred_at
    }
}

impl Event for UserRegisteredEvent {
    fn event_type(&self) -> &'static str {
        "UserCreatedEvent"
    }

    fn to_json(&self) -> String {
        format!(
            "{{\"user_id\": \"{}\", \"email\": \"{}\", \"occurred_at\": \"{}\", \"event_type\": \"{}\"}}",
            self.user_id,
            self.email,
            self.occurred_at,
            self.event_type()
        )
    }

    fn occurred_at(&self) -> &DateTime<Utc> {
        &self.occurred_at
    }
}
