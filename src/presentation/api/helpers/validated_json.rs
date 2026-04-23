use axum::{Json, body::Body, extract::FromRequest, http::Request};
use chrono::Utc;

use serde::de::DeserializeOwned;
use validator::Validate;

use crate::presentation::api::helpers::app_error::AppErrorResponse;

pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
{
    type Rejection = AppErrorResponse;

    async fn from_request(req: Request<Body>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) =
            Json::<T>::from_request(req, _state)
                .await
                .map_err(|err| AppErrorResponse {
                    message: "Invalid JSON".to_string(),
                    status_code: 400,
                    timestamp: Utc::now(),
                    code: None,
                    details: Some(err.to_string()),
                })?;

        value.validate().map_err(|err| {
            let error_messages: Vec<String> = err
                .field_errors()
                .iter()
                .flat_map(|(field, errors)| {
                    errors.iter().map(move |error| {
                        format!(
                            "{}: {}",
                            field,
                            error
                                .message
                                .clone()
                                .unwrap_or_else(|| "Validation error".into())
                        )
                    })
                })
                .collect();

            AppErrorResponse {
                message: "Validation error".to_string(),
                status_code: 400,
                timestamp: Utc::now(),
                code: None,
                details: Some(error_messages.join(", ")),
            }
        })?;

        Ok(ValidatedJson(value))
    }
}

#[cfg(test)]
mod tests {

    use crate::presentation::api::requests::register_user_request::RegisterUserRequest;

    use super::*;
    use axum::{
        Router,
        body::Body,
        http::{Request, StatusCode},
        routing::post,
    };
    use serde_json::json;
    use tower::ServiceExt;

    #[tokio::test]
    async fn when_valid_user_data_is_provided_then_user_is_created() {
        let app = Router::new().route(
            "/test",
            post(
                |ValidatedJson(input): ValidatedJson<RegisterUserRequest>| async move {
                    assert_eq!(input.email, "john.doe@email.com");
                    assert_eq!(input.password, "password123");
                },
            ),
        );

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .header("content-type", "application/json")
            .body(Body::from(
                json!({
                    "email": "john.doe@email.com",
                    "password": "password123",
                })
                .to_string(),
            ))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn when_invalid_json_then_returns_bad_request() {
        let app = Router::new().route(
            "/test",
            post(|_: ValidatedJson<RegisterUserRequest>| async {
                // This should not be called
            }),
        );

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .header("content-type", "application/json")
            .body(Body::from("invalid json"))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn when_validation_fails_then_returns_bad_request() {
        let app = Router::new().route(
            "/test",
            post(|_: ValidatedJson<RegisterUserRequest>| async {
                // This should not be called
            }),
        );

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .header("content-type", "application/json")
            .body(Body::from(
                json!({
                    "email": "not-an-email",
                    "password": "short",
                })
                .to_string(),
            ))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
