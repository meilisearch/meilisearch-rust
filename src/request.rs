use crate::errors::{Error, MeilisearchError};
use log::{error, trace, warn};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, to_string};

#[derive(Debug)]
pub(crate) enum Method<Q, B> {
    Get { query: Q },
    Post { query: Q, body: B },
    Patch { query: Q, body: B },
    Put { query: Q, body: B },
    Delete { query: Q },
}

#[cfg(not(target_arch = "wasm32"))]
pub fn add_query_parameters<Query: Serialize>(url: &str, query: &Query) -> Result<String, Error> {
    let query = yaup::to_string(query)?;

    if query.is_empty() {
        Ok(url.to_string())
    } else {
        Ok(format!("{url}?{query}"))
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) async fn request<
    Query: Serialize,
    Body: Serialize,
    Output: DeserializeOwned + 'static,
>(
    url: &str,
    apikey: &str,
    method: Method<Query, Body>,
    expected_status_code: u16,
) -> Result<Output, Error> {
    use isahc::http::header;
    use isahc::*;

    let auth = format!("Bearer {apikey}");
    let user_agent = qualified_version();

    let mut response = match &method {
        Method::Get { query } => {
            let url = add_query_parameters(url, query)?;

            Request::get(url)
                .header(header::AUTHORIZATION, auth)
                .header(header::USER_AGENT, user_agent)
                .body(())
                .map_err(|_| crate::errors::Error::InvalidRequest)?
                .send_async()
                .await?
        }
        Method::Delete { query } => {
            let url = add_query_parameters(url, query)?;

            Request::delete(url)
                .header(header::AUTHORIZATION, auth)
                .header(header::USER_AGENT, user_agent)
                .body(())
                .map_err(|_| crate::errors::Error::InvalidRequest)?
                .send_async()
                .await?
        }
        Method::Post { query, body } => {
            let url = add_query_parameters(url, query)?;

            Request::post(url)
                .header(header::AUTHORIZATION, auth)
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::USER_AGENT, user_agent)
                .body(to_string(&body).unwrap())
                .map_err(|_| crate::errors::Error::InvalidRequest)?
                .send_async()
                .await?
        }
        Method::Patch { query, body } => {
            let url = add_query_parameters(url, query)?;

            Request::patch(url)
                .header(header::AUTHORIZATION, auth)
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::USER_AGENT, user_agent)
                .body(to_string(&body).unwrap())
                .map_err(|_| crate::errors::Error::InvalidRequest)?
                .send_async()
                .await?
        }
        Method::Put { query, body } => {
            let url = add_query_parameters(url, query)?;

            Request::put(url)
                .header(header::AUTHORIZATION, auth)
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::USER_AGENT, user_agent)
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

    parse_response(status, expected_status_code, body)
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) async fn stream_request<
    'a,
    Query: Serialize,
    Body: futures_io::AsyncRead + Send + Sync + 'static,
    Output: DeserializeOwned + 'static,
