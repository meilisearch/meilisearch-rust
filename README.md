<!-- Do NOT update manually the README.md file -->
<!-- Update the README.tpl or src/lib.rs files instead, and run: -->
<!-- sh scripts/update-readme.sh -->

<p align="center">
  <img src="https://raw.githubusercontent.com/meilisearch/integration-guides/main/assets/logos/meilisearch_rust.svg" alt="Meilisearch-Rust" width="200" height="200" />
</p>

<h1 align="center">Meilisearch Rust SDK</h1>

<h4 align="center">
  <a href="https://github.com/meilisearch/meilisearch">Meilisearch</a> |
  <a href="https://www.meilisearch.com/cloud?utm_campaign=oss&utm_source=github&utm_medium=meilisearch-rust">Meilisearch Cloud</a> |
  <a href="https://www.meilisearch.com/docs">Documentation</a> |
  <a href="https://discord.meilisearch.com">Discord</a> |
  <a href="https://roadmap.meilisearch.com/tabs/1-under-consideration">Roadmap</a> |
  <a href="https://www.meilisearch.com">Website</a> |
  <a href="https://www.meilisearch.com/docs/faq">FAQ</a>
</h4>

<p align="center">
  <a href="https://crates.io/crates/meilisearch-sdk"><img src="https://img.shields.io/crates/v/meilisearch-sdk.svg" alt="crates.io"></a>
  <a href="https://github.com/meilisearch/meilisearch-rust/actions"><img src="https://github.com/meilisearch/meilisearch-rust/workflows/Tests/badge.svg?branch=main" alt="Tests"></a>
  <a href="https://github.com/meilisearch/meilisearch-rust/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-informational" alt="License"></a>
  <a href="https://github.com/meilisearch/meilisearch/discussions" alt="Discussions"><img src="https://img.shields.io/badge/github-discussions-red" /></a>
  <a href="https://ms-bors.herokuapp.com/repositories/62"><img src="https://bors.tech/images/badge_small.svg" alt="Bors enabled"></a>
</p>

<p align="center">⚡ The Meilisearch API client written for Rust 🦀</p>

**Meilisearch Rust** is the Meilisearch API client for Rust developers.

