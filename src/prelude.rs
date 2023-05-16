pub use client::*;

use crate::{client, indexes, request};

#[cfg(feature = "isahc")]
#[cfg(not(target_arch = "wasm32"))]
pub type Client = client::Client<request::IsahcClient>;
#[cfg(feature = "isahc")]
#[cfg(not(target_arch = "wasm32"))]
pub type Index = indexes::Index<request::IsahcClient>;

#[cfg(target_arch = "wasm32")]
pub type Client = client::Client<request::WebSysClient>;
#[cfg(target_arch = "wasm32")]
pub type Index = indexes::Index<request::WebSysClient>;
