use std::sync::Arc;

use jsonwebtoken::EncodingKey;

use crate::{
    application::use_cases::{
        confirm_email_use_case::ConfirmEmailUseCase, register_user_use_case::RegisterUserUseCase,
    },
    domain::repositories::user_repository::UserRepository,
};

#[derive(Clone)]
pub struct AppState<R: UserRepository> {
    pub confirm_email_use_case: Arc<ConfirmEmailUseCase<R>>,
    pub register_use_user_case: Arc<RegisterUserUseCase<R>>,
    pub encoding_key: Arc<EncodingKey>,
}

impl<R: UserRepository> AppState<R> {
    pub fn new(repository: Arc<R>, encoding_key: Arc<EncodingKey>) -> Self {
        Self {
            confirm_email_use_case: Arc::new(ConfirmEmailUseCase::new(repository.clone())),
            register_use_user_case: Arc::new(RegisterUserUseCase::new(repository.clone())),
            encoding_key,
        }
    }
}
