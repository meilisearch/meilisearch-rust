//! # Indexes
//!
//! ```
//! # use meilisearch_sdk::{client::*, indexes::*};
//! # #[tokio::main]
//! # async fn main() {
//! # let client = Client::new("http://localhost:7700", "masterKey");
//! // list all indexes
//! let indexes: Vec<Index> = client.list_all_indexes().await.unwrap();
//! # }
//! ```
//! ...
//! ETC...
//! ...

/// Module containing the Client struct.
pub mod client;
/// Module containing the Document trait.
pub mod document;
/// Module containing the Error struct.
pub mod errors;
/// Module containing the Index struct.
pub mod indexes;
/// Module containing objects useful for tracking the progress of async operations.
pub mod progress;
mod request;
/// Module related to search queries and results.
pub mod search;
/// Module containing settings
pub mod settings;
