use axum::{http::StatusCode, response::IntoResponse};
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::domain::models::user::UserError;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppErrorResponse {
    pub message: String,
    pub status_code: u16,
    pub timestamp: DateTime<Utc>,
    pub code: Option<String>,
    pub details: Option<String>,
}

impl IntoResponse for AppErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let status_code =
            StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        (status_code, axum::Json(self)).into_response()
    }
}

impl From<UserError> for AppErrorResponse {
    fn from(error: UserError) -> Self {
        let (message, status_code, details) = match error {
            UserError::AlreadyExists(details) => (
                "User already exists".to_string(),
                409,
                Some(details.to_string()),
            ),
            UserError::InvalidData(details) => {
                ("Invalid data".to_string(), 400, Some(details.to_string()))
            }
            UserError::NotFound(details) => {
                ("User not found".to_string(), 404, Some(details.to_string()))
            }
            UserError::VersionConflict(details) => (
                "Version conflict".to_string(),
                409,
                Some(details.to_string()),
            ),
            UserError::Unauthorized(details) => {
                ("Unauthorized".to_string(), 401, Some(details.to_string()))
            }
            UserError::Unknown(details) => {
                ("Unknown error".to_string(), 500, Some(details.to_string()))
            }
        };

        Self {
            message,
            status_code,
            timestamp: Utc::now(),
            code: None,
            details,
        }
    }
}
