# Contributing <!-- omit in toc -->

First of all, thank you for contributing to Meilisearch! The goal of this document is to provide everything you need to know in order to contribute to Meilisearch and its different integrations.

- [Assumptions](#assumptions)
- [How to Contribute](#how-to-contribute)
- [Development Workflow](#development-workflow)
- [Git Guidelines](#git-guidelines)
- [Release Process (for internal team only)](#release-process-for-internal-team-only)


## Assumptions

1. **You're familiar with [GitHub](https://github.com) and the [Pull Request](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/about-pull-requests)(PR) workflow.**
2. **You've read the Meilisearch [documentation](https://www.meilisearch.com/docs) and the [README](/README.md).**
3. **You know about the [Meilisearch community](https://discord.com/invite/meilisearch). Please use this for help.**

## How to Contribute

1. Make sure that the contribution you want to make is explained or detailed in a GitHub issue! Find an [existing issue](https://github.com/meilisearch/meilisearch-rust/issues/) or [open a new one](https://github.com/meilisearch/meilisearch-rust/issues/new).
2. Once done, [fork the meilisearch-rust repository](https://help.github.com/en/github/getting-started-with-github/fork-a-repo) in your own GitHub account. Ask a maintainer if you want your issue to be checked before making a PR.
3. [Create a new Git branch](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/creating-and-deleting-branches-within-your-repository).
4. Review the [Development Workflow](#development-workflow) section that describes the steps to maintain the repository.
5. Make the changes on your branch.
6. [Submit the branch as a PR](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/creating-a-pull-request-from-a-fork) pointing to the `main` branch of the main meilisearch-rust repository. A maintainer should comment and/or review your Pull Request within a few days. Although depending on the circumstances, it may take longer.<br>
 We do not enforce a naming convention for the PRs, but **please use something descriptive of your changes**, having in mind that the title of your PR will be automatically added to the next [release changelog](https://github.com/meilisearch/meilisearch-rust/releases/).

## Development Workflow

You can set up your local environment natively or using `docker`, check out the [`docker-compose.yml`](/docker-compose.yml).

Example of running all the checks with docker:
```bash
docker-compose run --rm package bash -c "cargo test"
```

To install dependencies:

```bash
cargo build --release
```

To ensure the same dependency versions in all environments, for example the CI, update the dependencies by running: `cargo update`.

### Tests <!-- omit in toc -->

To run the tests, run:

```bash
# Tests
curl -L https://install.meilisearch.com | sh # download Meilisearch
./meilisearch --master-key=masterKey --no-analytics # run Meilisearch
cargo test
```

There is two kind of tests, documentation tests and unit tests.
If you need to write or read the unit tests you should consider reading this
[readme](meilisearch-test-macro/README.md) about our custom testing macro.

Also, the WASM example compilation should be checked:

```bash
rustup target add wasm32-unknown-unknown
cargo check -p web_app --target wasm32-unknown-unknown
```

Each PR should pass the tests to be accepted.

### Clippy <!-- omit in toc -->

Each PR should pass [`clippy`](https://github.com/rust-lang/rust-clippy) (the linter) to be accepted.

```bash
cargo clippy -- -D warnings
```

If you don't have `clippy` installed on your machine yet, run:

```bash
rustup update
rustup component add clippy
```

⚠️ Also, if you have installed `clippy` a long time ago, you might need to update it:

```bash
rustup update
```

### Fmt

Each PR should pass the format test to be accepted.

Run the following to fix the formating errors:

```
cargo fmt
```

and the following to test if the formating is correct:
```
cargo fmt --all -- --check
```

### Update the README <!-- omit in toc -->

The README is generated. Please do not update manually the `README.md` file.

Instead, update the `README.tpl` and `src/lib.rs` files, and run:

```sh
sh scripts/update-readme.sh
```

Then, push the changed files.

You can check the current `README.md` is up-to-date by running:

```sh
sh scripts/check-readme.sh
# To see the diff
sh scripts/check-readme.sh --diff
```

If it's not, the CI will fail on your PR.

### Yaml lint

To check if your `yaml` files are correctly formatted, you need to [install yamllint](https://yamllint.readthedocs.io/en/stable/quickstart.html#installing-yamllint) and then run `yamllint .`

## Git Guidelines

### Git Branches <!-- omit in toc -->

All changes must be made in a branch and submitted as PR.
We do not enforce any branch naming style, but please use something descriptive of your changes.

### Git Commits <!-- omit in toc -->

As minimal requirements, your commit message should:
- be capitalized
- not finish by a dot or any other punctuation character (!,?)
- start with a verb so that we can read your commit message this way: "This commit will ...", where "..." is the commit message.
  e.g.: "Fix the home page button" or "Add more tests for create_index method"

We don't follow any other convention, but if you want to use one, we recommend [this one](https://chris.beams.io/posts/git-commit/).

### GitHub Pull Requests <!-- omit in toc -->

Some notes on GitHub PRs:

- [Convert your PR as a draft](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/changing-the-stage-of-a-pull-request) if your changes are a work in progress: no one will review it until you pass your PR as ready for review.<br>
  The draft PR can be very useful if you want to show that you are working on something and make your work visible.
- The branch related to the PR must be **up-to-date with `main`** before merging. Fortunately, this project [integrates a bot](https://github.com/meilisearch/integration-guides/blob/main/resources/bors.md) to automatically enforce this requirement without the PR author having to do it manually.
- All PRs must be reviewed and approved by at least one maintainer.
- The PR title should be accurate and descriptive of the changes. The title of the PR will be indeed automatically added to the next [release changelogs](https://github.com/meilisearch/meilisearch-rust/releases/).

## Release Process (for the internal team only)

Meilisearch tools follow the [Semantic Versioning Convention](https://semver.org/).

### Automation to Rebase and Merge the PRs <!-- omit in toc -->

This project integrates a bot that helps us manage pull requests merging.<br>
_[Read more about this](https://github.com/meilisearch/integration-guides/blob/main/resources/bors.md)._

### Automated Changelogs <!-- omit in toc -->

This project integrates a tool to create automated changelogs.<br>
_[Read more about this](https://github.com/meilisearch/integration-guides/blob/main/resources/release-drafter.md)._

### How to Publish the Release <!-- omit in toc -->

⚠️ Before doing anything, make sure you got through the guide about [Releasing an Integration](https://github.com/meilisearch/integration-guides/blob/main/resources/integration-release.md).

Make a PR modifying the file [`Cargo.toml`](/Cargo.toml):

```toml
version = "X.X.X"
```

the [`README.tpl`](/README.tpl):

```rust
//! meilisearch-sdk = "X.X.X"
```

and the [code-samples file](/.code-samples.meilisearch.yaml):

```yml
  meilisearch-sdk = "X.X.X"
```

with the right version.


After the changes on `Cargo.toml`, run the following command: 

```
sh scripts/update_macro_versions.sh
```

After the changes on `lib.rs`, run the following command:

```bash
sh scripts/update-readme.sh
```

Once the changes are merged on `main`, you can publish the current draft release via the [GitHub interface](https://github.com/meilisearch/meilisearch-rust/releases): on this page, click on `Edit` (related to the draft release) > update the description (be sure you apply [these recommendations](https://github.com/meilisearch/integration-guides/blob/main/resources/integration-release.md#writting-the-release-description)) > when you are ready, click on `Publish release`.

GitHub Actions will be triggered and push the package to [crates.io](https://crates.io/crates/meilisearch-sdk).

<hr>

Thank you again for reading this through. We can not wait to begin to work with you if you make your way through this contributing guide ❤️
