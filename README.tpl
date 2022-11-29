<!-- Do NOT update manually the README.md file -->
<!-- Update the README.tpl or src/lib.rs files instead, and run: -->
<!-- sh scripts/update-readme.sh -->

<p align="center">
  <img src="https://raw.githubusercontent.com/meilisearch/integration-guides/main/assets/logos/meilisearch_rust.svg" alt="Meilisearch-Rust" width="200" height="200" />
</p>

<h1 align="center">Meilisearch Rust SDK</h1>

<h4 align="center">
  <a href="https://github.com/meilisearch/meilisearch">Meilisearch</a> |
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
  <a href="https://github.com/meilisearch/meilisearch/discussions" alt="Discussions"><img src="https://img.shields.io/badge/github-discussions-red" /></a>
  <a href="https://app.bors.tech/repositories/28502"><img src="https://bors.tech/images/badge_small.svg" alt="Bors enabled"></a>
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

For general information on how to use Meilisearch—such as our API reference, tutorials, guides, and in-depth articles—refer to our [main documentation website](https://docs.meilisearch.com/).

## 🔧 Installation

To use `meilisearch-sdk`, add this to your `Cargo.toml`:

```toml
[dependencies]
meilisearch-sdk = "0.21.0"
```

The following optional dependencies may also be useful:

```toml
futures = "0.3" # To be able to block on async functions if you are not using an async runtime
serde = { version = "1.0", features = ["derive"] }
```

This crate is `async` but you can choose to use an async runtime like [tokio](https://crates.io/crates/tokio) or just [block on futures](https://docs.rs/futures/latest/futures/executor/fn.block_on.html).
You can enable the `sync` feature to make most structs `Sync`. It may be a bit slower.

Using this crate is possible without [serde](https://crates.io/crates/serde), but a lot of features require serde.

### Run a Meilisearch Instance <!-- omit in TOC -->

This crate requires a Meilisearch server to run.

There are many easy ways to [download and run a Meilisearch instance](https://docs.meilisearch.com/reference/features/installation.html#download-and-launch).

For example,using the `curl` command in [your Terminal](https://itconnect.uw.edu/learn/workshops/online-tutorials/web-publishing/what-is-a-terminal/):

```bash
# Install Meilisearch
curl -L https://install.meilisearch.com | sh

# Launch Meilisearch
./meilisearch --master-key=masterKey
```

NB: you can also download Meilisearch from **Homebrew** or **APT**.

{{readme}}

## 🌐 Running in the Browser with WASM <!-- omit in TOC -->

This crate fully supports WASM.

The only difference between the WASM and the native version is that the native version has one more variant (`Error::Http`) in the Error enum. That should not matter so much but we could add this variant in WASM too.

However, making a program intended to run in a web browser requires a **very** different design than a CLI program. To see an example of a simple Rust web app using Meilisearch, see the [our demo](./examples/web_app).

WARNING: `meilisearch-sdk` will panic if no Window is available (ex: Web extension).

## 🤖 Compatibility with Meilisearch

This package only guarantees the compatibility with the [version v0.30.0 of Meilisearch](https://github.com/meilisearch/meilisearch/releases/tag/v0.30.0).

## ⚙️ Contributing

Any new contribution is more than welcome in this project!

If you want to know more about the development workflow or want to contribute, please visit our [contributing guidelines](/CONTRIBUTING.md) for detailed instructions!

<hr>

**Meilisearch** provides and maintains many **SDKs and Integration tools** like this one. We want to provide everyone with an **amazing search experience for any kind of project**. If you want to contribute, make suggestions, or just know what's going on right now, visit us in the [integration-guides](https://github.com/meilisearch/integration-guides) repository.