>(
    url: &str,
    apikey: &str,
    method: Method<Query, Body>,
    content_type: &str,
    expected_status_code: u16,
) -> Result<Output, Error> {
    use isahc::http::header;
    use isahc::*;

    let auth = format!("Bearer {apikey}");
    let user_agent = qualified_version();

    let mut response = match method {
        Method::Get { query } => {
            let url = add_query_parameters(url, &query)?;

            Request::get(url)
                .header(header::AUTHORIZATION, auth)
                .header(header::USER_AGENT, user_agent)
                .body(())
                .map_err(|_| crate::errors::Error::InvalidRequest)?
                .send_async()
                .await?
        }
        Method::Delete { query } => {
            let url = add_query_parameters(url, &query)?;

            Request::delete(url)
                .header(header::AUTHORIZATION, auth)
                .header(header::USER_AGENT, user_agent)
                .body(())
                .map_err(|_| crate::errors::Error::InvalidRequest)?
                .send_async()
                .await?
        }
        Method::Post { query, body } => {
            let url = add_query_parameters(url, &query)?;

            Request::post(url)
                .header(header::AUTHORIZATION, auth)
                .header(header::USER_AGENT, user_agent)
                .header(header::CONTENT_TYPE, content_type)
                .body(AsyncBody::from_reader(body))
                .map_err(|_| crate::errors::Error::InvalidRequest)?
                .send_async()
                .await?
        }
        Method::Patch { query, body } => {
            let url = add_query_parameters(url, &query)?;

            Request::patch(url)
                .header(header::AUTHORIZATION, auth)
                .header(header::USER_AGENT, user_agent)
                .header(header::CONTENT_TYPE, content_type)
                .body(AsyncBody::from_reader(body))
                .map_err(|_| crate::errors::Error::InvalidRequest)?
                .send_async()
                .await?
        }
        Method::Put { query, body } => {
            let url = add_query_parameters(url, &query)?;

            Request::put(url)
                .header(header::AUTHORIZATION, auth)
                .header(header::USER_AGENT, user_agent)
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

    parse_response(status, expected_status_code, body)
}

#[cfg(target_arch = "wasm32")]
pub fn add_query_parameters<Query: Serialize>(
    mut url: String,
    query: &Query,
) -> Result<String, Error> {
    let query = yaup::to_string(query)?;

    if !query.is_empty() {
        url = format!("{}?{}", url, query);
    };
    return Ok(url);
}
#[cfg(target_arch = "wasm32")]
pub(crate) async fn request<
    Query: Serialize,
    Body: Serialize,
    Output: DeserializeOwned + 'static,
>(
    url: &str,
    apikey: &str,
    method: Method<Query, Body>,
    expected_status_code: u16,
) -> Result<Output, Error> {
    use wasm_bindgen::JsValue;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{Headers, RequestInit, Response};

    const CONTENT_TYPE: &str = "Content-Type";
    const JSON: &str = "application/json";

    // The 2 following unwraps should not be able to fail
    let mut mut_url = url.clone().to_string();
    let headers = Headers::new().unwrap();
    headers
        .append("Authorization", format!("Bearer {}", apikey).as_str())
        .unwrap();
    headers
        .append("X-Meilisearch-Client", qualified_version().as_str())
        .unwrap();

    let mut request: RequestInit = RequestInit::new();
    request.headers(&headers);

    match &method {
        Method::Get { query } => {
            mut_url = add_query_parameters(mut_url, &query)?;

            request.method("GET");
        }
        Method::Delete { query } => {
            mut_url = add_query_parameters(mut_url, &query)?;
            request.method("DELETE");
        }
        Method::Patch { query, body } => {
            mut_url = add_query_parameters(mut_url, &query)?;
            request.method("PATCH");
            headers.append(CONTENT_TYPE, JSON).unwrap();
            request.body(Some(&JsValue::from_str(&to_string(body).unwrap())));
        }
        Method::Post { query, body } => {
            mut_url = add_query_parameters(mut_url, &query)?;
            request.method("POST");
            headers.append(CONTENT_TYPE, JSON).unwrap();
            request.body(Some(&JsValue::from_str(&to_string(body).unwrap())));
        }
        Method::Put { query, body } => {
            mut_url = add_query_parameters(mut_url, &query)?;
            request.method("PUT");
            headers.append(CONTENT_TYPE, JSON).unwrap();
            request.body(Some(&JsValue::from_str(&to_string(body).unwrap())));
        }
    }

    let window = web_sys::window().unwrap(); // TODO remove this unwrap
    let response =
        match JsFuture::from(window.fetch_with_str_and_init(mut_url.as_str(), &request)).await {
            Ok(response) => Response::from(response),
            Err(e) => {
                error!("Network error: {:?}", e);
                return Err(Error::UnreachableServer);
            }
        };
    let status = response.status() as u16;
    let text = match response.text() {
        Ok(text) => match JsFuture::from(text).await {
            Ok(text) => text,
            Err(e) => {
                error!("Invalid response: {:?}", e);
                return Err(Error::HttpError("Invalid response".to_string()));
            }
        },
        Err(e) => {
            error!("Invalid response: {:?}", e);
            return Err(Error::HttpError("Invalid response".to_string()));
        }
    };

    if let Some(t) = text.as_string() {
        if t.is_empty() {
            parse_response(status, expected_status_code, String::from("null"))
        } else {
            parse_response(status, expected_status_code, t)
        }
    } else {
        error!("Invalid response");
        Err(Error::HttpError("Invalid utf8".to_string()))
    }
}

fn parse_response<Output: DeserializeOwned>(
    status_code: u16,
    expected_status_code: u16,
    body: String,
) -> Result<Output, Error> {
    if status_code == expected_status_code {
        match from_str::<Output>(&body) {
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
    // TODO: create issue where it is clear what the HTTP error is
    // ParseError(Error("invalid type: null, expected struct MeilisearchError", line: 1, column: 4))

    warn!(
        "Expected response code {}, got {}",
        expected_status_code, status_code
    );
    match from_str::<MeilisearchError>(&body) {
        Ok(e) => Err(Error::from(e)),
        Err(e) => Err(Error::ParseError(e)),
    }
}

pub fn qualified_version() -> String {
    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

    format!("Meilisearch Rust (v{})", VERSION.unwrap_or("unknown"))
}
