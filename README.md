
# Rust GCP Bigquery tests

## Motivation

Small repository for testing communicating with Google Cloud Platform
Bigquery APIs using HTTP, gRPC protocols and the newly published GCP SDKs for Rust.

As of the time of writing, the main goal is allowing for RUST-based pipelines to perform
queries to specific project databases and parsing the results to polars-compatible dataframes.

## Resources

The aim of the repository is to use as few dependencies as possible, while:

- Creating Structs / Traits for accessing the GCP client endpoints
- Fetching Bigquery resources (Query data and other metadata)
- Allowing for multiple ways of authentication
- Allowing for the usage of both HTTP and gRPC for benchmark purposes
