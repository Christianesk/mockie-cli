/// JSON file-based storage
use crate::error::Result;
use crate::models::RouteConfig;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::fs;

use super::{RouteStore, Storage};

pub struct FileStorage {
    path: String,
}

impl FileStorage {
    /// Creates a new file-based storage
    pub fn new(path: String) -> Self {
        FileStorage { path }
    }

    /// Loads initial state from file
    pub fn load_initial_state(&self) -> Result<RouteStore> {
        if let Ok(data) = std::fs::read_to_string(&self.path) {
            if let Ok(list) = serde_json::from_str::<Vec<RouteConfig>>(&data) {
                let mut map = HashMap::new();
                for r in list {
                    map.insert((r.method.clone(), r.path.clone()), r);
                }
                return Ok(Arc::new(RwLock::new(map)));
            }
        }
        Ok(Arc::new(RwLock::new(HashMap::new())))
    }

    /// Extracts routes from store and saves them
    #[allow(dead_code)]
    pub async fn save_from_store(&self, routes: &RouteStore) -> Result<()> {
        let list: Vec<_> = {
            let map = routes
                .read()
                .map_err(|e| crate::error::AppError::Internal(format!("RwLock poisoned: {}", e)))?;
            map.values().cloned().collect()
        };

        self.save(&list).await
    }
}

#[async_trait::async_trait]
impl Storage for FileStorage {
    async fn load(&self) -> Result<Vec<RouteConfig>> {
        let data = fs::read_to_string(&self.path)
            .await
            .or_else(|_| Ok::<String, std::io::Error>(String::new()))?;

        if data.is_empty() {
            return Ok(Vec::new());
        }

        serde_json::from_str(&data).map_err(|e| crate::error::AppError::Json(e))
    }

    async fn save(&self, routes: &[RouteConfig]) -> Result<()> {
        let json =
            serde_json::to_string_pretty(routes).map_err(|e| crate::error::AppError::Json(e))?;
        fs::write(&self.path, json)
            .await
            .map_err(|e| crate::error::AppError::Io(e))
    }
}
