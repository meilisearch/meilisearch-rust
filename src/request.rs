use crate::errors::{Error, MeilisearchError};
use log::{error, trace, warn};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, to_string};

#[derive(Debug)]
pub(crate) enum Method<T: Serialize> {
    Get,
    Post(T),
    Patch(T),
    Put(T),
    Delete,
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) async fn request<Input: Serialize, Output: DeserializeOwned + 'static>(
    url: &str,
    apikey: Option<&str>,
    method: Method<Input>,
    expected_status_code: u16,
) -> Result<Output, Error> {
    use isahc::http::header;
    use isahc::*;

    let user_agent = qualified_version();

    let mut response = match &method {
        Method::Get => {
            let mut request = Request::get(url)
              .header(header::USER_AGENT, user_agent);
            
            if let Some(key) = apikey {
                let auth = format!("Bearer {}", key);
                request = request.header(header::AUTHORIZATION, auth);
            }
            request
                    .body(())
                    .map_err(|_| crate::errors::Error::InvalidRequest)?
                    .send_async()
                    .await?
            }
        }
        Method::Delete => {
            if let Some(key) = apikey {
                let auth = format!("Bearer {}", key);
                Request::delete(url)
                    .header(header::AUTHORIZATION, auth)
                    .header(header::USER_AGENT, user_agent)
                    .body(())
                    .map_err(|_| crate::errors::Error::InvalidRequest)?
                    .send_async()
                    .await?
            } else {
                Request::delete(url)
                    .header(header::USER_AGENT, user_agent)
                    .body(())
                    .map_err(|_| crate::errors::Error::InvalidRequest)?
                    .send_async()
                    .await?
            }
        }
        Method::Post(body) => {
            if let Some(key) = apikey {
                let auth = format!("Bearer {}", key);
                Request::post(url)
                    .header(header::AUTHORIZATION, auth)
                    .header(header::CONTENT_TYPE, "application/json")
                    .header(header::USER_AGENT, user_agent)
                    .body(to_string(&body).unwrap())
                    .map_err(|_| crate::errors::Error::InvalidRequest)?
                    .send_async()
                    .await?
            } else {
                Request::post(url)
                    .header(header::CONTENT_TYPE, "application/json")
                    .header(header::USER_AGENT, user_agent)
                    .body(to_string(&body).unwrap())
                    .map_err(|_| crate::errors::Error::InvalidRequest)?
                    .send_async()
                    .await?
            }
        }
        Method::Patch(body) => {
            if let Some(key) = apikey {
                let auth = format!("Bearer {}", key);
                Request::patch(url)
                    .header(header::AUTHORIZATION, auth)
                    .header(header::CONTENT_TYPE, "application/json")
                    .header(header::USER_AGENT, user_agent)
                    .body(to_string(&body).unwrap())
                    .map_err(|_| crate::errors::Error::InvalidRequest)?
                    .send_async()
                    .await?
            } else {
                Request::patch(url)
                    .header(header::CONTENT_TYPE, "application/json")
                    .header(header::USER_AGENT, user_agent)
                    .body(to_string(&body).unwrap())
                    .map_err(|_| crate::errors::Error::InvalidRequest)?
                    .send_async()
                    .await?
            }
        }
        Method::Put(body) => {
            if let Some(key) = apikey {
                let auth = format!("Bearer {}", key);
                Request::put(url)
                    .header(header::AUTHORIZATION, auth)
                    .header(header::CONTENT_TYPE, "application/json")
                    .header(header::USER_AGENT, user_agent)
                    .body(to_string(&body).unwrap())
                    .map_err(|_| crate::errors::Error::InvalidRequest)?
                    .send_async()
                    .await?
            } else {
                Request::put(url)
                    .header(header::CONTENT_TYPE, "application/json")
                    .header(header::USER_AGENT, user_agent)
                    .body(to_string(&body).unwrap())
                    .map_err(|_| crate::errors::Error::InvalidRequest)?
                    .send_async()
                    .await?
            }
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
    apikey: Option<&str>,
    method: Method<Input>,
    expected_status_code: u16,
) -> Result<Output, Error> {
    use wasm_bindgen::JsValue;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{Headers, RequestInit, Response};

    const CONTENT_TYPE: &str = "Content-Type";
    const JSON: &str = "application/json";
    let user_agent = qualified_version();

    // The 2 following unwraps should not be able to fail

    let headers = Headers::new().unwrap();

    if let Some(key) = apikey {
        headers.append("Authorization: Bearer", key).unwrap();
    }
    headers.append("User-Agent", &user_agent).unwrap();

    let mut request: RequestInit = RequestInit::new();
    request.headers(&headers);

    match &method {
        Method::Get => {
            request.method("GET");
        }
        Method::Delete => {
            request.method("DELETE");
        }
        Method::Patch(body) => {
            request.method("PATCH");
            headers.append(CONTENT_TYPE, JSON).unwrap();
            request.body(Some(&JsValue::from_str(&to_string(body).unwrap())));
        }
        Method::Post(body) => {
            request.method("POST");
            headers.append(CONTENT_TYPE, JSON).unwrap();
            request.body(Some(&JsValue::from_str(&to_string(body).unwrap())));
        }
        Method::Put(body) => {
            request.method("PUT");
            headers.append(CONTENT_TYPE, JSON).unwrap();
            request.body(Some(&JsValue::from_str(&to_string(body).unwrap())));
        }
    }

    let window = web_sys::window().unwrap(); // TODO remove this unwrap
    let response = match JsFuture::from(window.fetch_with_str_and_init(url, &request)).await {
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
