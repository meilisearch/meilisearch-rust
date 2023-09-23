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
        Method::Get { .. } => {
            request.method("GET");
        }
        Method::Delete { .. } => {
            request.method("DELETE");
        }
        Method::Patch { body, .. } => {
            request.method("PATCH");
            headers.append(CONTENT_TYPE, JSON).unwrap();
            request.body(Some(&JsValue::from_str(&to_string(body).unwrap())));
        }
        Method::Post { body, .. } => {
            request.method("POST");
            headers.append(CONTENT_TYPE, JSON).unwrap();
            request.body(Some(&JsValue::from_str(&to_string(body).unwrap())));
        }
        Method::Put { body, .. } => {
            request.method("PUT");
            headers.append(CONTENT_TYPE, JSON).unwrap();
            request.body(Some(&JsValue::from_str(&to_string(body).unwrap())));
        }
    }

    let response = JsFuture::from(
        web_sys::window()
            .expect("browser context")
            .fetch_with_str_and_init(&add_query_parameters(url, method.query())?, &request),
    )
    .await
    .map(Response::from)
    .map_err(|e| {
        error!("Network error: {:?}", e);
        Error::UnreachableServer
    })?;

    let status = response.status() as u16;

    let text = response.text().map_err(invalid_response)?;
    let text = JsFuture::from(text).await.map_err(invalid_response)?;
    let text = text.as_string().ok_or_else(|| {
        error!("Invalid response");
        Error::HttpError("Invalid utf8".to_string())
    })?;

    parse_response(status, expected_status_code, &text, url.to_string())
}

fn invalid_response(e: wasm_bindgen::JsValue) -> Error {
    error!("Invalid response: {:?}", e);
    Error::HttpError("Invalid response".to_string())
}
