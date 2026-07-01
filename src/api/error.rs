use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use utoipa::ToSchema;

use crate::application::ServiceError;

#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    code: &'static str,
    message: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorEnvelope {
    error: ErrorBody,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorBody {
    code: String,
    message: String,
    request_id: String,
}

impl ApiError {
    pub fn bad_request(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code,
            message: message.into(),
        }
    }

    fn new(status: StatusCode, code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status,
            code,
            message: message.into(),
        }
    }
}

impl From<ServiceError> for ApiError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::NotFound => {
                Self::new(StatusCode::NOT_FOUND, "NOT_FOUND", "resource not found")
            }
            ServiceError::Forbidden => {
                Self::new(StatusCode::FORBIDDEN, "FORBIDDEN", "access denied")
            }
            ServiceError::Validation(message) => {
                Self::new(StatusCode::BAD_REQUEST, "VALIDATION_ERROR", message)
            }
            ServiceError::Conflict(message) => Self::new(StatusCode::CONFLICT, "CONFLICT", message),
            ServiceError::Storage(message) => {
                Self::new(StatusCode::INTERNAL_SERVER_ERROR, "STORAGE_ERROR", message)
            }
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let request_id = "see-x-request-id";
        (
            self.status,
            Json(ErrorEnvelope {
                error: ErrorBody {
                    code: self.code.to_owned(),
                    message: self.message,
                    request_id: request_id.to_owned(),
                },
            }),
        )
            .into_response()
    }
}
