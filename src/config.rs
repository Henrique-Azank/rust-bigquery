// Base dependencies
use std::path::PathBuf;

// Third-party dependencies
use config::{Config, ConfigError, Environment, File};
use dotenv::dotenv;
use serde::Deserialize;

// App information structure
#[derive(Debug, Deserialize, Clone, Default)]
pub struct AppInfo {
    // Application name
    #[serde(default)]
    pub name: String,

    // Debug mode flag
    #[serde(default)]
    pub debug_mode: bool,
}

// Google credentials structure
#[derive(Debug, Deserialize, Clone, Default)]
pub struct GoogleCredentials {
    // Public client email
    #[serde(default)]
    pub client_email: String,

    // Email password
    #[serde(default)]
    pub password: String,

    // Service account key path
    #[serde(default)]
    pub private_key_path: PathBuf,
}

// BigQuery configuration settings
#[derive(Debug, Deserialize, Clone, Default)]
pub struct BigQueryConfig {
    // Base service URL for bigquery
    #[serde(default)]
    pub main_url: String,
}

// Main application name
#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    // Application information settings
    #[serde(default)]
    pub application: AppInfo,

    // Main bigquery data settings
    #[serde(default)]
    pub bigquery: BigQueryConfig,

    // Google credentials settings
    #[serde(default)]
    pub google_credentials: GoogleCredentials,
}

// Implement the new method for Settings
impl Settings {
    // Load configuration settings from various sources
    pub fn new() -> Result<Self, ConfigError> {
        // Load the environment variables from a .env file
        dotenv().ok();

        // Instantiate the configuration builder
        let config = Config::builder()
            // Layer 1: Start with a default configuration file (Config.toml)
            .add_source(File::with_name("Config").required(false))
            // Layer 2: Environment variables with APP prefix and double underscore separator
            .add_source(
                Environment::with_prefix("RBQ")
                    .separator("__")
                    .try_parsing(true),
            )
            .build()?;

        // Build and deserialize into the Settings struct
        config.try_deserialize()
    }
}
