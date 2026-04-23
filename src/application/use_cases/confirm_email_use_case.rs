use std::sync::Arc;

use crate::{
    application::inputs::confirm_email_input::ConfirmEmailInput,
    domain::{
        events::{
            event::{Event, EventType},
            user_activated_event::UserActivatedEvent,
        },
        models::user::UserError,
        repositories::user_repository::UserRepository,
    },
};

#[derive(Clone)]
pub struct ConfirmEmailUseCase<R: UserRepository> {
    repository: Arc<R>,
}

impl<R: UserRepository> ConfirmEmailUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[tracing::instrument(name = "Confirming user email", skip(self))]
    pub async fn execute(&self, input: ConfirmEmailInput) -> Result<(), UserError> {
        if let Ok(Some(mut user)) = self
            .repository
            .find_by_id(&input.user_id)
            .await
            .map_err(UserError::from)
        {
            if user.status().is_confirmed() {
                return Err(UserError::VersionConflict(
                    "User is already confirmed".to_string(),
                ));
            }

            if user.is_token_valid(&input.token) {
                user.confirm_email_and_activate_user();

                let event_payload =
                    UserActivatedEvent::new(user.id().clone(), user.email().clone());
                let event = Event::new(EventType::UserActivated, event_payload);

                self.repository
                    .save(&user, &event)
                    .await
                    .map_err(UserError::from)?;

                return Ok(());
            } else {
                return Err(UserError::Unauthorized(
                    "Invalid or expired token".to_string(),
                ));
            }
        }

        Err(UserError::NotFound(input.user_id.to_string()))
    }
}
