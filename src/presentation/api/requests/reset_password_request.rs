use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct ResetPasswordRequest {
    #[validate(length(min = 1, message = "Token is required"))]
    pub token: String,

    #[validate(length(
        min = 6,
        max = 15,
        message = "Password must be between 6 and 15 characters"
    ))]
    pub password: String,
}
