use std::sync::Arc;

use jsonwebtoken::EncodingKey;

use crate::{
    application::use_cases::register_user_use_case::RegisterUserUseCase,
    domain::repositories::user_repository::UserRepository,
};

#[derive(Clone)]
pub struct AppState<R: UserRepository> {
    pub register_use_user_case: Arc<RegisterUserUseCase<R>>,
    pub encoding_key: Arc<EncodingKey>,
}

impl<R: UserRepository> AppState<R> {
    pub fn new(repository: Arc<R>, encoding_key: Arc<EncodingKey>) -> Self {
        Self {
            register_use_user_case: Arc::new(RegisterUserUseCase::new(repository)),
            encoding_key,
        }
    }
}
