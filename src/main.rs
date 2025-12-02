// Base dependencies
use std::error::Error;

// Project dependencies
use rust_bigquery::config::Settings;

// Main entry point for the script
fn main() -> Result<(), Box<dyn Error>> {
    // State the start of the script execution
    println!("Starting the GCP bigquery tests");

    // Load configuration settings
    let settings: Settings = Settings::new()?;

    // Base return
    Ok(())
}
