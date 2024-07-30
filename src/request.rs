use std::convert::Infallible;

use async_trait::async_trait;
use log::{error, trace, warn};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, to_vec};

use crate::errors::{Error, MeilisearchCommunicationError, MeilisearchError};

#[derive(Debug)]
pub enum Method<Q, B> {
    Get { query: Q },
    Post { query: Q, body: B },
    Patch { query: Q, body: B },
    Put { query: Q, body: B },
    Delete { query: Q },
}

impl<Q, B> Method<Q, B> {
    pub fn map_body<B2>(self, f: impl Fn(B) -> B2) -> Method<Q, B2> {
        match self {
            Method::Get { query } => Method::Get { query },
            Method::Delete { query } => Method::Delete { query },
            Method::Post { query, body } => Method::Post {
                query,
                body: f(body),
            },
            Method::Patch { query, body } => Method::Patch {
                query,
                body: f(body),
            },
            Method::Put { query, body } => Method::Put {
                query,
                body: f(body),
            },
        }
    }

    pub fn query(&self) -> &Q {
        match self {
            Method::Get { query } => query,
            Method::Delete { query } => query,
            Method::Post { query, .. } => query,
            Method::Put { query, .. } => query,
            Method::Patch { query, .. } => query,
        }
    }

    pub fn body(&self) -> Option<&B> {
        match self {
            Method::Get { query: _ } | Method::Delete { query: _ } => None,
            Method::Post { body, query: _ } => Some(body),
            Method::Put { body, query: _ } => Some(body),
            Method::Patch { body, query: _ } => Some(body),
        }
    }

    pub fn into_body(self) -> Option<B> {
        match self {
            Method::Get { query: _ } | Method::Delete { query: _ } => None,
            Method::Post { body, query: _ } => Some(body),
            Method::Put { body, query: _ } => Some(body),
            Method::Patch { body, query: _ } => Some(body),
        }
    }
}

#[cfg_attr(feature = "futures-unsend", async_trait(?Send))]
#[cfg_attr(not(feature = "futures-unsend"), async_trait)]
pub trait HttpClient: Clone + Send + Sync {
    async fn request<Query, Body, Output>(
        &self,
        url: &str,
        method: Method<Query, Body>,
        expected_status_code: u16,
    ) -> Result<Output, Error>
    where
        Query: Serialize + Send + Sync,
        Body: Serialize + Send + Sync,
        Output: DeserializeOwned + 'static + Send,
    {
        use futures::io::Cursor;

        self.stream_request(
            url,
            method.map_body(|body| Cursor::new(to_vec(&body).unwrap())),
            "application/json",
            expected_status_code,
        )
        .await
    }

    async fn stream_request<
        Query: Serialize + Send + Sync,
        Body: futures_io::AsyncRead + Send + Sync + 'static,
        Output: DeserializeOwned + 'static,
    >(
        &self,
        url: &str,
        method: Method<Query, Body>,
        content_type: &str,
        expected_status_code: u16,
    ) -> Result<Output, Error>;
}

pub fn parse_response<Output: DeserializeOwned>(
    status_code: u16,
    expected_status_code: u16,
    body: &str,
    url: String,
) -> Result<Output, Error> {
    if status_code == expected_status_code {
        return match from_str::<Output>(body) {
            Ok(output) => {
                trace!("Request succeed");
                Ok(output)
            }
            Err(e) => {
                error!("Request succeeded but failed to parse response");
                Err(Error::ParseError(e))
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

#[cfg_attr(feature = "futures-unsend", async_trait(?Send))]
#[cfg_attr(not(feature = "futures-unsend"), async_trait)]
impl HttpClient for Infallible {
    async fn request<Query, Body, Output>(
        &self,
        _url: &str,
        _method: Method<Query, Body>,
        _expected_status_code: u16,
    ) -> Result<Output, Error>
    where
        Query: Serialize + Send + Sync,
        Body: Serialize + Send + Sync,
        Output: DeserializeOwned + 'static + Send,
    {
        unreachable!()
    }

    async fn stream_request<
        Query: Serialize + Send + Sync,
        Body: futures_io::AsyncRead + Send + Sync + 'static,
        Output: DeserializeOwned + 'static,
    >(
        &self,
        _url: &str,
        _method: Method<Query, Body>,
        _content_type: &str,
        _expected_status_code: u16,
    ) -> Result<Output, Error> {
        unreachable!()
    }
}
