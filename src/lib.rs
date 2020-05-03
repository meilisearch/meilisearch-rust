//! MeiliSearch Rust is a client for [MeiliSearch](https://www.meilisearch.com/) written in Rust.
//! [MeiliSearch](https://www.meilisearch.com/) is a powerful, fast, open-source, easy to use and deploy search engine.
//! Both searching and indexing are highly customizable.
//! Features such as typo-tolerance, filters, and synonyms are provided out-of-the-box.
//! 
//! ## Table of Contents <!-- omit in toc -->
//! - [🔧 Installation](#-installation)
//! - [🚀 Getting started](#-getting-started)
//! - [🤖 Compatibility with MeiliSearch](#-compatibility-with-meilisearch)
//! 
//! # 🔧 Installation
//! 
//! This crate requires a MeiliSearch server to run. See [here](https://docs.meilisearch.com/guides/advanced_guides/installation.html#download-and-launch) to install and run MeiliSearch.  
//!   
//! Then, put `meilisearch-sdk = "0.1"` in your Cargo.toml, as usual.  
//!   
//! Using this crate is possible without [serde](https://crates.io/crates/serde), but a lot of features require serde.
//! Add `serde = {version="1.0", features=["derive"]}` in your Cargo.toml.
//! 
//! # 🚀 Getting Started
//! 
//! Here is a quickstart for a search request (please follow the [installation](#-installation) steps before)
//! 
//! ```rust
//! use meilisearch_sdk::{document::*, indexes::*, client::*, search::*};
//! use serde::{Serialize, Deserialize};
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
//! // Create a client (without sending any request so that can't fail)
//! let client = Client::new("http://localhost:7700", "");
//! 
//! // Get the index called "books"
//! let mut books = client.get_or_create("books").unwrap();
//! 
//! // Add some books in the index
//! books.add_documents(vec![
//!     Book{book_id: 123,  title: String::from("Pride and Prejudice")},
//!     Book{book_id: 456,  title: String::from("Le Petit Prince")},
//!     Book{book_id: 1,    title: String::from("Alice In Wonderland")},
//!     Book{book_id: 1344, title: String::from("The Hobbit")},
//!     Book{book_id: 4,    title: String::from("Harry Potter and the Half-Blood Prince")},
//!     Book{book_id: 42,   title: String::from("The Hitchhiker's Guide to the Galaxy")},
//! ], Some("book_id")).unwrap();
//! 
//! // Query books (note that there is a typo)
//! let query = Query::new("harry pottre");
//! println!("{:?}", books.search::<Book>(&query).unwrap().hits);
//! ```
//! 
//! Output:
//! 
//! ```ignore
//! [Book { book_id: 4, title: "Harry Potter and the Half-Blood Prince" }]
//! ```
//! 
//! # 🤖 Compatibility with MeiliSearch
//! 
//! This crate is currently supporting MeiliSearch v10.0 and will be maintained.
//! 
//! # Running the tests
//! 
//! All the tests are documentation tests.
//! Since they are all making operations on the MeiliSearch server, running all the tests simultaneously would cause panics.
//! To run the tests one by one, run `cargo test -- --test-threads=1`.

#![warn(missing_docs)]

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