**Meilisearch** is an open-source search engine. [Learn more about Meilisearch.](https://github.com/meilisearch/meilisearch)

## Table of Contents <!-- omit in TOC -->

- [📖 Documentation](#-documentation)
- [🔧 Installation](#-installation)
- [🚀 Getting started](#-getting-started)
- [🌐 Running in the Browser with WASM](#-running-in-the-browser-with-wasm)
- [🤖 Compatibility with Meilisearch](#-compatibility-with-meilisearch)
- [⚙️ Contributing](#️-contributing)

## 📖 Documentation

This readme contains all the documentation you need to start using this Meilisearch SDK.

For general information on how to use Meilisearch—such as our API reference, tutorials, guides, and in-depth articles—refer to our [main documentation website](https://www.meilisearch.com/docs).

## 🔧 Installation

To use `meilisearch-sdk`, add this to your `Cargo.toml`:

```toml
[dependencies]
meilisearch-sdk = "0.28.0"
```

The following optional dependencies may also be useful:

```toml
futures = "0.3" # To be able to block on async functions if you are not using an async runtime
serde = { version = "1.0", features = ["derive"] }
```

This crate is `async` but you can choose to use an async runtime like [tokio](https://crates.io/crates/tokio) or just [block on futures](https://docs.rs/futures/latest/futures/executor/fn.block_on.html).
You can enable the `sync` feature to make most structs `Sync`. It may be a bit slower.

Using this crate is possible without [serde](https://crates.io/crates/serde), but a lot of features require serde.

### Run Meilisearch <!-- omit in toc -->

⚡️ **Launch, scale, and streamline in minutes with Meilisearch Cloud**—no maintenance, no commitment, cancel anytime. [Try it free now](https://cloud.meilisearch.com/login?utm_campaign=oss&utm_source=github&utm_medium=meilisearch-rust).

🪨  Prefer to self-host? [Download and deploy](https://www.meilisearch.com/docs/learn/self_hosted/getting_started_with_self_hosted_meilisearch?utm_campaign=oss&utm_source=github&utm_medium=meilisearch-rust) our fast, open-source search engine on your own infrastructure.

## 🚀 Getting started

#### Add Documents <!-- omit in TOC -->

```rust
use meilisearch_sdk::client::*;
use serde::{Serialize, Deserialize};
use futures::executor::block_on;

#[derive(Serialize, Deserialize, Debug)]
struct Movie {
    id: usize,
    title: String,
    genres: Vec<String>,
}


#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Create a client (without sending any request so that can't fail)
    let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();

    // An index is where the documents are stored.
    let movies = client.index("movies");

    // Add some movies in the index. If the index 'movies' does not exist, Meilisearch creates it when you first add the documents.
    movies.add_documents(&[
        Movie { id: 1, title: String::from("Carol"), genres: vec!["Romance".to_string(), "Drama".to_string()] },
        Movie { id: 2, title: String::from("Wonder Woman"), genres: vec!["Action".to_string(), "Adventure".to_string()] },
        Movie { id: 3, title: String::from("Life of Pi"), genres: vec!["Adventure".to_string(), "Drama".to_string()] },
        Movie { id: 4, title: String::from("Mad Max"), genres: vec!["Adventure".to_string(), "Science Fiction".to_string()] },
        Movie { id: 5, title: String::from("Moana"), genres: vec!["Fantasy".to_string(), "Action".to_string()] },
        Movie { id: 6, title: String::from("Philadelphia"), genres: vec!["Drama".to_string()] },
    ], Some("id")).await.unwrap();
}
```

With the `uid`, you can check the status (`enqueued`, `canceled`, `processing`, `succeeded` or `failed`) of your documents addition using the [task](https://www.meilisearch.com/docs/reference/api/tasks#get-task).

#### Basic Search <!-- omit in TOC -->

```rust
// Meilisearch is typo-tolerant:
println!("{:?}", client.index("movies_2").search().with_query("caorl").execute::<Movie>().await.unwrap().hits);
```

Output:
```
[Movie { id: 1, title: String::from("Carol"), genres: vec!["Romance", "Drama"] }]
```

Json output:
```json
{
  "hits": [{
    "id": 1,
    "title": "Carol",
    "genres": ["Romance", "Drama"]
  }],
  "offset": 0,
  "limit": 10,
  "processingTimeMs": 1,
  "query": "caorl"
}
```

#### Custom Search <!-- omit in toc -->

```rust
let search_result = client.index("movies_3")
  .search()
  .with_query("phil")
  .with_attributes_to_highlight(Selectors::Some(&["*"]))
  .execute::<Movie>()
  .await
  .unwrap();
println!("{:?}", search_result.hits);
```

Json output:
```json
{
    "hits": [
        {
            "id": 6,
            "title": "Philadelphia",
            "_formatted": {
                "id": 6,
                "title": "<em>Phil</em>adelphia",
                "genre": ["Drama"]
            }
        }
    ],
    "offset": 0,
    "limit": 20,
    "processingTimeMs": 0,
    "query": "phil"
}
```

#### Custom Search With Filters <!-- omit in TOC -->

If you want to enable filtering, you must add your attributes to the `filterableAttributes`
index setting.

```rust
let filterable_attributes = [
    "id",
    "genres",
];
client.index("movies_4").set_filterable_attributes(&filterable_attributes).await.unwrap();
```

You only need to perform this operation once.

Note that Meilisearch will rebuild your index whenever you update `filterableAttributes`. Depending on the size of your dataset, this might take time. You can track the process using the [tasks](https://www.meilisearch.com/docs/reference/api/tasks#get-task).

Then, you can perform the search:

```rust
let search_result = client.index("movies_5")
  .search()
  .with_query("wonder")
  .with_filter("id > 1 AND genres = Action")
  .execute::<Movie>()
  .await
  .unwrap();
println!("{:?}", search_result.hits);
```

Json output:
```json
{
  "hits": [
    {
      "id": 2,
      "title": "Wonder Woman",
      "genres": ["Action", "Adventure"]
    }
  ],
  "offset": 0,
  "limit": 20,
  "estimatedTotalHits": 1,
  "processingTimeMs": 0,
  "query": "wonder"
}
```

#### Customize the `HttpClient` <!-- omit in TOC -->

By default, the SDK uses [`reqwest`](https://docs.rs/reqwest/latest/reqwest/) to make http calls.
The SDK lets you customize the http client by implementing the `HttpClient` trait yourself and
initializing the `Client` with the `new_with_client` method.
You may be interested by the `futures-unsend` feature which lets you specify a non-Send http client.

#### Wasm support <!-- omit in TOC -->

The SDK supports wasm through reqwest. You'll need to enable the `futures-unsend` feature while importing it, though.

## 🌐 Running in the Browser with WASM <!-- omit in TOC -->

This crate fully supports WASM.

The only difference between the WASM and the native version is that the native version has one more variant (`Error::Http`) in the Error enum. That should not matter so much but we could add this variant in WASM too.

However, making a program intended to run in a web browser requires a **very** different design than a CLI program. To see an example of a simple Rust web app using Meilisearch, see the [our demo](./examples/web_app).

WARNING: `meilisearch-sdk` will panic if no Window is available (ex: Web extension).

## 🤖 Compatibility with Meilisearch

This package guarantees compatibility with [version v1.x of Meilisearch](https://github.com/meilisearch/meilisearch/releases/latest), but some features may not be present. Please check the [issues](https://github.com/meilisearch/meilisearch-rust/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22+label%3Aenhancement) for more info.

## ⚙️ Contributing

Any new contribution is more than welcome in this project!

If you want to know more about the development workflow or want to contribute, please visit our [contributing guidelines](/CONTRIBUTING.md) for detailed instructions!

<hr>

**Meilisearch** provides and maintains many **SDKs and Integration tools** like this one. We want to provide everyone with an **amazing search experience for any kind of project**. If you want to contribute, make suggestions, or just know what's going on right now, visit us in the [integration-guides](https://github.com/meilisearch/integration-guides) repository.
