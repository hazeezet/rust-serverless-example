pub mod util;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
    pub status_code: u16,
    pub error: Option<String>,
    pub message: String,
    pub data: Option<T>,
}

pub enum ErrorCode {
    BadRequest,
    NotFound,
    UnknownError,
}

impl ErrorCode {
    /// Get the corresponding error message for the error code
    pub fn message(&self) -> &'static str {
        match self {
            ErrorCode::BadRequest => "Bad request, please check your input",
            ErrorCode::NotFound => "Resource not found",
            ErrorCode::UnknownError => "An unexpected error occurred",
        }
    }

    /// Get the error code as a string
    pub fn code(&self) -> &'static str {
        match self {
            ErrorCode::BadRequest => "BAD_REQUEST",
            ErrorCode::NotFound => "NOT_FOUND",
            ErrorCode::UnknownError => "UNKNOWN_ERROR",
        }
    }

    /// Get status code associated with the error
    pub fn status_code(&self) -> u16 {
        match self {
            ErrorCode::BadRequest => 400,
            ErrorCode::NotFound => 404,
            ErrorCode::UnknownError => 500,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseError {
    pub status_code: u16,
    pub code: String,
    pub message: String,
}

impl ResponseError {
    /// Create ResponseError with custom message
    pub fn new(error: ErrorCode, custom_message: Option<&str>) -> Self {
        Self {
            message: custom_message.unwrap_or(error.message()).to_string(),
            code: error.code().to_string(),
            status_code: error.status_code(),
        }
    }
}
