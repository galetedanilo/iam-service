use axum::{extract::State, http::StatusCode};

use crate::{
    application::inputs::forgot_password_input::ForgotPasswordInput,
    domain::repositories::user_repository::UserRepository,
    presentation::api::{
        helpers::{
            app_error::AppErrorResponse, app_state::AppState, validated_json::ValidatedJson,
        },
        requests::forgot_password_request::ForgotPasswordRequest,
    },
};

pub async fn forgot_password_handler<R: UserRepository>(
    State(state): State<AppState<R>>,
    ValidatedJson(payload): ValidatedJson<ForgotPasswordRequest>,
) -> Result<StatusCode, AppErrorResponse> {
    let command = ForgotPasswordInput::try_new(payload.email)?;

    state
        .forgot_password_use_case
        .execute(command)
        .await
        .map_err(AppErrorResponse::from)
        .map(|_| StatusCode::ACCEPTED)
}
