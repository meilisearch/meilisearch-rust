//! # 🔧 Installation
//!
//! To use `meilisearch-sdk`, add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! meilisearch-sdk = "0.10.2"
//! ```
//!
//! The following optional dependencies may also be useful:
//!
//! ```toml
//! futures = "0.3" # To be able to block on async functions if you are not using an async runtime
//! serde = { version = "1.0", features = ["derive"] }
//! ```
//!
//! This crate is `async` but you can choose to use an async runtime like [tokio](https://crates.io/crates/tokio) or just [block on futures](https://docs.rs/futures/latest/futures/executor/fn.block_on.html).
//! You can enable the `sync` feature to make most structs `Sync`. It may be a bit slower.
//!
//! Using this crate is possible without [serde](https://crates.io/crates/serde), but a lot of features require serde.
//!
//! ## Run a MeiliSearch Instance <!-- omit in TOC -->
//!
//! This crate requires a MeiliSearch server to run.
//!
//! There are many easy ways to [download and run a MeiliSearch instance](https://docs.meilisearch.com/reference/features/installation.html#download-and-launch).
//!
//! For example, if you use Docker:
//! ```bash
//! docker pull getmeili/meilisearch:latest # Fetch the latest version of MeiliSearch image from Docker Hub
//! docker run -it --rm -p 7700:7700 getmeili/meilisearch:latest ./meilisearch --master-key=masterKey
//! ```
//!
//! NB: you can also download MeiliSearch from **Homebrew** or **APT**.
//!
//! # 🚀 Getting Started
//!
//! ```
//! use meilisearch_sdk::{document::*, client::*, search::*};
//! use serde::{Serialize, Deserialize};
//! use futures::executor::block_on;
//!
//! #[derive(Serialize, Deserialize, Debug)]
//! struct Movie {
//!     id: usize,
//!     title: String,
//!     genres: Vec<String>,
//! }
//!
//! // That trait is required to make a struct usable by an index
//! impl Document for Movie {
//!     type UIDType = usize;
//!
//!     fn get_uid(&self) -> &Self::UIDType {
//!         &self.id
//!     }
//! }
//!
//! fn main() { block_on(async move {
//!     // Create a client (without sending any request so that can't fail)
//!     let client = Client::new("http://localhost:7700", "masterKey");
//!
//!     // Get the index called "movies"
//!     let movies = client.get_or_create("movies").await.unwrap();
//!
//!     // Add some movies in the index
//!     movies.add_documents(&[
//!         Movie{id: 1, title: String::from("Carol"), genres: vec!["Romance".to_string(), "Drama".to_string()]},
//!         Movie{id: 2, title: String::from("Wonder Woman"), genres: vec!["Action".to_string(), "Adventure".to_string()]},
//!         Movie{id: 3, title: String::from("Life of Pi"), genres: vec!["Adventure".to_string(), "Drama".to_string()]},
//!         Movie{id: 4, title: String::from("Mad Max"), genres: vec!["Adventure".to_string(), "Science Fiction".to_string()]},
//!         Movie{id: 5, title: String::from("Moana"), genres: vec!["Fantasy".to_string(), "Action".to_string()]},
//!         Movie{id: 6, title: String::from("Philadelphia"), genres: vec!["Drama".to_string()]},
//!     ], Some("id")).await.unwrap();
//!
//!     // Query movies (note that there is a typo)
//!     println!("{:?}", movies.search().with_query("carol").execute::<Movie>().await.unwrap().hits);
//! })}
//! ```
//!
//! Output:
//!
//! ```text
//! [Movie{id: 1, title: String::from("Carol"), genres: vec!["Romance", "Drama"]}]
//! ```
//!
//! ## 🌐 Running in the Browser with WASM <!-- omit in TOC -->
//!
//! This crate fully supports WASM.
//!
//! The only difference between the WASM and the native version is that the native version has one more variant (`Error::Http`) in the Error enum. That should not matter so much but we could add this variant in WASM too.
//!
//! However, making a program intended to run in a web browser requires a **very** different design than a CLI program. To see an example of a simple Rust web app using MeiliSearch, see the [our demo](./examples/web_app).
//!
//! WARNING: `meilisearch-sdk` will panic if no Window is available (ex: Web extension).

#![warn(clippy::all)]
#![allow(clippy::needless_doctest_main)]

/// Module containing the Client struct.
pub mod client;
/// Module containing the Document trait.
pub mod document;
pub mod dumps;
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

#[cfg(feature = "sync")]
pub(crate) type Rc<T> = std::sync::Arc<T>;
#[cfg(not(feature = "sync"))]
pub(crate) type Rc<T> = std::rc::Rc<T>;
