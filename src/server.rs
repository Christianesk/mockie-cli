/// Main HTTP server
use axum::{extract::Request, routing::post, Router};
use tokio::net::TcpListener;

use crate::handlers::{add_route, handle_mock_request, list_routes};
use crate::storage::RouteStore;

/// Starts the HTTP server on the specified port
pub async fn run(routes: RouteStore, port: u16) -> crate::error::Result<()> {
    let routes_for_fallback = routes.clone();
    let app = Router::new()
        // Admin API
        .route("/__admin/routes", post(add_route).get(list_routes))
        // Catch-all for mock endpoints
        .fallback(|request: Request| async move {
            handle_mock_request(routes_for_fallback.clone(), request).await
        })
        .with_state(routes);

    let listener = TcpListener::bind(("0.0.0.0", port)).await?;

    println!("🚀 Mock server listening at http://localhost:{port}");

    axum::serve(listener, app).await?;

    Ok(())
}
