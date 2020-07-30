//! ```
//! use meilisearch_sdk::{document::*, client::*, search::*};
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
//! #[tokio::main]
//! async fn main() {
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
//!     let query = Query::new("harry pottre");
//!     println!("{:?}", books.search::<Book>(&query).await.unwrap().hits);
//! }
//! ```
//!
//! Output:
//!
//! ```text
//! [Book { book_id: 4, title: "Harry Potter and the Half-Blood Prince" }]
//! ```

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
