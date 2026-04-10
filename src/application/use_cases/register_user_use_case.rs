use std::sync::Arc;

use crate::{
    application::inputs::register_user_input::RegisterUserInput,
    domain::{
        models::user::{User, UserError},
        repositories::user_repository::UserRepository,
    },
};

#[derive(Clone)]
pub struct RegisterUserUsecase<R: UserRepository + Send + Sync> {
    repository: Arc<R>,
}

impl<R: UserRepository + Send + Sync> RegisterUserUsecase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

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
        let user_registered_event = user.get_register_event();

        self.repository
            .save(&user, Box::new(user_registered_event))
            .await
            .map_err(UserError::from)?;

        Ok(())
    }
}
