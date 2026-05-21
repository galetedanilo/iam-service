use std::sync::Arc;

use crate::{
    application::{
        events::{event::Event, user_registered_event::UserRegisteredEvent},
        inputs::register_user_input::RegisterUserInput,
    },
    domain::{
        events::domain_event::EventType,
        models::user::{User, UserError},
        object_values::{email::Email, password::Password},
        repositories::user_repository::UserRepository,
    },
};

#[derive(Clone)]
pub struct RegisterUserUseCase<R: UserRepository> {
    repository: Arc<R>,
}

impl<R: UserRepository> RegisterUserUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[tracing::instrument(name = "Registering a new user", skip(self, input))]
    pub async fn execute(&self, input: RegisterUserInput) -> Result<(), UserError> {
        let email = Email::try_from(input.email())?;
        let password = Password::try_from(input.password())?;

        if self
            .repository
            .find_by_email(&email)
            .await
            .map_err(UserError::from)?
            .is_some()
        {
            return Err(UserError::AlreadyExists(email.to_string()));
        }

        let user = User::new(email, password);

        let event_payload =
            UserRegisteredEvent::new(user.id().to_string(), user.email().to_string());
        let event = Event::new(
            EventType::UserRegistered,
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
