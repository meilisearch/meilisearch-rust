use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    errors::Error,
    request::{parse_response, HttpClient, Method},
};

#[derive(Debug, Clone, Default)]
pub struct ReqwestClient {
    client: reqwest::Client,
}

impl ReqwestClient {
    pub fn new(api_key: Option<&str>) -> Self {
        use reqwest::{header, ClientBuilder};

        let builder = ClientBuilder::new();
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str(&qualified_version()).unwrap(),
        );

        if let Some(api_key) = api_key {
            headers.insert(
                header::AUTHORIZATION,
                header::HeaderValue::from_str(&format!("Bearer {api_key}")).unwrap(),
            );
        }

        let builder = builder.default_headers(headers);
        let client = builder.build().unwrap();

        ReqwestClient { client }
    }
}

#[async_trait]
impl HttpClient for ReqwestClient {
    async fn stream_request<
        'a,
        Query: Serialize + Send + Sync,
        Body: futures_io::AsyncRead + Send + Sync + 'static,
        Output: DeserializeOwned + 'static,
    >(
        &self,
        url: &str,
        method: Method<Query, Body>,
        content_type: &str,
        expected_status_code: u16,
    ) -> Result<Output, Error> {
        use reqwest::header;

        let query = method.query();
        let query = yaup::to_string(query)?;

        let url = if query.is_empty() {
            url.to_string()
        } else {
            format!("{url}?{query}")
        };

        let mut request = self.client.request(verb(&method), &url);

        if let Some(body) = method.into_body() {
            let reader = tokio_util::compat::FuturesAsyncReadCompatExt::compat(body);
            let stream = tokio_util::io::ReaderStream::new(reader);
            let body = reqwest::Body::wrap_stream(stream);

            request = request
                .header(header::CONTENT_TYPE, content_type)
                .body(body);
        }

        let response = self.client.execute(request.build()?).await?;
        let status = response.status().as_u16();
        let mut body = response.text().await?;

        if body.is_empty() {
            body = "null".to_string();
        }

        parse_response(status, expected_status_code, &body, url.to_string())
    }
}

fn verb<Q, B>(method: &Method<Q, B>) -> reqwest::Method {
    match method {
        Method::Get { .. } => reqwest::Method::GET,
        Method::Delete { .. } => reqwest::Method::DELETE,
        Method::Post { .. } => reqwest::Method::POST,
        Method::Put { .. } => reqwest::Method::PUT,
        Method::Patch { .. } => reqwest::Method::PATCH,
    }
}

pub fn qualified_version() -> String {
    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

    format!("Meilisearch Rust (v{})", VERSION.unwrap_or("unknown"))
}
