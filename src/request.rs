use log::{error, trace, warn};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::from_str;

use crate::{Error, MeilisearchCommunicationError, MeilisearchError};

#[derive(Debug)]
pub(crate) enum Method<Q, B> {
    Get { query: Q },
    Post { query: Q, body: B },
    Patch { query: Q, body: B },
    Put { query: Q, body: B },
    Delete { query: Q },
}

impl<Q, B> Method<Q, B> {
    pub fn query(&self) -> &Q {
        match self {
            Method::Get { query } => query,
            Method::Post { query, .. } => query,
            Method::Patch { query, .. } => query,
            Method::Put { query, .. } => query,
            Method::Delete { query } => query,
        }
    }
}

fn parse_response<Output: DeserializeOwned>(
    status_code: u16,
    expected_status_code: u16,
    body: &str,
    url: String,
) -> Result<Output, Error> {
    if status_code == expected_status_code {
        match from_str::<Output>(body) {
            Ok(output) => {
                trace!("Request succeed");
                return Ok(output);
            }
            Err(e) => {
                error!("Request succeeded but failed to parse response");
                return Err(Error::ParseError(e));
            }
        };
    }

    warn!(
        "Expected response code {}, got {}",
        expected_status_code, status_code
    );

    match from_str::<MeilisearchError>(body) {
        Ok(e) => Err(Error::from(e)),
        Err(e) => {
            if status_code >= 400 {
                return Err(Error::MeilisearchCommunication(
                    MeilisearchCommunicationError {
                        status_code,
                        message: None,
                        url,
                    },
                ));
            }
            Err(Error::ParseError(e))
        }
    }
}

pub fn qualified_version() -> String {
    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

    format!("Meilisearch Rust (v{})", VERSION.unwrap_or("unknown"))
}

pub fn add_query_parameters<Query: Serialize>(url: &str, query: &Query) -> Result<String, Error> {
    let query = yaup::to_string(query)?;

    Ok(if query.is_empty() {
        url.into()
    } else {
        format!("{url}?{query}")
    })
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) use native_client::{request, stream_request};
#[cfg(not(target_arch = "wasm32"))]
mod native_client;

#[cfg(target_arch = "wasm32")]
pub(crate) use wasm_client::request;
#[cfg(target_arch = "wasm32")]
mod wasm_client;
