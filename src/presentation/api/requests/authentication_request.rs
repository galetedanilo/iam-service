use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct AuthenticationRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(
        min = 6,
        max = 15,
        message = "Password must be between 6 and 15 characters"
    ))]
    pub password: String,
}
