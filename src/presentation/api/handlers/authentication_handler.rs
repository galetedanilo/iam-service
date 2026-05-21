use axum::extract::State;

use crate::{
    application::inputs::authentication_input::AuthenticationInput,
    domain::repositories::user_repository::UserRepository,
    presentation::api::{
        helpers::{
            app_error::AppErrorResponse, app_state::AppState, correlation_id::CorrelationId,
            validated_json::ValidatedJson,
        },
        requests::authentication_request::AuthenticationRequest,
    },
};

pub async fn authentication_handler<R: UserRepository>(
    CorrelationId(correlation_id): CorrelationId,
    State(state): State<AppState<R>>,
    ValidatedJson(payload): ValidatedJson<AuthenticationRequest>,
) -> Result<String, AppErrorResponse> {
    let command = AuthenticationInput::new(payload.email, payload.password, correlation_id);

    state
        .authentication_use_case
        .execute(command)
        .await
        .map_err(AppErrorResponse::from)
}
