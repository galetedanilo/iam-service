use axum::{extract::State, http::StatusCode};

use crate::{
    application::inputs::register_user_input::RegisterUserInput,
    domain::repositories::user_repository::UserRepository,
    presentation::api::{
        helpers::{
            app_error::AppErrorResponse, app_state::AppState, validated_json::ValidatedJson,
        },
        requests::register_user_request::RegisterUserRequest,
    },
};

pub async fn register_user_handler<R: UserRepository>(
    State(state): State<AppState<R>>,
    ValidatedJson(payload): ValidatedJson<RegisterUserRequest>,
) -> Result<StatusCode, AppErrorResponse> {
    let command = RegisterUserInput::try_new(payload.email, payload.password)?;

    state
        .register_use_user_case
        .execute(command)
        .await
        .map_err(AppErrorResponse::from)
        .map(|_| StatusCode::CREATED)
}
