//! # 🚀 Getting Started
//!
//! ### Add Documents <!-- omit in TOC -->
//!
//! ```rust
//! use meilisearch_sdk::{document::*, client::*};
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
//! #    let index = client.create_index("movies", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap().try_make_index(&client).unwrap();
//!     // An index is where the documents are stored.
//!     let movies = client.index("movies");
//!
//!     // Add some movies in the index. If the index 'movies' does not exist, Meilisearch creates it when you first add the documents.
//!     movies.add_documents(&[
//!         Movie { id: 1, title: String::from("Carol"), genres: vec!["Romance".to_string(), "Drama".to_string()] },
//!         Movie { id: 2, title: String::from("Wonder Woman"), genres: vec!["Action".to_string(), "Adventure".to_string()] },
//!         Movie { id: 3, title: String::from("Life of Pi"), genres: vec!["Adventure".to_string(), "Drama".to_string()] },
//!         Movie { id: 4, title: String::from("Mad Max"), genres: vec!["Adventure".to_string(), "Science Fiction".to_string()] },
//!         Movie { id: 5, title: String::from("Moana"), genres: vec!["Fantasy".to_string(), "Action".to_string()] },
//!         Movie { id: 6, title: String::from("Philadelphia"), genres: vec!["Drama".to_string()] },
//!     ], Some("id")).await.unwrap();
//! #   index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
//! })}
//! ```
//!
//! ### Basic Search <!-- omit in TOC -->
//!
//! ```rust
//! # use meilisearch_sdk::{document::*, client::*};
//! # use serde::{Serialize, Deserialize};
//! # use futures::executor::block_on;
//! # #[derive(Serialize, Deserialize, Debug)]
//! # struct Movie {
//! #    id: usize,
//! #    title: String,
//! #    genres: Vec<String>,
//! # }
//! # impl Document for Movie {
//! #    type UIDType = usize;
//! #    fn get_uid(&self) -> &Self::UIDType {
//! #        &self.id
//! #    }
//! # }
//! # fn main() { block_on(async move {
//! #    let client = Client::new("http://localhost:7700", "masterKey");
//! #    let movies = client.create_index("movies_2", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap().try_make_index(&client).unwrap();
//! // Meilisearch is typo-tolerant:
//! println!("{:?}", client.index("movies_2").search().with_query("caorl").execute::<Movie>().await.unwrap().hits);
//! # movies.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
//! # })}
//! ```
//!
//! Output:
//! ```text
//! [Movie { id: 1, title: String::from("Carol"), genres: vec!["Romance", "Drama"] }]
//! ```
//!
//! Json output:
//! ```json
//! {
//!   "hits": [{
//!     "id": 1,
//!     "title": "Carol",
//!     "genres": ["Romance", "Drama"]
//!   }],
//!   "offset": 0,
//!   "limit": 10,
//!   "processingTimeMs": 1,
//!   "query": "caorl"
//! }
//! ```
//!
//! ### Custom Search <!-- omit in toc -->
//!
//! ```rust
//! # use meilisearch_sdk::{document::*, client::*, search::*};
//! # use serde::{Serialize, Deserialize};
//! # use futures::executor::block_on;
//! # #[derive(Serialize, Deserialize, Debug)]
//! # struct Movie {
//! #    id: usize,
//! #    title: String,
//! #    genres: Vec<String>,
//! # }
//! # impl Document for Movie {
//! #    type UIDType = usize;
//! #    fn get_uid(&self) -> &Self::UIDType {
//! #        &self.id
//! #    }
//! # }
//! # fn main() { block_on(async move {
//! #    let client = Client::new("http://localhost:7700", "masterKey");
//! #    let movies = client.create_index("movies_3", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap().try_make_index(&client).unwrap();
//! let search_result = client.index("movies_3")
//!   .search()
//!   .with_query("phil")
//!   .with_attributes_to_highlight(Selectors::Some(&["*"]))
//!   .execute::<Movie>()
//!   .await
//!   .unwrap();
//! println!("{:?}", search_result.hits);
//! # movies.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
//! # })}
//! ```
//!
//! Json output:
//! ```json
//! {
//!     "hits": [
//!         {
//!             "id": 6,
//!             "title": "Philadelphia",
//!             "_formatted": {
//!                 "id": 6,
//!                 "title": "<em>Phil</em>adelphia",
//!                 "genre": ["Drama"]
//!             }
//!         }
//!     ],
//!     "offset": 0,
//!     "limit": 20,
//!     "processingTimeMs": 0,
//!     "query": "phil"
//! }
//! ```
//!
//! ### Custom Search With Filters <!-- omit in TOC -->
//!
//! If you want to enable filtering, you must add your attributes to the `filterableAttributes`
//! index setting.
//!
//! ```
//! # use meilisearch_sdk::{client::*};
//! # use serde::{Serialize, Deserialize};
//! # use futures::executor::block_on;
//! # fn main() { block_on(async move {
//! #    let client = Client::new("http://localhost:7700", "masterKey");
//! #    let movies = client.create_index("movies_4", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap().try_make_index(&client).unwrap();
//! let filterable_attributes = [
//!     "id",
//!     "genres",
//! ];
//! client.index("movies_4").set_filterable_attributes(&filterable_attributes).await.unwrap();
//! # movies.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
//! # })}
//! ```
//!
//! You only need to perform this operation once.
//!
//! Note that Meilisearch will rebuild your index whenever you update `filterableAttributes`.
//! Depending on the size of your dataset, this might take time. You can track the whole process
//! using the [update
//! status](https://docs.meilisearch.com/reference/api/updates.html#get-an-update-status).
//!
//! Then, you can perform the search:
//!
//! ```
//! # use meilisearch_sdk::{document::*, client::*, search::*};
//! # use serde::{Serialize, Deserialize};
//! # use futures::executor::block_on;
//! # #[derive(Serialize, Deserialize, Debug)]
//! # struct Movie {
//! #    id: usize,
//! #    title: String,
//! #    genres: Vec<String>,
//! # }
//! # impl Document for Movie {
//! #    type UIDType = usize;
//! #    fn get_uid(&self) -> &Self::UIDType {
//! #        &self.id
//! #    }
//! # }
//! # fn main() { block_on(async move {
//! # let client = Client::new("http://localhost:7700", "masterKey");
//! # let movies = client.create_index("movies_5", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap().try_make_index(&client).unwrap();
//! # let filterable_attributes = [
//! #     "id",
//! #    "genres"
//! # ];
//! # movies.set_filterable_attributes(&filterable_attributes).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
//! # movies.add_documents(&[
//! #     Movie { id: 1, title: String::from("Carol"), genres: vec!["Romance".to_string(), "Drama".to_string()] },
//! #     Movie { id: 2, title: String::from("Wonder Woman"), genres: vec!["Action".to_string(), "Adventure".to_string()] },
//! #     Movie { id: 3, title: String::from("Life of Pi"), genres: vec!["Adventure".to_string(), "Drama".to_string()] },
//! #     Movie { id: 4, title: String::from("Mad Max"), genres: vec!["Adventure".to_string(), "Science Fiction".to_string()] },
//! #     Movie { id: 5, title: String::from("Moana"), genres: vec!["Fantasy".to_string(), "Action".to_string()] },
//! #     Movie { id: 6, title: String::from("Philadelphia"), genres: vec!["Drama".to_string()] },
//! # ], Some("id")).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
//! let search_result = client.index("movies_5")
//!   .search()
//!   .with_query("wonder")
//!   .with_filter("id > 1 AND genres = Action")
//!   .execute::<Movie>()
//!   .await
//!   .unwrap();
//! println!("{:?}", search_result.hits);
//! # movies.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
//! # })}
//! ```
//!
//! Json output:
//! ```json
//! {
//!   "hits": [
//!     {
//!       "id": 2,
//!       "title": "Wonder Woman",
//!       "genres": ["Action", "Adventure"]
//!     }
//!   ],
//!   "offset": 0,
//!   "limit": 20,
//!   "nbHits": 1,
//!   "processingTimeMs": 0,
//!   "query": "wonder"
//! }
//! ```

#![warn(clippy::all)]
#![allow(clippy::needless_doctest_main)]

/// Module containing the [client::Client] struct.
pub mod client;
/// Module containing the [document::Document] trait.
pub mod document;
pub mod dumps;
/// Module containing the [errors::Error] struct.
pub mod errors;
/// Module containing the Index struct.
pub mod indexes;
/// Module containing the [key::Key] struct.
pub mod key;
mod request;
/// Module related to search queries and results.
pub mod search;
/// Module containing [settings::Settings].
pub mod settings;
/// Module representing the [tasks::Task]s.
pub mod tasks;

#[cfg(feature = "sync")]
pub(crate) type Rc<T> = std::sync::Arc<T>;
#[cfg(not(feature = "sync"))]
pub(crate) type Rc<T> = std::rc::Rc<T>;
