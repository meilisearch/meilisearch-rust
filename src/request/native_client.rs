use isahc::AsyncBody;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::to_string;

use crate::Error;

use super::*;

pub(crate) async fn request<Q: Serialize, B: Serialize, Output: DeserializeOwned + 'static>(
    url: &str,
    apikey: Option<&str>,
    method: Method<Q, B>,
    expected_status_code: u16,
) -> Result<Output, Error> {
    request_impl(
        url,
        apikey,
        method,
        "application/json",
        expected_status_code,
        |body: B| AsyncBody::from_bytes_static(to_string(&body).unwrap()),
    )
    .await
}

pub(crate) async fn stream_request<
    Q: Serialize,
    B: futures_io::AsyncRead + Send + Sync + 'static,
    Output: DeserializeOwned + 'static,
>(
    url: &str,
    apikey: Option<&str>,
    method: Method<Q, B>,
    content_type: &str,
    expected_status_code: u16,
) -> Result<Output, Error> {
    request_impl(
        url,
        apikey,
        method,
        content_type,
        expected_status_code,
        |body: B| AsyncBody::from_reader(body),
    )
    .await
}

async fn request_impl<'a, Query, Body, Output, G>(
    url: &str,
    apikey: Option<&str>,
    method: Method<Query, Body>,
    content_type: &'a str,
    expected_status_code: u16,
    into_async_body: G,
) -> Result<Output, Error>
where
    Query: Serialize,
    Output: DeserializeOwned + 'static,
    G: FnOnce(Body) -> AsyncBody,
{
    use isahc::http::header;
    use isahc::http::method::Method as HttpMethod;
    use isahc::*;

    let mut builder = Request::builder()
        .header(header::USER_AGENT, qualified_version())
        .uri(add_query_parameters(url, method.query())?);
    if let Some(apikey) = apikey {
        builder = builder.header(header::AUTHORIZATION, format!("Bearer {apikey}"));
    }

    let builder = match method {
        Method::Get { .. } => builder.method(HttpMethod::GET).body(AsyncBody::empty()),
        Method::Delete { .. } => builder.method(HttpMethod::DELETE).body(AsyncBody::empty()),
        Method::Post { body, .. } => builder
            .method(HttpMethod::POST)
            .header(header::CONTENT_TYPE, content_type)
            .body(into_async_body(body)),
        Method::Patch { body, .. } => builder
            .method(HttpMethod::PATCH)
            .header(header::CONTENT_TYPE, content_type)
            .body(into_async_body(body)),
        Method::Put { body, .. } => builder
            .method(HttpMethod::PUT)
            .header(header::CONTENT_TYPE, content_type)
            .body(into_async_body(body)),
    };

    let mut response = builder
        .map_err(|_| crate::errors::Error::InvalidRequest)?
        .send_async()
        .await?;

    let status = response.status().as_u16();
    let mut body = response.text().await.map_err(isahc::Error::from)?;
    if body.is_empty() {
        body = "null".to_string();
    }

    parse_response(status, expected_status_code, &body, url.to_string())
}
