use std::sync::Arc;

use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::Deserialize;

use crate::{
    application::inputs::reset_password_input::ResetPasswordInput,
    domain::{
        enums::audience::Audience,
        events::{
            event::{Event, EventType},
            password_reset_completed_event::PasswordResetCompletedEvent,
        },
        models::user::UserError,
        object_values::{id::Id, status::Status},
        repositories::user_repository::UserRepository,
    },
};

#[derive(Debug, Deserialize)]
struct PasswordResetClaims {
    sub: String,
    aud: Vec<String>,
    exp: usize,
    iat: usize,
}

#[derive(Clone)]
pub struct ResetPasswordUseCase<R: UserRepository> {
    repository: Arc<R>,
    decoding_key: Arc<DecodingKey>,
}

impl<R: UserRepository> ResetPasswordUseCase<R> {
    pub fn new(repository: Arc<R>, decoding_key: Arc<DecodingKey>) -> Self {
        Self {
            repository,
            decoding_key,
        }
    }

    #[tracing::instrument(name = "Resetting user password", skip(self, input))]
    pub async fn execute(&self, input: ResetPasswordInput) -> Result<(), UserError> {
        let mut validation = Validation::new(Algorithm::EdDSA);

        validation.set_audience(&[Audience::EmailService]);

        validation.validate_exp = true;
        validation.set_required_spec_claims(&["sub", "exp", "aud"]);

        let token_data = decode::<PasswordResetClaims>(&input.jwt, &self.decoding_key, &validation)
            .map_err(|_| UserError::Unauthorized("Invalid or expired token".to_string()))?;

        let user_id = Id::try_from(token_data.claims.sub).map_err(UserError::from)?;

        if let Ok(Some(mut user)) = self
            .repository
            .find_by_id(&user_id)
            .await
            .map_err(UserError::from)
        {
            if user.status() != &Status::PendingResetPassword {
                return Err(UserError::Unauthorized(
                    "Password reset is not pending".to_string(),
                ));
            }

            user.set_password(input.new_password);
            user.set_status(Status::Active);

            let event_payload =
                PasswordResetCompletedEvent::new(user.id().clone(), user.email().clone());
            let event = Event::new(EventType::PasswordResetCompleted, event_payload);

            self.repository
                .save(&user, &event)
                .await
                .map_err(UserError::from)?;

            return Ok(());
        }

        Err(UserError::Unauthorized(
            "Invalid or expired token".to_string(),
        ))
    }
}
