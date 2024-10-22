// Make our own error that wraps `anyhow::Error`.
use anyhow::anyhow;
use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use tracing::error;

pub struct AppError(anyhow::Error);

impl AppError {
    pub fn new(msg: &str) -> AppError {
        AppError(anyhow!(msg.to_owned()))
    }
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let e = format!("{:?}", self.0);
        error!("AppError: {}", e);
        let body = Json(format!("{{\"error\":\"{e:?}\"}}\""));

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            // cors_response_headers(),
            body,
        )
            .into_response()
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for AppError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        AppError(anyhow!("{}", err))
    }
}
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError(err)
    }
}

impl From<askama::Error> for AppError {
    fn from(err: askama::Error) -> Self {
        AppError(anyhow!("{}", err))
    }
}

impl From<url::ParseError> for AppError {
    fn from(err: url::ParseError) -> Self {
        AppError(anyhow!("{}", err))
    }
}
