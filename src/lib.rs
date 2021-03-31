//! # 🔧 Installation
//!
//! To use `meilisearch-sdk`, add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! meilisearch-sdk = "0.7.0"
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
//! struct Book {
//!     book_id: usize,
//!     title: String,
//! }
//!
//! // That trait is required to make a struct usable by an index
//! impl Document for Book {
//!     type UIDType = usize;
//!
//!     fn get_uid(&self) -> &Self::UIDType {
//!         &self.book_id
//!     }
//! }
//!
//! fn main() { block_on(async move {
//!     // Create a client (without sending any request so that can't fail)
//!     let client = Client::new("http://localhost:7700", "masterKey");
//!
//!     // Get the index called "books"
//!     let books = client.get_or_create("books").await.unwrap();
//!
//!     // Add some books in the index
//!     books.add_documents(&[
//!         Book{book_id: 123,  title: String::from("Pride and Prejudice")},
//!         Book{book_id: 456,  title: String::from("Le Petit Prince")},
//!         Book{book_id: 1,    title: String::from("Alice In Wonderland")},
//!         Book{book_id: 1344, title: String::from("The Hobbit")},
//!         Book{book_id: 4,    title: String::from("Harry Potter and the Half-Blood Prince")},
//!         Book{book_id: 42,   title: String::from("The Hitchhiker's Guide to the Galaxy")},
//!     ], Some("book_id")).await.unwrap();
//!
//!     // Query books (note that there is a typo)
//!     println!("{:?}", books.search().with_query("harry pottre").execute::<Book>().await.unwrap().hits);
//! })}
//! ```
//!
//! Output:
//!
//! ```text
//! [Book { book_id: 4, title: "Harry Potter and the Half-Blood Prince" }]
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
