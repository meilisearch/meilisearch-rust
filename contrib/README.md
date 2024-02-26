This directory contains unofficial `RequestClient` implementations that can replace the default `reqwest` based one. Those wishing to do so are encouraged to fork this repository and replace `src/request/request_client_impl.rs` with the respective client implementation.

Keeping `RequestClient` implementations separated from the official SDK is a deliberate design decision. https://github.com/meilisearch/meilisearch-rust/pull/524#issuecomment-1735105590 may be a good starting point if you're interested in reading about the motivation behind it.
