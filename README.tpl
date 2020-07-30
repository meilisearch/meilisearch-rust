<!-- Do NOT update manually the README.md file -->
<!-- Update the README.tpl or src/lib.rs files instead, and run: -->
<!-- sh scripts/update-readme.sh -->

<p align="center">
  <img src="https://res.cloudinary.com/meilisearch/image/upload/v1587402338/SDKs/meilisearch_rust.svg" alt="MeiliSearch-Dotnet" width="200" height="200" />
</p>

<h1 align="center">MeiliSearch Rust SDK (WIP)</h1>

<h4 align="center">
  <a href="https://github.com/meilisearch/MeiliSearch">MeiliSearch</a> |
  <a href="https://www.meilisearch.com">Website</a> |
  <a href="https://blog.meilisearch.com">Blog</a> |
  <a href="https://twitter.com/meilisearch">Twitter</a> |
  <a href="https://docs.meilisearch.com">Documentation</a> |
  <a href="https://docs.meilisearch.com/faq">FAQ</a>
</h4>

<p align="center">
  <a href="https://crates.io/crates/meilisearch-sdk"><img src="https://img.shields.io/crates/v/meilisearch-sdk.svg" alt="crates.io"></a>
  <a href="https://github.com/meilisearch/meilisearch-rust/actions"><img src="https://github.com/meilisearch/meilisearch-rust/workflows/Test/badge.svg?branch=master" alt="Tests"></a>
  <a href="https://github.com/meilisearch/meilisearch-rust/blob/master/LICENSE"><img src="https://img.shields.io/badge/license-MIT-informational" alt="License"></a>
  <a href="https://github.com/meilisearch/MeiliSearch/discussions" alt="Discussions"><img src="https://img.shields.io/badge/github-discussions-red" /></a>
  <a href="https://slack.meilisearch.com"><img src="https://img.shields.io/badge/slack-MeiliSearch-blue.svg?logo=slack" alt="Slack"></a>
</p>

<p align="center">⚡ Lightning Fast, Ultra Relevant, and Typo-Tolerant Search Engine MeiliSearch client written in Rust</p>

**MeiliSearch Rust** is a client for **MeiliSearch** written in Rust. **MeiliSearch** is a powerful, fast, open-source, easy to use and deploy search engine. Both searching and indexing are highly customizable. Features such as typo-tolerance, filters, and synonyms are provided out-of-the-box.

### Table of Contents

- [🔧 Installation](#-installation)
- [🚀 Getting Started](#-getting-started)
- [🤖 Compatibility with MeiliSearch](#-compatibility-with-meilisearch)
- [🎬 Examples](#-examples)
- [🌐 Running in the Browser with WASM](#-running-in-the-browser-with-wasm)
- [⚙️ Development Workflow and Contributing](#️-development-workflow-and-contributing)

## 🔧 Installation

To use `meilisearch-sdk`, add this to your `Cargo.toml`:

```toml
[dependencies]
meilisearch-sdk = {{version}}
```

The following optional dependencies may also be useful:

```toml
tokio = { version = "0.2", features = ["macros"] }
serde = { version="1.0", features = ["derive"] }
```

Since this crate is async, you have to run your program in the [tokio](https://crates.io/crates/tokio) runtime. When targetting Wasm, the browser will replace tokio.

Using this crate is possible without [serde](https://crates.io/crates/serde), but a lot of features require serde.

### Run a MeiliSearch Instance

This crate requires a MeiliSearch server to run.

There are many easy ways to [download and run a MeiliSearch instance](https://docs.meilisearch.com/guides/advanced_guides/installation.html#download-and-launch).

For example, if you use Docker:
```bash
$ docker pull getmeili/meilisearch:latest # Fetch the latest version of MeiliSearch image from Docker Hub
$ docker run -it --rm -p 7700:7700 getmeili/meilisearch:latest ./meilisearch --master-key=masterKey
```

NB: you can also download MeiliSearch from **Homebrew** or **APT**.


## 🚀 Getting Started

{{readme}}

## 🤖 Compatibility with MeiliSearch

This package is compatible with the following MeiliSearch versions:
- `v0.12.X`
- `v0.11.X`

## 🎬 Examples

_TODO_

## 🌐 Running in the Browser with WASM

This crate fully supports WASM.

The only difference between the WASM and the native version is that the native version has one more variant (`Error::Http`) in the Error enum. That should not matter so much but we could add this variant in WASM too.

However, making a program intended to run in a web browser requires a **very** different design than a CLI program. To see an example of a simple Rust web app using MeiliSearch, see the [tutorial (not available yet)]().

WARNING: `meilisearch-sdk` will panic if no Window is available (ex: Web extension).

## ⚙️ Development Workflow and Contributing

Any new contribution is more than welcome in this project!

If you want to know more about the development workflow or want to contribute, please visit our [contributing guidelines](/CONTRIBUTING.md) for detailed instructions!

<hr>

**MeiliSearch** provides and maintains many **SDKs and Integration tools** like this one. We want to provide everyone with an **amazing search experience for any kind of project**. If you want to contribute, make suggestions, or just know what's going on right now, visit us in the [integration-guides](https://github.com/meilisearch/integration-guides) repository.
