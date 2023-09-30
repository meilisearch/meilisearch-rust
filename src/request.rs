#[cfg(all(
    not(target_arch = "wasm32"),
    any(feature = "isahc-static-curl", feature = "isahc-static-ssl")
))]
mod isahc_native_client;

#[cfg(all(
    not(target_arch = "wasm32"),
    any(feature = "reqwest-native-tls", feature = "reqwest-rustls")
))]
mod reqwest_native_client;

#[cfg(target_arch = "wasm32")]
mod wasm_client;

use http::header;
use log::{error, trace, warn};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::from_str;
use url::Url;

use crate::{Error, MeilisearchCommunicationError, MeilisearchError};

pub(crate) use method::Method;
mod method {
    #[derive(Debug)]
    pub enum Method<Q, B> {
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

        pub fn http_method(&self) -> http::Method {
            match self {
                Method::Get { .. } => http::Method::GET,
                Method::Post { .. } => http::Method::POST,
                Method::Patch { .. } => http::Method::PATCH,
                Method::Put { .. } => http::Method::PUT,
                Method::Delete { .. } => http::Method::DELETE,
            }
        }
    }
}

pub(crate) async fn request<'a, Q: 'a, B: 'a, O>(
    url: &'a str,
    apikey: Option<&'a str>,
    method: Method<Q, B>,
    expected_status_code: u16,
) -> Result<O, Error>
where
    Q: Serialize + Send,
    B: Serialize + Send,
    O: DeserializeOwned,
{
    const CONTENT_TYPE: &str = "application/json";

    #[cfg(all(
        not(target_arch = "wasm32"),
        any(feature = "isahc-static-curl", feature = "isahc-static-ssl")
    ))]
    type RequestClient<B> =
        isahc_native_client::IsahcRequestClient<isahc_native_client::SerializeBodyTransform, B>;
    #[cfg(all(
        not(target_arch = "wasm32"),
        any(feature = "reqwest-native-tls", feature = "reqwest-rustls")
    ))]
    type RequestClient<B> =
        reqwest_native_client::ReqwestClient<reqwest_native_client::SerializeBodyTransform, B>;

    #[cfg(not(target_arch = "wasm32"))]
    return RequestClient::request(url, apikey, method, CONTENT_TYPE, expected_status_code).await;

    #[cfg(target_arch = "wasm32")]
    return self::wasm_client::BrowserRequestClient::request(
        url,
        apikey,
        method,
        CONTENT_TYPE,
        expected_status_code,
    )
    .await;
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) async fn stream_request<
    Q: Serialize + Send,
    B: futures_io::AsyncRead + Send + Sync + 'static,
    Output: DeserializeOwned,
>(
    url: &str,
    apikey: Option<&str>,
    method: Method<Q, B>,
    content_type: &str,
    expected_status_code: u16,
) -> Result<Output, Error> {
    #[cfg(any(feature = "isahc-static-curl", feature = "isahc-static-ssl"))]
    type RequestClient<B> =
        isahc_native_client::IsahcRequestClient<isahc_native_client::ReadBodyTransform, B>;
    #[cfg(any(feature = "reqwest-native-tls", feature = "reqwest-rustls"))]
    type RequestClient<B> =
        reqwest_native_client::ReqwestClient<reqwest_native_client::ReadBodyTransform, B>;

    RequestClient::request(url, apikey, method, content_type, expected_status_code).await
}

#[async_trait::async_trait]
trait RequestClient<'a, B: 'a + Send>: Sized {
    type Request: Send;
    type Response: Send;
    type HttpError: Into<crate::Error> + Send;

    fn new(url: Url) -> Self;

    fn append_header(self, name: http::HeaderName, value: http::HeaderValue) -> Self;

    fn with_method(self, http_method: http::Method) -> Self;

    fn add_body(self, body: Option<B>) -> Self::Request;

    async fn send_request(request: Self::Request) -> Result<Self::Response, Error>;

    fn extract_status_code(response: &Self::Response) -> u16;

    async fn response_to_text(response: Self::Response) -> Result<String, Error>;

    async fn request<T, Q>(
        url: &'a str,
        apikey: Option<&'a str>,
        method: Method<Q, B>,
        content_type: &'a str,
        expected_status_code: u16,
    ) -> Result<T, Error>
    where
        Q: Serialize + 'a + Send,
        T: DeserializeOwned,
    {
        let mut request_client = Self::new(add_query_parameters(url, method.query())?.parse()?)
            .with_method(method.http_method())
            .append_header(header::USER_AGENT, USER_AGENT_HEADER_VALUE.clone());

        if let Some(apikey) = apikey {
            request_client = request_client
                .append_header(header::AUTHORIZATION, format!("Bearer {apikey}").parse()?);
        }

        let body = match method {
            Method::Put { body, .. } | Method::Post { body, .. } | Method::Patch { body, .. } => {
                request_client =
                    request_client.append_header(header::CONTENT_TYPE, content_type.parse()?);
                Some(body)
            }
            _ => None,
        };

        let response = Self::send_request(request_client.add_body(body)).await?;
        let status = Self::extract_status_code(&response);
        let text = Self::response_to_text(response).await?;

        Self::parse_response(status, expected_status_code, &text, url.to_string())
    }

    fn parse_response<O: DeserializeOwned>(
        status_code: u16,
        expected_status_code: u16,
        mut body: &str,
        url: String,
    ) -> Result<O, Error> {
        if body.is_empty() {
            body = "null"
        }

        if status_code == expected_status_code {
            match from_str::<O>(body) {
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
}

lazy_static::lazy_static! {
    pub static ref USER_AGENT_HEADER_VALUE: header::HeaderValue = {
        format!("Meilisearch Rust (v{})", option_env!("CARGO_PKG_VERSION").unwrap_or("unknown")).parse().expect("invalid header value")
    };
}

pub fn add_query_parameters<Query: Serialize>(url: &str, query: &Query) -> Result<String, Error> {
    let query = yaup::to_string(query)?;

    Ok(if query.is_empty() {
        url.into()
    } else {
        format!("{url}?{query}")
    })
}
