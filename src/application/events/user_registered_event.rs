use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserRegisteredEvent {
    user_id: String,
    email: String,
}

impl UserRegisteredEvent {
    pub fn new(user_id: String, email: String) -> Self {
        Self { user_id, email }
    }
}
