use std::sync::Arc;

use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::Deserialize;

use crate::{
    application::{
        events::{event::Event, user_activated_event::UserActivatedEvent},
        inputs::confirm_email_input::ConfirmEmailInput,
    },
    domain::{
        enums::audience::Audience,
        events::domain_event::EventType,
        models::user::UserError,
        object_values::{id::Id, status::Status},
        repositories::user_repository::UserRepository,
    },
};

#[derive(Debug, Deserialize)]
struct ConfirmEmailClaims {
    sub: String,
    aud: Vec<String>,
    exp: usize,
    iat: usize,
}

#[derive(Clone)]
pub struct ConfirmEmailUseCase<R: UserRepository> {
    repository: Arc<R>,
    decoding_key: Arc<DecodingKey>,
}

impl<R: UserRepository> ConfirmEmailUseCase<R> {
    pub fn new(repository: Arc<R>, decoding_key: Arc<DecodingKey>) -> Self {
        Self {
            repository,
            decoding_key,
        }
    }

    #[tracing::instrument(name = "Confirming user email", skip(self))]
    pub async fn execute(&self, input: ConfirmEmailInput) -> Result<(), UserError> {
        let mut validation = Validation::new(Algorithm::EdDSA);

        validation.set_audience(&[Audience::EmailService]);

        validation.validate_exp = true;
        validation.set_required_spec_claims(&["sub", "exp", "aud"]);

        let token_data = decode::<ConfirmEmailClaims>(input.jwt(), &self.decoding_key, &validation)
            .map_err(|_| UserError::Unauthorized("Invalid or expired token".to_string()))?;

        let user_id = Id::try_from(token_data.claims.sub.as_str()).map_err(UserError::from)?;

        if let Ok(Some(mut user)) = self
            .repository
            .find_by_id(&user_id)
            .await
            .map_err(UserError::from)
        {
            if user.status() != &Status::PendingConfirmation {
                return Err(UserError::VersionConflict(
                    "User is already confirmed".to_string(),
                ));
            }

            user.confirm_email_and_activate_user();

            let event_payload =
                UserActivatedEvent::new(user.id().to_string(), user.email().to_string());
            let event = Event::new(
                EventType::UserActivated,
                "iam_service".to_string(),
                input.correlation_id().clone(),
                event_payload,
            );

            self.repository
                .save(&user, &event)
                .await
                .map_err(UserError::from)?;

            return Ok(());
        }
        return Err(UserError::Unauthorized(
            "Invalid or expired token".to_string(),
        ));
    }
}
