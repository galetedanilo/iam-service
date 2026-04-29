use std::sync::Arc;

use crate::{
    application::inputs::register_user_input::RegisterUserInput,
    domain::{
        events::{
            event::{Event, EventType},
            user_registered_event::UserRegisteredEvent,
        },
        models::user::{User, UserError},
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
        if self
            .repository
            .find_by_email(&input.email)
            .await
            .map_err(UserError::from)?
            .is_some()
        {
            return Err(UserError::AlreadyExists(input.email.to_string()));
        }

        let user = User::new(input.email, input.password);
        let token = user.token_hash().as_ref().ok_or_else(|| {
            UserError::InvalidData("Token was not generated for new user".to_string())
        })?;
        
        let event_payload =
            UserRegisteredEvent::new(user.id().clone(), user.email().clone(), token.clone());
        let event = Event::new(EventType::UserRegistered, event_payload);

        self.repository
            .save(&user, &event)
            .await
            .map_err(UserError::from)?;

        Ok(())
    }
}
