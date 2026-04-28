use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::{
    enums::{audience::Audience, scope::Scope},
    object_values::{email::Email, id::Id},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub aud: Vec<String>,
    pub scopes: Vec<String>,
    pub sub: String,
    pub email: String,
    pub exp: usize,
    pub iat: usize,
}

impl Claims {
    pub fn new(audiences: &Vec<Audience>, scopes: &Vec<Scope>, id: &Id, email: &Email) -> Self {
        let exp = Utc::now()
            .checked_add_signed(Duration::hours(24)) // Expira em 24h
            .expect("valid timestamp")
            .timestamp() as usize;

        let iat = Utc::now().timestamp() as usize;

        let aud = audiences.iter().map(|s| s.to_string()).collect();
        let scopes = scopes.iter().map(|s| s.to_string()).collect();
        let sub = id.to_string();
        let email = email.to_string();

        Self {
            aud,
            scopes,
            sub,
            email,
            exp,
            iat,
        }
    }
}
