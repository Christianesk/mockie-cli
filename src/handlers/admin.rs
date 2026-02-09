/// Admin API handlers
use axum::response::IntoResponse;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::models::RouteConfig;
use crate::storage::RouteStore;

#[derive(Deserialize, Debug)]
pub struct AddRouteRequest {
    pub method: String,
    pub path: String,
    #[serde(default = "default_status")]
    pub status: u16,
    #[serde(default)]
    pub delay_ms: u64,
    pub response: serde_json::Value,
}

fn default_status() -> u16 {
    200
}

#[derive(Serialize)]
pub struct RouteResponse {
    pub method: String,
    pub path: String,
    pub status: u16,
    pub delay_ms: u64,
}

/// Adds a new route
pub async fn add_route(
    State(routes): State<RouteStore>,
    Json(payload): Json<AddRouteRequest>,
) -> impl IntoResponse {
    // Create and validate the new route
    let route = RouteConfig::new(
        payload.method.to_uppercase(),
        payload.path.clone(),
        payload.status,
        payload.delay_ms,
        payload.response,
    );

    if let Err(e) = route.validate() {
        return (
            axum::http::StatusCode::BAD_REQUEST,
            Json(json!({"error": e.to_string()})),
        )
            .into_response();
    }

    // Insert into storage
    {
        let mut map = match routes.write() {
            Ok(m) => m,
            Err(e) => {
                return (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": format!("Storage error: {}", e)})),
                )
                    .into_response();
            }
        };
        map.insert((route.method.clone(), route.path.clone()), route);
    }

    Json(json!({ "ok": true })).into_response()
}

/// Lists all registered routes
pub async fn list_routes(State(routes): State<RouteStore>) -> impl IntoResponse {
    let map = match routes.read() {
        Ok(m) => m,
        Err(e) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Storage error: {}", e)})),
            )
                .into_response();
        }
    };

    let list: Vec<RouteResponse> = map
        .values()
        .map(|r| RouteResponse {
            method: r.method.clone(),
            path: r.path.clone(),
            status: r.status,
            delay_ms: r.delay_ms,
        })
        .collect();

    Json(list).into_response()
}
