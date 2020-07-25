use crate::errors::Error;
use log::{error, trace, warn};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, to_string};

#[derive(Debug)]
pub(crate) enum Method<T: Serialize> {
    Get,
    Post(T),
    Put(T),
    Delete,
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) async fn request<Input: Serialize + std::fmt::Debug, Output: 'static + DeserializeOwned>(
    url: &str,
    apikey: &str,
    method: Method<Input>,
    expected_status_code: u16
) -> Result<Output, Error> {
    trace!("{:?} on {}", method, url);

    let client = reqwest::Client::new();

    let response = match &method {
        Method::Get => client.get(url).header("X-Meili-API-Key", apikey).send().await?,
        Method::Delete => client.delete(url).header("X-Meili-API-Key", apikey).send().await?,
        Method::Post(body) => client.post(url)
            .header("X-Meili-API-Key", apikey)
            .header("Content-Type", "application/json")
            .body(to_string(&body).unwrap())
            .send().await?,
        Method::Put(body) => client.put(url)
            .header("X-Meili-API-Key", apikey)
            .header("Content-Type", "application/json")
            .body(to_string(&body).unwrap())
            .send().await?,
    };

    let status = response.status().as_u16();
    let mut body = response.text().await?;
    if body.is_empty() {
        body = "null".to_string();
    }

    parse_response(status, expected_status_code, body)
}

#[cfg(target_arch = "wasm32")]
pub(crate) async fn request<Input: Serialize + std::fmt::Debug, Output: 'static + DeserializeOwned>(
    url: &str,
    apikey: &str,
    method: Method<Input>,
    expected_status_code: u16
) -> Result<Output, Error> {
    use wasm_bindgen::JsValue;
    use web_sys::{Headers, RequestInit, Response};
    use wasm_bindgen_futures::JsFuture;

    trace!("{:?} on {}", method, url);

    // The 2 following unwraps should not be able to fail

    let headers = Headers::new().unwrap();
    headers.append("X-Meili-API-Key", apikey).unwrap();

    let mut request: RequestInit = RequestInit::new();
    request.headers(&headers);

    match &method {
        Method::Get => {
            request.method("GET");
        }
        Method::Delete => {
            request.method("DELETE");
        }
        Method::Post(body) => {
            request.method("POST");
            headers.append("Content-Type", "application/json").unwrap();
            request.body(Some(&JsValue::from_str(&to_string(body).unwrap())));
        }
        Method::Put(body) => {
            request.method("PUT");
            headers.append("Content-Type", "application/json").unwrap();
            request.body(Some(&JsValue::from_str(&to_string(body).unwrap())));
        }
    }

    let window = web_sys::window().unwrap(); // TODO remove this unwrap
    let response = match JsFuture::from(window.fetch_with_str_and_init(url, &request)).await {
        Ok(response) => Response::from(response),
        Err(e) => {
            error!("Network error: {:?}", e);
            return Err(Error::UnreachableServer)
        },
    };
    let status = response.status() as u16;
    let text = match response.text() {
        Ok(text) => match JsFuture::from(text).await {
            Ok(text) => text,
            Err(e) => {
                error!("Invalid response: {:?}", e);
                return Err(Error::Unknown("Invalid response".to_string()));
            }
        }
        Err(e) => {
            error!("Invalid response: {:?}", e);
            return Err(Error::Unknown("Invalid response".to_string()));
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
        Err(Error::Unknown("Invalid utf8".to_string()))
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
                error!("Request succeed but failed to parse response");
                return Err(Error::Unknown(e.to_string()));
            }
        };
    }
    warn!("Expected response code {}, got {}", expected_status_code, status_code);
    match from_str(&body) {
        Ok(e) => Err(Error::from(&e)),
        Err(e) => Err(Error::Unknown(e.to_string()))
    }
}
