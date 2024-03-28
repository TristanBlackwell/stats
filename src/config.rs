use dotenv::dotenv;
use serde::Deserialize;
use std::env;

/// The application configuration.
#[derive(Deserialize)]
pub struct Config {
    /// The domain this application is hosted on.
    pub app_url: String,
    /// The port at which the API & UI can be accessed.
    pub service_port: String,
    /// URL of the database.
    pub database_url: String,
    /// Comma separated list of allowed domains to collect events from.
    pub cors_domains: Vec<String>,
    /// Maximum buffer of incoming events to queue. Events exceeding the
    /// buffer will be dropped until space is released.
    pub processing_batch_size: usize,
    /// Whether the app is running in development mode.
    pub is_development: bool,
}

// TODO: potentially replace this with arctix settings later
impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        let service_port = Self::get_env("SERVICE_PORT", "5775");

        Config {
            app_url: Self::get_env("APP_URL", &format!("0.0.0.0:{service_port}")),
            service_port,
            database_url: Self::get_env("DATABASE_URL", "/data/stats.sqlite"),
            cors_domains: Self::get_env_list("CORS_DOMAINS", ""),
            processing_batch_size: Self::get_env_usize("PROCESSING_BATCH_SIZE", 4),
            is_development: Self::get_env_bool("IS_DEVELOPMENT", false),
        }
    }

    fn get_env(key: &str, default: &str) -> String {
        env::var(key).unwrap_or_else(|_| default.to_string())
    }

    /// Reads an environment variable as a comma seperated list returning
    /// a Vector of each element.
    fn get_env_list(key: &str, default: &str) -> Vec<String> {
        env::var(key)
            .unwrap_or_else(|_| default.to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    /// Reads an environment variable and converts it to a `usize`.
    fn get_env_usize(key: &str, default: usize) -> usize {
        env::var(key)
            .unwrap_or_else(|_| default.to_string())
            .parse()
            .expect(&format!("Failed to parse {} as a number.", key))
    }

    /// Reads an environment variable and converts it to a `bool`.
    fn get_env_bool(key: &str, default: bool) -> bool {
        env::var(key)
            .unwrap_or_else(|_| default.to_string())
            .parse()
            .expect(&format!("Failed to parse {} as a boolean", key))
    }
}
