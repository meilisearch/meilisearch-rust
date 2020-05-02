#![warn(missing_docs)]

//! TODO top level doc

/// Module containing the Client struct.
pub mod client;
/// Module containing the Document struct and Documentable trait.
pub mod documents;
/// Module containing the Error struct.
pub mod errors;
/// Module containing the Index struct.
pub mod indexes;
/// Module containing the useful objects for tracking progress of async operations.
pub mod progress;
mod request;
/// Module related to search queries and results.
pub mod search;
