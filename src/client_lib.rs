/*
    Module for fetching data from Google BigQuery using CLIENT Library.
*/

// Third-party dependencies
use google_cloud_bigquery::client::{Client, ClientConfig};
use google_cloud_bigquery::http::job::query::QueryRequest;
use google_cloud_bigquery::query::{Iterator, row::Row};
use timed::timed;

// Main authentication function
pub async fn authenticate_client() -> (Client, String) {
    // Create a new CLIENT Library client configuration with authentication
    let (config, project_id) = ClientConfig::new_with_auth().await.unwrap();

    // Print the project ID for verification
    let project_id: String = match project_id {
        // The *new* project_id variable shadows the old one
        Some(pid) => {
            println!("Authenticated with project ID: {}", pid);
            pid
        }
        None => {
            println!("No project ID found in the credentials.");
            "".to_string()
        }
    };

    // Instantiate the google bigquery client
    let client: Client = Client::new(config).await.unwrap();

    // Return the bigquery client and project ID
    (client, project_id)
}

// Function to run a sample query
#[timed("Client Library (HTTP) query execution time")]
pub async fn run_query(query_string: &str, client: &Client, project_id: &str) {
    // Instantite the request object
    let request: QueryRequest = QueryRequest {
        // Actual request string
        query: query_string.to_string(),

        // Apply all the other defaults to the request
        ..Default::default()
    };

    // Run the query and get the iterator
    let mut iter: Iterator<Row> = client.query::<Row>(project_id, request).await.unwrap();

    // Print the first 5 rows
    println!("First 5 rows of query results:");
    let mut count = 0;

    loop {
        match iter.next().await {
            Ok(Some(row)) => {
                count += 1;
                println!("Row {}: ", count);

                // Try to access row data using the column method
                // Let's try to access the first few columns by index
                for i in 0..10 {
                    // Try up to 10 columns
                    if let Ok(value) = row.column::<String>(i) {
                        println!("  Column {}: {}", i, value);
                    } else if let Ok(value) = row.column::<i64>(i) {
                        println!("  Column {}: {}", i, value);
                    } else if let Ok(value) = row.column::<f64>(i) {
                        println!("  Column {}: {}", i, value);
                    } else if let Ok(value) = row.column::<bool>(i) {
                        println!("  Column {}: {}", i, value);
                    } else {
                        // No more columns or unsupported type
                        break;
                    }
                }

                // Stop after 5 rows
                if count >= 5 {
                    break;
                }
            }
            Ok(None) => {
                // No more rows
                break;
            }
            Err(e) => {
                eprintln!("Error reading row: {}", e);
                break;
            }
        }
    }

    // No query results
    if count == 0 {
        println!("No rows returned from the query.");
    }
}
