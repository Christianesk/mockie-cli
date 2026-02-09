/// Mock handler - intercepts all non-matching requests
use axum::{
    extract::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use tokio::time::{sleep, Duration};

use crate::storage::RouteStore;

pub async fn handle_mock_request(routes: RouteStore, req: Request) -> Response {
    let method = req.method().as_str().to_string();
    let path = req.uri().path().to_string();

    // Search for route in storage
    let route = {
        let map = match routes.read() {
            Ok(m) => m,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Internal server error"})),
                )
                    .into_response();
            }
        };
        map.get(&(method, path.clone())).cloned()
    };

    if let Some(route) = route {
        // Apply delay if configured
        if route.delay_ms > 0 {
            sleep(Duration::from_millis(route.delay_ms)).await;
        }

        (
            StatusCode::from_u16(route.status).unwrap_or(StatusCode::OK),
            Json(route.response),
        )
            .into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Mock endpoint not found", "path": path })),
        )
            .into_response()
    }
}
