use serde::{de::DeserializeOwned, Serialize};
use serde_json::to_string;

use crate::Error;

use super::*;

pub(crate) async fn request<
    Query: Serialize,
    Body: Serialize,
    Output: DeserializeOwned + 'static,
>(
    url: &str,
    apikey: Option<&str>,
    method: Method<Query, Body>,
    expected_status_code: u16,
) -> Result<Output, Error> {
    use isahc::http::header;
    use isahc::http::method::Method as HttpMethod;
    use isahc::*;

    let builder = Request::builder().header(header::USER_AGENT, qualified_version());
    let builder = match apikey {
        Some(apikey) => builder.header(header::AUTHORIZATION, format!("Bearer {apikey}")),
        None => builder,
    };

    let mut response = match &method {
        Method::Get { query } => {
            let url = add_query_parameters(url, query)?;

            builder
                .method(HttpMethod::GET)
                .uri(url)
                .body(())
                .map_err(|_| crate::errors::Error::InvalidRequest)?
                .send_async()
                .await?
        }
        Method::Delete { query } => {
            let url = add_query_parameters(url, query)?;

            builder
                .method(HttpMethod::DELETE)
                .uri(url)
                .body(())
                .map_err(|_| crate::errors::Error::InvalidRequest)?
                .send_async()
                .await?
        }
        Method::Post { query, body } => {
            let url = add_query_parameters(url, query)?;

            builder
                .method(HttpMethod::POST)
                .uri(url)
                .header(header::CONTENT_TYPE, "application/json")
                .body(to_string(&body).unwrap())
                .map_err(|_| crate::errors::Error::InvalidRequest)?
                .send_async()
                .await?
        }
        Method::Patch { query, body } => {
            let url = add_query_parameters(url, query)?;

            builder
                .method(HttpMethod::PATCH)
                .uri(url)
                .header(header::CONTENT_TYPE, "application/json")
                .body(to_string(&body).unwrap())
                .map_err(|_| crate::errors::Error::InvalidRequest)?
                .send_async()
                .await?
        }
        Method::Put { query, body } => {
            let url = add_query_parameters(url, query)?;

            builder
                .method(HttpMethod::PUT)
                .uri(url)
                .header(header::CONTENT_TYPE, "application/json")
                .body(to_string(&body).unwrap())
                .map_err(|_| crate::errors::Error::InvalidRequest)?
                .send_async()
                .await?
        }
    };

    let status = response.status().as_u16();

    let mut body = response
        .text()
        .await
        .map_err(|e| crate::errors::Error::HttpError(e.into()))?;

    if body.is_empty() {
        body = "null".to_string();
    }

    parse_response(status, expected_status_code, &body, url.to_string())
    // parse_response(status, expected_status_code, body)
}

pub(crate) async fn stream_request<
    'a,
    Query: Serialize,
    Body: futures_io::AsyncRead + Send + Sync + 'static,
    Output: DeserializeOwned + 'static,
>(
    url: &str,
    apikey: Option<&str>,
    method: Method<Query, Body>,
    content_type: &str,
    expected_status_code: u16,
) -> Result<Output, Error> {
    use isahc::http::header;
    use isahc::http::method::Method as HttpMethod;
    use isahc::*;

    let builder = Request::builder().header(header::USER_AGENT, qualified_version());
    let builder = match apikey {
        Some(apikey) => builder.header(header::AUTHORIZATION, format!("Bearer {apikey}")),
        None => builder,
    };

    let mut response = match method {
        Method::Get { query } => {
            let url = add_query_parameters(url, &query)?;

            builder
                .method(HttpMethod::GET)
                .uri(url)
                .body(())
                .map_err(|_| crate::errors::Error::InvalidRequest)?
                .send_async()
                .await?
        }
        Method::Delete { query } => {
            let url = add_query_parameters(url, &query)?;

            builder
                .method(HttpMethod::DELETE)
                .uri(url)
                .body(())
                .map_err(|_| crate::errors::Error::InvalidRequest)?
                .send_async()
                .await?
        }
        Method::Post { query, body } => {
            let url = add_query_parameters(url, &query)?;

            builder
                .method(HttpMethod::POST)
                .uri(url)
                .header(header::CONTENT_TYPE, content_type)
                .body(AsyncBody::from_reader(body))
                .map_err(|_| crate::errors::Error::InvalidRequest)?
                .send_async()
                .await?
        }
        Method::Patch { query, body } => {
            let url = add_query_parameters(url, &query)?;

            builder
                .method(HttpMethod::PATCH)
                .uri(url)
                .header(header::CONTENT_TYPE, content_type)
                .body(AsyncBody::from_reader(body))
                .map_err(|_| crate::errors::Error::InvalidRequest)?
                .send_async()
                .await?
        }
        Method::Put { query, body } => {
            let url = add_query_parameters(url, &query)?;

            builder
                .method(HttpMethod::PUT)
                .uri(url)
                .header(header::CONTENT_TYPE, content_type)
                .body(AsyncBody::from_reader(body))
                .map_err(|_| crate::errors::Error::InvalidRequest)?
                .send_async()
                .await?
        }
    };

    let status = response.status().as_u16();

    let mut body = response
        .text()
        .await
        .map_err(|e| crate::errors::Error::HttpError(e.into()))?;

    if body.is_empty() {
        body = "null".to_string();
    }

    parse_response(status, expected_status_code, &body, url.to_string())
}
