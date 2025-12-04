// Base dependencies
use std::error::Error;

// Project dependencies
use rust_bigquery::client_lib::{authenticate_client, run_query};
use rust_bigquery::config::Settings;

// Main entry point for the script
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // State the start of the script execution
    println!("Starting the GCP bigquery tests");

    // Load configuration settings
    let _settings: Settings = Settings::new()?;

    // CLIENT Library code ===========

    // HTTP Version

    // Creating the bigquery client
    let (client, project_id) = authenticate_client().await;

    // Fetch the query string from the CLI parameters
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        println!("No query provided via CLI arguments, finishing execution.");
        return Ok(());
    }

    // Run a base query using the client
    run_query(&args[1], &client, &project_id).await;

    // gRPC Version

    // Default HTTP API fetching ===========

    // gRPC API fetching ===========

    // Base return
    Ok(())
}
