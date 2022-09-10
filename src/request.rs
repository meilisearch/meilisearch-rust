use crate::errors::{Error, MeilisearchError};
use log::{error, trace, warn};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, to_string};

#[derive(Debug)]
pub(crate) enum Data<T, S>
where
    T: IntoIterator,
    T::Item: Serialize,
    S: Serialize,
{
    Iterable(T),
    NonIterable(S),
}

#[derive(Debug)]
pub(crate) enum Method<T, S>
where
    T: IntoIterator,
    T::Item: Serialize,
    S: Serialize,
{
    Get(S),
    Post(Data<T, S>),
    Patch(Data<T, S>),
    Put(Data<T, S>),
    Delete,
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) async fn request<T, S, Output>(
    url: &str,
    apikey: &str,
    method: Method<T, S>,
    expected_status_code: u16,
) -> Result<Output, Error>
where
    T: IntoIterator,
    T::Item: Serialize,
    S: Serialize,
    Output: DeserializeOwned + 'static,
{
    use isahc::http::header;
    use isahc::*;

    let auth = format!("Bearer {}", apikey);
    let user_agent = qualified_version();

    let mut response = match method {
        Method::Get(query) => {
            let query = yaup::to_string(&query)?;

            let url = if query.is_empty() {
                url.to_string()
            } else {
                format!("{}?{}", url, query)
            };

            Request::get(url)
                .header(header::AUTHORIZATION, auth)
                .header(header::USER_AGENT, user_agent)
                .body(())
                .map_err(|_| crate::errors::Error::InvalidRequest)?
                .send_async()
                .await?
        }
        Method::Delete => {
            Request::delete(url)
                .header(header::AUTHORIZATION, auth)
                .header(header::USER_AGENT, user_agent)
                .body(())
                .map_err(|_| crate::errors::Error::InvalidRequest)?
                .send_async()
                .await?
        }
        Method::Post(body) => {
            let (content_type, body_serialized) = match body {
                Data::Iterable(body) => ("application/x-ndjson", to_jsonlines(body)),
                Data::NonIterable(body) => ("application/json", to_string(&body).unwrap()),
            };
            Request::post(url)
                .header(header::AUTHORIZATION, auth)
                .header(header::CONTENT_TYPE, content_type)
                .header(header::USER_AGENT, user_agent)
                .body(body_serialized)
                .map_err(|_| crate::errors::Error::InvalidRequest)?
                .send_async()
                .await?
        }
        Method::Patch(body) => {
            let (content_type, body_serialized) = match body {
                Data::Iterable(body) => ("application/x-ndjson", to_jsonlines(body)),
                Data::NonIterable(body) => ("application/json", to_string(&body).unwrap()),
            };
            Request::patch(url)
                .header(header::AUTHORIZATION, auth)
                .header(header::CONTENT_TYPE, content_type)
                .header(header::USER_AGENT, user_agent)
                .body(body_serialized)
                .map_err(|_| crate::errors::Error::InvalidRequest)?
                .send_async()
                .await?
        }
        Method::Put(body) => {
            let (content_type, body_serialized) = match body {
                Data::Iterable(body) => ("application/x-ndjson", to_jsonlines(body)),
                Data::NonIterable(body) => ("application/json", to_string(&body).unwrap()),
            };
            Request::put(url)
                .header(header::AUTHORIZATION, auth)
                .header(header::CONTENT_TYPE, content_type)
                .header(header::USER_AGENT, user_agent)
                .body(body_serialized)
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
pub(crate) async fn request<Input: Serialize, Output: DeserializeOwned + 'static>(
    url: &str,
    apikey: &str,
    method: Method<Input>,
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
        Method::Get(query) => {
            let query = yaup::to_string(query)?;

            if !query.is_empty() {
                mut_url = format!("{}?{}", mut_url, query);
            };

            request.method("GET");
        }
        Method::Delete => {
            request.method("DELETE");
        }
        Method::Patch(body) => {
            let (content_type, body_serialized) = match body {
                Data::Iterable(body) => ("application/x-ndjson", to_jsonlines(body)),
                Data::NonIterable(body) => ("application/json", to_string(&body).unwrap()),
            };
            request.method("PATCH");
            headers.append(CONTENT_TYPE, content_type).unwrap();
            request.body(Some(&JsValue::from_str(&body_serialized)));
        }
        Method::Post(body) => {
            let (content_type, body_serialized) = match body {
                Data::Iterable(body) => ("application/x-ndjson", to_jsonlines(body)),
                Data::NonIterable(body) => ("application/json", to_string(&body).unwrap()),
            };
            request.method("POST");
            headers.append(CONTENT_TYPE, content_type).unwrap();
            request.body(Some(&JsValue::from_str(&body_serialized)));
        }
        Method::Put(body) => {
            let (content_type, body_serialized) = match body {
                Data::Iterable(body) => ("application/x-ndjson", to_jsonlines(body)),
                Data::NonIterable(body) => ("application/json", to_string(&body).unwrap()),
            };
            request.method("PUT");
            headers.append(CONTENT_TYPE, content_type).unwrap();
            request.body(Some(&JsValue::from_str(&body_serialized)));
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

fn to_jsonlines<T>(data: T) -> String
where
    T: IntoIterator,
    T::Item: Serialize,
{
    let mut jsonlines = data
        .into_iter()
        .map(|x| serde_json::to_string(&x).unwrap())
        .collect::<Vec<String>>()
        .join("\n");
    jsonlines.push('\n');
    jsonlines
}
