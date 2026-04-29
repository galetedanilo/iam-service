use std::sync::Arc;

use jsonwebtoken::{DecodingKey, EncodingKey};

use crate::{
    application::use_cases::{
        authentication_use_case::AuthenticationUseCase,
        confirm_email_use_case::ConfirmEmailUseCase, register_user_use_case::RegisterUserUseCase,
    },
    domain::repositories::user_repository::UserRepository,
};

#[derive(Clone)]
pub struct AppState<R: UserRepository> {
    pub authentication_use_case: Arc<AuthenticationUseCase<R>>,
    pub confirm_email_use_case: Arc<ConfirmEmailUseCase<R>>,
    pub register_use_user_case: Arc<RegisterUserUseCase<R>>,
    pub encoding_key: Arc<EncodingKey>,
    pub decoding_key: Arc<DecodingKey>,
}

impl<R: UserRepository> AppState<R> {
    pub fn new(repository: Arc<R>, encoding_key: Arc<EncodingKey>, decoding_key: Arc<DecodingKey>) -> Self {
        Self {
            authentication_use_case: Arc::new(AuthenticationUseCase::new(
                repository.clone(),
                encoding_key.clone(),
            )),
            confirm_email_use_case: Arc::new(ConfirmEmailUseCase::new(
                repository.clone(),
                decoding_key.clone(),
            )),
            register_use_user_case: Arc::new(RegisterUserUseCase::new(repository.clone())),
            encoding_key,
            decoding_key,
        }
    }
}
