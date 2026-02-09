/// Custom error types for the application
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::fmt;

/// Result type alias using custom error
pub type Result<T> = std::result::Result<T, AppError>;

/// Possible errors in the application
#[derive(Debug)]
pub enum AppError {
    /// I/O error (file not found, etc)
    Io(std::io::Error),
    /// Error parsing JSON
    Json(serde_json::Error),
    /// Route not found
    #[allow(dead_code)]
    RouteNotFound(String),
    /// Error serializing/deserializing
    SerdeError(String),
    /// Internal server error
    Internal(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "I/O error: {}", e),
            AppError::Json(e) => write!(f, "JSON error: {}", e),
            AppError::RouteNotFound(path) => write!(f, "Route not found: {}", path),
            AppError::SerdeError(e) => write!(f, "Serialization error: {}", e),
            AppError::Internal(e) => write!(f, "Internal error: {}", e),
        }
    }
}

impl std::error::Error for AppError {}

// Automatic conversions from other error types
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Json(err)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::Internal(format!("HTTP error: {}", err))
    }
}

// Convert AppError to HTTP response
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Io(ref e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::Json(ref e) => (StatusCode::BAD_REQUEST, e.to_string()),
            AppError::RouteNotFound(ref path) => {
                (StatusCode::NOT_FOUND, format!("Route not found: {}", path))
            }
            AppError::SerdeError(ref e) => (StatusCode::BAD_REQUEST, e.clone()),
            AppError::Internal(ref e) => (StatusCode::INTERNAL_SERVER_ERROR, e.clone()),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
