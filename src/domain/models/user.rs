use chrono::{DateTime, Utc};
use thiserror::Error;

use crate::domain::{
    enums::{audience::Audience, scope::Scope},
    events::user_registered_event::UserRegisteredEvent,
    object_values::{email::Email, id::Id, password::Password, status::Status},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct User {
    id: Id,
    email: Email,
    password: Password,
    status: Status,
    audiences: Vec<Audience>,
    scopes: Vec<Scope>,
    token_hash: Option<String>,
    token_expires_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum UserError {
    #[error("User with email {0} already exists")]
    AlreadyExists(String),

    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("User not found with id: {0}")]
    NotFound(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl User {
    pub fn new(email: Email, password: Password) -> Self {
        let id = Id::generate();
        let now = Utc::now();
        let token_hash = Some(nanoid::nanoid!(32));
        let token_expires_at = Some(now + chrono::Duration::hours(1));

        Self {
            id,
            email,
            password,
            status: Status::PendingConfirmation,
            audiences: vec![Audience::default()],
            scopes: vec![Scope::default()],
            token_hash,
            token_expires_at,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn from_parts(
        id: Id,
        email: Email,
        password: Password,
        status: Status,
        audiences: Vec<Audience>,
        scopes: Vec<Scope>,
        token_hash: Option<String>,
        token_expires_at: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            email,
            password,
            status,
            audiences,
            scopes,
            token_hash,
            token_expires_at,
            created_at,
            updated_at,
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

    pub fn audiences(&self) -> &[Audience] {
        &self.audiences
    }

    pub fn scopes(&self) -> &[Scope] {
        &self.scopes
    }

    pub fn token_hash(&self) -> &Option<String> {
        &self.token_hash
    }

    pub fn token_expires_at(&self) -> &Option<DateTime<Utc>> {
        &self.token_expires_at
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    pub fn set_password(&mut self, new_password: Password) {
        self.password = new_password;
        self.updated_at = Utc::now();
    }

    pub fn set_status(&mut self, new_status: Status) {
        self.status = new_status;
        self.updated_at = Utc::now();
    }

    pub fn set_audiences(&mut self, new_audiences: Vec<Audience>) {
        self.audiences = new_audiences;
        self.updated_at = Utc::now();
    }

    pub fn set_scopes(&mut self, new_scopes: Vec<Scope>) {
        self.scopes = new_scopes;
        self.updated_at = Utc::now();
    }

    pub fn update_token(&mut self) {
        self.token_hash = Some(nanoid::nanoid!(32));
        self.token_expires_at = Some(Utc::now() + chrono::Duration::hours(1));
        self.updated_at = Utc::now();
    }

    pub fn invalidate_token(&mut self) {
        self.token_hash = None;
        self.token_expires_at = None;
        self.updated_at = Utc::now();
    }

    pub fn is_token_valid(&self, token: &str) -> bool {
        if let (Some(hash), Some(expires_at)) = (&self.token_hash, &self.token_expires_at) {
            hash == token && Utc::now() < *expires_at
        } else {
            false
        }
    }

    pub fn can_authenticate(&self) -> bool {
        self.status == Status::Active
    }

    pub fn get_register_event(&self) -> UserRegisteredEvent {
        UserRegisteredEvent::new(self.id.clone(), self.email.clone())
    }
}
