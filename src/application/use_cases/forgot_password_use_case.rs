use std::sync::Arc;

use crate::{
    application::inputs::forgot_password_input::ForgotPasswordInput,
    domain::{
        events::{
            event::{Event, EventType},
            password_reset_requested_event::PasswordResetRequestedEvent,
        },
        models::user::UserError,
        object_values::status::Status,
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
        let Ok(Some(mut user)) = self
            .repository
            .find_by_email(&input.email)
            .await
            .map_err(UserError::from)
        else {
            return Ok(());
        };

        user.set_status(Status::PendingResetPassword);

        let event_payload =
            PasswordResetRequestedEvent::new(user.id().clone(), user.email().clone());
        let event = Event::new(
            EventType::PasswordResetRequested,
            "iam_service".to_string(),
            event_payload,
        );

        self.repository
            .save(&user, &event)
            .await
            .map_err(UserError::from)?;

        Ok(())
    }
}
