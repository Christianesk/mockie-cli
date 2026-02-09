/// Storage layer - Abstraction for persistence
pub mod file_storage;

use crate::error::Result;
use crate::models::RouteConfig;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub use file_storage::FileStorage;

pub type RouteStore = Arc<RwLock<HashMap<(String, String), RouteConfig>>>;

/// Trait for storage abstraction
#[allow(dead_code)]
#[async_trait::async_trait]
pub trait Storage: Send + Sync {
    /// Loads all routes from storage
    async fn load(&self) -> Result<Vec<RouteConfig>>;
    /// Saves routes to storage
    async fn save(&self, routes: &[RouteConfig]) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_storage() {
        let storage = FileStorage::new("test_routes.json".to_string());
        let routes = vec![RouteConfig::new(
            "GET".to_string(),
            "/test".to_string(),
            200,
            0,
            serde_json::json!({"ok": true}),
        )];

        assert!(storage.save(&routes).await.is_ok());
        let loaded = storage.load().await.unwrap();
        assert_eq!(loaded.len(), 1);

        // Cleanup
        let _ = std::fs::remove_file("test_routes.json");
    }
}
