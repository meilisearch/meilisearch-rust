<!-- Do NOT update manually the README.md file -->
<!-- Update the README.tpl or src/lib.rs files instead, and run: -->
<!-- sh scripts/update-readme.sh -->

<p align="center">
  <img src="https://res.cloudinary.com/meilisearch/image/upload/v1587402338/SDKs/meilisearch_rust.svg" alt="MeiliSearch-Dotnet" width="200" height="200" />
</p>

<h1 align="center">MeiliSearch Rust SDK</h1>

<h4 align="center">
  <a href="https://github.com/meilisearch/MeiliSearch">MeiliSearch</a> |
  <a href="https://docs.meilisearch.com">Documentation</a> |
  <a href="https://slack.meilisearch.com">Slack</a> |
  <a href="https://roadmap.meilisearch.com/tabs/1-under-consideration">Roadmap</a> |
  <a href="https://www.meilisearch.com">Website</a> |
  <a href="https://docs.meilisearch.com/faq">FAQ</a>
</h4>

<p align="center">
  <a href="https://crates.io/crates/meilisearch-sdk"><img src="https://img.shields.io/crates/v/meilisearch-sdk.svg" alt="crates.io"></a>
  <a href="https://github.com/meilisearch/meilisearch-rust/actions"><img src="https://github.com/meilisearch/meilisearch-rust/workflows/Tests/badge.svg?branch=main" alt="Tests"></a>
  <a href="https://github.com/meilisearch/meilisearch-rust/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-informational" alt="License"></a>
  <a href="https://github.com/meilisearch/MeiliSearch/discussions" alt="Discussions"><img src="https://img.shields.io/badge/github-discussions-red" /></a>
  <a href="https://app.bors.tech/repositories/28502"><img src="https://bors.tech/images/badge_small.svg" alt="Bors enabled"></a>
</p>

<p align="center">⚡ The MeiliSearch API client written for Rust 🦀</p>

**MeiliSearch Rust** is the MeiliSearch API client for Rust developers.

**MeiliSearch** is an open-source search engine. [Discover what MeiliSearch is!](https://github.com/meilisearch/MeiliSearch)

## Table of Contents <!-- omit in TOC -->

- [📖 Documentation](#-documentation)
- [🔧 Installation](#-installation)
- [🚀 Getting Started](#-getting-started)
- [🤖 Compatibility with MeiliSearch](#-compatibility-with-meilisearch)
- [⚙️ Development Workflow and Contributing](#️-development-workflow-and-contributing)

## 📖 Documentation

See our [Documentation](https://docs.meilisearch.com/learn/tutorials/getting_started.html) or our [API References](https://docs.meilisearch.com/reference/api/).

## 🔧 Installation

To use `meilisearch-sdk`, add this to your `Cargo.toml`:

```toml
[dependencies]
meilisearch-sdk = "0.7.0"
```

The following optional dependencies may also be useful:

```toml
futures = "0.3" # To be able to block on async functions if you are not using an async runtime
serde = { version = "1.0", features = ["derive"] }
```

This crate is `async` but you can choose to use an async runtime like [tokio](https://crates.io/crates/tokio) or just [block on futures](https://docs.rs/futures/latest/futures/executor/fn.block_on.html).

Using this crate is possible without [serde](https://crates.io/crates/serde), but a lot of features require serde.

### Run a MeiliSearch Instance <!-- omit in TOC -->

This crate requires a MeiliSearch server to run.

There are many easy ways to [download and run a MeiliSearch instance](https://docs.meilisearch.com/reference/features/installation.html#download-and-launch).

For example, if you use Docker:
```bash
docker pull getmeili/meilisearch:latest # Fetch the latest version of MeiliSearch image from Docker Hub
docker run -it --rm -p 7700:7700 getmeili/meilisearch:latest ./meilisearch --master-key=masterKey
```

NB: you can also download MeiliSearch from **Homebrew** or **APT**.

## 🚀 Getting Started

```rust
use meilisearch_sdk::{document::*, client::*, search::*};
use serde::{Serialize, Deserialize};
use futures::executor::block_on;

#[derive(Serialize, Deserialize, Debug)]
struct Book {
    book_id: usize,
    title: String,
}

// That trait is required to make a struct usable by an index
impl Document for Book {
    type UIDType = usize;

    fn get_uid(&self) -> &Self::UIDType {
        &self.book_id
    }
}

fn main() { block_on(async move {
    // Create a client (without sending any request so that can't fail)
    let client = Client::new("http://localhost:7700", "masterKey");

    // Get the index called "books"
    let books = client.get_or_create("books").await.unwrap();

    // Add some books in the index
    books.add_documents(&[
        Book{book_id: 123,  title: String::from("Pride and Prejudice")},
        Book{book_id: 456,  title: String::from("Le Petit Prince")},
        Book{book_id: 1,    title: String::from("Alice In Wonderland")},
        Book{book_id: 1344, title: String::from("The Hobbit")},
        Book{book_id: 4,    title: String::from("Harry Potter and the Half-Blood Prince")},
        Book{book_id: 42,   title: String::from("The Hitchhiker's Guide to the Galaxy")},
    ], Some("book_id")).await.unwrap();

    // Query books (note that there is a typo)
    println!("{:?}", books.search().with_query("harry pottre").execute::<Book>().await.unwrap().hits);
})}
```

Output:

```
[Book { book_id: 4, title: "Harry Potter and the Half-Blood Prince" }]
```

### 🌐 Running in the Browser with WASM <!-- omit in TOC -->

This crate fully supports WASM.

The only difference between the WASM and the native version is that the native version has one more variant (`Error::Http`) in the Error enum. That should not matter so much but we could add this variant in WASM too.

However, making a program intended to run in a web browser requires a **very** different design than a CLI program. To see an example of a simple Rust web app using MeiliSearch, see the [our demo](./examples/web_app).

WARNING: `meilisearch-sdk` will panic if no Window is available (ex: Web extension).

## 🤖 Compatibility with MeiliSearch

This package only guarantees the compatibility with the [version v0.20.0 of MeiliSearch](https://github.com/meilisearch/MeiliSearch/releases/tag/v0.20.0).

## ⚙️ Development Workflow and Contributing

Any new contribution is more than welcome in this project!

If you want to know more about the development workflow or want to contribute, please visit our [contributing guidelines](/CONTRIBUTING.md) for detailed instructions!

<hr>

**MeiliSearch** provides and maintains many **SDKs and Integration tools** like this one. We want to provide everyone with an **amazing search experience for any kind of project**. If you want to contribute, make suggestions, or just know what's going on right now, visit us in the [integration-guides](https://github.com/meilisearch/integration-guides) repository.
