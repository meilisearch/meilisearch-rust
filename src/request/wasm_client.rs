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
    use wasm_bindgen::JsValue;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{Headers, RequestInit, Response};

    const CONTENT_TYPE: &str = "Content-Type";
    const JSON: &str = "application/json";

    // The 2 following unwraps should not be able to fail
    let mut mut_url = url.to_string();
    let headers = Headers::new().unwrap();
    if let Some(apikey) = apikey {
        headers
            .append("Authorization", format!("Bearer {}", apikey).as_str())
            .unwrap();
    }
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
            parse_response(status, expected_status_code, "null", url.to_string())
        } else {
            parse_response(status, expected_status_code, &t, url.to_string())
        }
    } else {
        error!("Invalid response");
        Err(Error::HttpError("Invalid utf8".to_string()))
    }
}
