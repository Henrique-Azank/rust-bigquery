// Base dependencies
use std::path::PathBuf;

// Third-party dependencies
use serde::Deserialize;

// Google credentials structure
#[derive(Debug, Deserialize, Clone)]
pub struct GoogleCredentials {
    // Public client email
    pub client_email: String,

    // Service account key path
    pub private_key_path: PathBuf,
}

// BigQuery configuration settings
#[derive(Debug, Deserialize, Clone)]
pub struct BigQueryConfig {
    // Base service URL for bigquery
    pub main_url: String,
}

// Main application name
#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub application_name: String,
    pub debug_mode: bool,

    // Main bigquery data settings
    pub database: BigQueryConfig,
}
