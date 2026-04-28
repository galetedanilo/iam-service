use std::sync::Arc;

use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};

use crate::{
    application::{helpers::token::Claims, inputs::authentication_input::AuthenticationInput},
    domain::{models::user::UserError, repositories::user_repository::UserRepository},
};

pub struct AuthenticationUseCase<R: UserRepository> {
    repository: Arc<R>,
    encoding_key: Arc<EncodingKey>,
}

impl<R: UserRepository> AuthenticationUseCase<R> {
    pub fn new(repository: Arc<R>, encoding_key: Arc<EncodingKey>) -> Self {
        Self {
            repository,
            encoding_key,
        }
    }

    #[tracing::instrument(
        name = "Authentication application",
        skip(self, input),
        fields(user.email = %input.email),
        err)]
    pub async fn execute(&self, input: AuthenticationInput) -> Result<String, UserError> {
        if let Some(user) = self
            .repository
            .find_by_email(&input.email)
            .await
            .map_err(UserError::from)?
        {
            if !user.status().can_authenticate() || !user.verify_password(input.password) {
                return Err(UserError::Unauthorized(input.email.to_string()));
            }

            let claims = Claims::new(user.audiences(), user.scopes(), user.id(), user.email());

            // Configura o Header para RS256
            let header = Header::new(Algorithm::EdDSA);

            let token = encode(&header, &claims, &self.encoding_key)
                .map_err(|_| UserError::Unknown("Encoding token error".to_string()))?;

            return Ok(token);
        }

        Err(UserError::Unauthorized(input.email.to_string()))
    }
}
