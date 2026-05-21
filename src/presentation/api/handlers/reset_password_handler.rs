use axum::{extract::State, http::StatusCode};

use crate::{
    application::inputs::reset_password_input::ResetPasswordInput,
    domain::repositories::user_repository::UserRepository,
    presentation::api::{
        helpers::{
            app_error::AppErrorResponse, app_state::AppState, correlation_id::CorrelationId,
            validated_json::ValidatedJson,
        },
        requests::reset_password_request::ResetPasswordRequest,
    },
};

pub async fn reset_password_handler<R: UserRepository>(
    CorrelationId(correlation_id): CorrelationId,
    State(state): State<AppState<R>>,
    ValidatedJson(payload): ValidatedJson<ResetPasswordRequest>,
) -> Result<StatusCode, AppErrorResponse> {
    let command = ResetPasswordInput::new(payload.token, payload.password, correlation_id);

    state
        .reset_password_use_case
        .execute(command)
        .await
        .map_err(AppErrorResponse::from)
        .map(|_| StatusCode::OK)
}
