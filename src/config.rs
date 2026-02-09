/// Application configuration
use std::env;

/// Configuration structure
#[derive(Debug, Clone)]
pub struct Config {
    /// Port on which the server listens
    pub server_port: u16,
    /// Routes storage file
    pub storage_file: String,
    /// Log level (debug, info, warn, error)
    #[allow(dead_code)]
    pub log_level: String,
}

impl Config {
    /// Loads configuration from environment variables or default values
    pub fn from_env() -> Self {
        Config {
            server_port: env::var("MOCKIE_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            storage_file: env::var("MOCKIE_STORAGE").unwrap_or_else(|_| "routes.json".to_string()),
            log_level: env::var("MOCKIE_LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::from_env();
        assert_eq!(config.server_port, 3000);
        assert_eq!(config.storage_file, "routes.json");
    }
}
