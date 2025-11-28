
# Rust GCP Bigquery tests

## Motivation

Small repository for testing communicating with Google Cloud Platform 
Bigquery APIs using HTTP and gRPC protocols. 

The aim is testing the implementation requirements of a complete bespoke 
RUST-based client library for future projects using this cloud provider 
(whose client library for this language is still in early stages of development). As of the time of writing, the main goal is allowing for RUST-based pipelines to perform queries to specific project databases and parsing the results to polars-compatible dataframes.

## Resources

The aim of the repository is to use as few dependencies as possible, while: 

- Creating Structs / Traits for accessing the GCP client endpoints
- Fetching Bigquery resources (Query data and other metadata)
- Allowing for multiple ways of authentication 
- Allowing for the usage of both HTTP and gRPC for benchmark purposes






