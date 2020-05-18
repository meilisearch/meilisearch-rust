use crate::errors::Error;
use log::{error, trace};
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
pub(crate) fn request<Input: Serialize + std::fmt::Debug, Output: DeserializeOwned>(
    url: &str,
    apikey: &str,
    method: Method<Input>,
    expected_status_code: i32,
) -> Result<Output, Error> {
    use minreq::{delete, get, post, put};

    let response = match &method {
        Method::Get => get(url).with_header("X-Meili-API-Key", apikey).send()?,
        Method::Delete => delete(url).with_header("X-Meili-API-Key", apikey).send()?,
        Method::Post(body) => post(url)
            .with_header("X-Meili-API-Key", apikey)
            .with_body(to_string(&body).unwrap())
            .send()?,
        Method::Put(body) => put(url)
            .with_header("X-Meili-API-Key", apikey)
            .with_body(to_string(&body).unwrap())
            .send()?,
    };

    let mut body = response.as_str()?;
    if body.is_empty() {
        body = "null";
    }

    parse_response(response.status_code, expected_status_code, body)
}

#[cfg(target_arch = "wasm32")]
pub(crate) async fn request<Input: Serialize + std::fmt::Debug, Output: 'static + DeserializeOwned>(
    url: &str,
    apikey: &str,
    method: Method<Input>,
    expected_status_code: i32
) -> Result<Output, Error> {
    use std::rc::Rc;
    use wasm_bindgen::{prelude::*, JsCast, JsValue};
    use web_sys::{Headers, RequestInit, Response};
    use wasm_bindgen_futures::JsFuture;

    // NOTE: Unwrap are not a big problem on web-sys objects

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
            request.body(Some(&JsValue::from_str(&to_string(body).unwrap())));
        }
        Method::Put(body) => {
            request.method("PUT");
            request.body(Some(&JsValue::from_str(&to_string(body).unwrap())));
        }
    }

    let window = web_sys::window().unwrap();
    let response = Response::from(JsFuture::from(window.fetch_with_str_and_init(url, &request)).await.unwrap());
    let status = response.status() as i32;
    let text = JsFuture::from(response.text().unwrap()).await.unwrap();

    if let Some(t) = text.as_string() {
        if t.is_empty() {
            parse_response(status, expected_status_code, "null")
        } else {
            parse_response(status, expected_status_code, &t)
        }
    } else {
        Err(Error::Unknown("Invalid utf8".to_string()))
    }

}

fn parse_response<Output: DeserializeOwned>(
    status_code: i32,
    expected_status_code: i32,
    body: &str,
) -> Result<Output, Error> {
    if status_code == expected_status_code {
        match from_str::<Output>(body) {
            Ok(output) => {
                return Ok(output);
            }
            Err(e) => {
                return Err(Error::from(e.to_string().as_str()));
            }
        };
    }
    Err(Error::from(body))
}
