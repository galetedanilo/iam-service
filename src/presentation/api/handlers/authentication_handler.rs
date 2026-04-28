use axum::{extract::State};

use crate::{
    application::inputs::authentication_input::AuthenticationInput,
    domain::repositories::user_repository::UserRepository,
    presentation::api::{
        helpers::{
            app_error::AppErrorResponse, app_state::AppState, validated_json::ValidatedJson,
        },
        requests::authentication_request::AuthenticationRequest,
    },
};

pub async fn authentication_handler<R: UserRepository>(
    State(state): State<AppState<R>>,
    ValidatedJson(payload): ValidatedJson<AuthenticationRequest>,
) -> Result<String, AppErrorResponse> {
    let command = AuthenticationInput::try_new(payload.email, payload.password)?;

    state
        .authentication_use_case
        .execute(command)
        .await
        .map_err(AppErrorResponse::from)
}
