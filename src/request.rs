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
pub(crate) fn request<Input: Serialize + std::fmt::Debug, Output: 'static +  DeserializeOwned>(
    url: &str,
    apikey: &str,
    method: Method<Input>,
    expected_status_code: i32,
    callback: Box<dyn Fn(Result<Output, Error>)>
) {
    use web_sys::{RequestInit, Headers, Response};
    use wasm_bindgen::{JsValue, JsCast, prelude::*};
    use std::rc::Rc;

    // NOTE: Unwrap are not a big problem on web-sys objects

    let headers = Headers::new().unwrap();
    headers.append("X-Meili-API-Key", apikey).unwrap();

    let mut request: RequestInit = RequestInit::new();
    request.headers(&headers);

    match &method {
        Method::Get => {request.method("GET");},
        Method::Delete => {request.method("DELETE");},
        Method::Post(body) => {
            request.method("POST");
            request.body(Some(&JsValue::from_str(&to_string(body).unwrap())));
        }
        Method::Put(body) => {
            request.method("PUT");
            request.body(Some(&JsValue::from_str(&to_string(body).unwrap())));
        }
    }
    
    let callback = Rc::new(callback);
    let callback = Rc::clone(&callback);
    let window = web_sys::window().unwrap();
    let fetch_closure = Closure::wrap(Box::new(move |response: JsValue| {
        let response = Response::from(response);
        let status = response.status() as i32;

        let callback2 = Rc::clone(&callback);
        let text_promise = Closure::wrap(Box::new(move |text: JsValue| {
            if let Some(t) = text.as_string() {
                if t.is_empty() {
                    callback2(parse_response(status, expected_status_code, "null"))
                } else {
                    callback2(parse_response(status, expected_status_code, &t))
                }
            } else {
                callback2(Err(Error::Unknown("Invalid utf8".to_string())));
            }
        }) as Box<dyn FnMut(_)>);
    
        if let Ok(text) = response.text() {
            text.then(&text_promise);
            text_promise.forget();
        } else {
            callback(Err(Error::Unknown("Invalid utf8".to_string())));
        }
    }) as Box<dyn FnMut(_)>);
    window.fetch_with_str_and_init(url, &request).then(&fetch_closure);
    fetch_closure.forget();
}

fn parse_response<Output: DeserializeOwned>(status_code: i32, expected_status_code: i32, body: &str) -> Result<Output, Error> {
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