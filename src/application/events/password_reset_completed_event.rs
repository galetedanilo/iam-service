use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct PasswordResetCompletedEvent {
    user_id: String,
    email: String,
}

impl PasswordResetCompletedEvent {
    pub fn new(user_id: String, email: String) -> Self {
        Self { user_id, email }
    }
}
