use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    application::inputs::confirm_email_input::ConfirmEmailInput,
    domain::repositories::user_repository::UserRepository,
    presentation::api::helpers::{app_error::AppErrorResponse, app_state::AppState},
};

pub async fn confirm_email_handler<R: UserRepository>(
    State(state): State<AppState<R>>,
    Path(jwt): Path<String>,
) -> Result<StatusCode, AppErrorResponse> {
    let command = ConfirmEmailInput::new(jwt);

    state
        .confirm_email_use_case
        .execute(command)
        .await
        .map_err(AppErrorResponse::from)
        .map(|_| StatusCode::OK)
}
