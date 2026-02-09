/// Configuration of a mock route
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RouteConfig {
    /// HTTP method (GET, POST, PUT, DELETE, etc)
    pub method: String,
    /// Endpoint path (e.g., /users, /api/posts)
    pub path: String,
    /// HTTP status code to return
    pub status: u16,
    /// Delay in milliseconds before responding
    pub delay_ms: u64,
    /// JSON response
    pub response: Value,
}

impl RouteConfig {
    /// Creates a new route configuration
    pub fn new(method: String, path: String, status: u16, delay_ms: u64, response: Value) -> Self {
        RouteConfig {
            method: method.to_uppercase(),
            path,
            status,
            delay_ms,
            response,
        }
    }

    /// Validates that the route is valid
    pub fn validate(&self) -> crate::error::Result<()> {
        if self.method.is_empty() {
            return Err(crate::error::AppError::SerdeError(
                "Method cannot be empty".to_string(),
            ));
        }
        if self.path.is_empty() {
            return Err(crate::error::AppError::SerdeError(
                "Path cannot be empty".to_string(),
            ));
        }
        if self.status == 0 || self.status > 599 {
            return Err(crate::error::AppError::SerdeError(
                "Status must be between 100 and 599".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_validation() {
        let valid_route = RouteConfig::new(
            "GET".to_string(),
            "/test".to_string(),
            200,
            0,
            serde_json::json!({"ok": true}),
        );
        assert!(valid_route.validate().is_ok());

        let invalid_status = RouteConfig::new(
            "GET".to_string(),
            "/test".to_string(),
            999,
            0,
            serde_json::json!({"ok": true}),
        );
        assert!(invalid_status.validate().is_err());
    }
}
