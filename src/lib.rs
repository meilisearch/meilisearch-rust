//! # 🚀 Getting started
//!
//! ### Add Documents <!-- omit in TOC -->
//!
//! ```
//! use meilisearch_sdk::client::*;
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
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() {
//! #   let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
//! #   let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
//!     // Create a client (without sending any request so that can't fail)
//!     let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
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
//! }
//! ```
//!
//! With the `uid`, you can check the status (`enqueued`, `canceled`, `processing`, `succeeded` or `failed`) of your documents addition using the [task](https://www.meilisearch.com/docs/reference/api/tasks#get-task).
//!
//! ### Basic Search <!-- omit in TOC -->
//!
//! ```
//! # use meilisearch_sdk::client::*;
//! # use serde::{Serialize, Deserialize};
//! # #[derive(Serialize, Deserialize, Debug)]
//! # struct Movie {
//! #    id: usize,
//! #    title: String,
//! #    genres: Vec<String>,
//! # }
//! # fn main() { tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
//! #    let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
//! #    let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
//! #    let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
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
//! ```
//! # use meilisearch_sdk::{client::*, search::*};
//! # use serde::{Serialize, Deserialize};
//! # #[derive(Serialize, Deserialize, Debug)]
//! # struct Movie {
//! #    id: usize,
//! #    title: String,
//! #    genres: Vec<String>,
//! # }
//! # fn main() { tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
//! #   let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
//! #   let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
//! #    let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
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
//! # fn main() { tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
//! #    let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
//! #    let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
//! #    let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
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
//! Note that Meilisearch will rebuild your index whenever you update `filterableAttributes`. Depending on the size of your dataset, this might take time. You can track the process using the [tasks](https://www.meilisearch.com/docs/reference/api/tasks#get-task).
//!
//! Then, you can perform the search:
//!
//! ```
//! # use meilisearch_sdk::{client::*, search::*};
//! # use serde::{Serialize, Deserialize};
//! # #[derive(Serialize, Deserialize, Debug)]
//! # struct Movie {
//! #    id: usize,
//! #    title: String,
//! #    genres: Vec<String>,
//! # }
//! # fn main() { tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
//! # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
//! # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
//! # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
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
//!   "estimatedTotalHits": 1,
//!   "processingTimeMs": 0,
//!   "query": "wonder"
//! }
//! ```
//!
//! ### Customize the `HttpClient` <!-- omit in TOC -->
//!
//! By default, the SDK uses [`reqwest`](https://docs.rs/reqwest/latest/reqwest/) to make http calls.
//! The SDK lets you customize the http client by implementing the `HttpClient` trait yourself and
//! initializing the `Client` with the `new_with_client` method.
//! You may be interested by the `futures-unsend` feature which lets you specify a non-Send http client.
//!
//! ### Wasm support <!-- omit in TOC -->
//!
//! The SDK supports wasm through reqwest. You'll need to enable the `futures-unsend` feature while importing it, though.
#![warn(clippy::all)]
#![allow(clippy::needless_doctest_main)]

/// Module containing the [`Client`](client::Client) struct.
pub mod client;
/// Module representing the [documents] structures.
pub mod documents;
/// Module containing the [dumps] trait.
pub mod dumps;
/// Module containing the [`errors::Error`] struct.
pub mod errors;
/// Module related to runtime and instance features.
pub mod features;
/// Module containing the Index struct.
pub mod indexes;
/// Module containing the [`Key`](key::Key) struct.
pub mod key;
pub mod request;
/// Module related to search queries and results.
pub mod search;
/// Module containing [`Settings`](settings::Settings).
pub mod settings;
/// Module containing the [snapshots](snapshots::create_snapshot)-feature.
pub mod snapshots;
/// Module representing the [`TaskInfo`](task_info::TaskInfo)s.
pub mod task_info;
/// Module representing the [`Task`](tasks::Task)s.
pub mod tasks;
/// Module that generates tenant tokens.
#[cfg(not(target_arch = "wasm32"))]
mod tenant_tokens;
/// Module containing utilizes functions.
mod utils;

/// Module related to similar queries and results.
pub mod similar;

#[cfg(feature = "reqwest")]
pub mod reqwest;

#[cfg(feature = "reqwest")]
pub type DefaultHttpClient = reqwest::ReqwestClient;

#[cfg(not(feature = "reqwest"))]
pub type DefaultHttpClient = std::convert::Infallible;

#[cfg(test)]
/// Support for the `IndexConfig` derive proc macro in the crate's tests.
extern crate self as meilisearch_sdk;
/// Can't assume that the user of proc_macro will have access to `async_trait` crate. So exporting the `async-trait` crate from `meilisearch_sdk` in a hidden module.
#[doc(hidden)]
pub mod macro_helper {
    pub use async_trait::async_trait;
}
