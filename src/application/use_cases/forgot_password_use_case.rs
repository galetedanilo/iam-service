use std::sync::Arc;

use crate::{
    application::{
        events::{event::Event, password_reset_requested_event::PasswordResetRequestedEvent},
        inputs::forgot_password_input::ForgotPasswordInput,
    },
    domain::{
        events::domain_event::EventType,
        models::user::UserError,
        object_values::{email::Email, status::Status},
        repositories::user_repository::UserRepository,
    },
};

#[derive(Clone)]
pub struct ForgotPasswordUseCase<R: UserRepository> {
    repository: Arc<R>,
}

impl<R: UserRepository> ForgotPasswordUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[tracing::instrument(name = "Requesting password reset", skip(self, input))]
    pub async fn execute(&self, input: ForgotPasswordInput) -> Result<(), UserError> {
        let email = Email::try_from(input.email())?;

        let Ok(Some(mut user)) = self
            .repository
            .find_by_email(&email)
            .await
            .map_err(UserError::from)
        else {
            return Ok(());
        };

        user.set_status(Status::PendingResetPassword);

        let event_payload =
            PasswordResetRequestedEvent::new(user.id().to_string(), user.email().to_string());
        let event = Event::new(
            EventType::PasswordResetRequested,
            "iam_service".to_string(),
            input.correlation_id().clone(),
            event_payload,
        );

        self.repository
            .save(&user, &event)
            .await
            .map_err(UserError::from)?;

        Ok(())
    }
}
