use chrono::{DateTime, Utc};

use crate::domain::object_values::{email::Email, id::Id, password::Password, status::Status};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserByEmail {
    id: Id,
    email: Email,
    password: Password,
    status: Status,
    token_hash: Option<String>,
    token_expires_at: Option<DateTime<Utc>>,
}

impl UserByEmail {
    pub fn from_parts(
        id: Id,
        email: Email,
        password: Password,
        status: Status,
        token_hash: Option<String>,
        token_expires_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            email,
            password,
            status,
            token_hash,
            token_expires_at,
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn password(&self) -> &Password {
        &self.password
    }

    pub fn status(&self) -> &Status {
        &self.status
    }
}
