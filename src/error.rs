use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use derive_new::new;
use thiserror::Error;

use crate::models::response::ErrorResponse;

#[derive(Debug, new, Error)]
pub enum CustomServiceError {
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Not Found")]
    NotFound,

    #[new(value = "The request is invalid")]
    #[error("BadRequest: {0}")]
    BadRequest(String),

    #[error("Authentication Failed")]
    AuthenticationFailed,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ResponseError for CustomServiceError {
    fn status_code(&self) -> StatusCode {
        match *self {
            CustomServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            CustomServiceError::BadRequest(ref _message) => StatusCode::BAD_REQUEST,
            CustomServiceError::NotFound => StatusCode::NOT_FOUND,
            CustomServiceError::AuthenticationFailed => StatusCode::UNAUTHORIZED,
            CustomServiceError::Other(ref _e) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        ErrorResponse::new(self.status_code(), &self.to_string()).build()
    }
}
