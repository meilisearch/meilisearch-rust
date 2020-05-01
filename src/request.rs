use crate::errors::Error;
use log::{debug, error, trace};
use minreq::{delete, get, post, put};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, to_string};

#[derive(Debug)]
pub(crate) enum Method<T: Serialize> {
    Get,
    Post(T),
    Put(T),
    Delete,
}

pub(crate) fn request<Input: Serialize + std::fmt::Debug, Output: DeserializeOwned>(
    url: &str,
    apikey: &str,
    method: Method<Input>,
    expected_status_code: i32,
) -> Result<Output, Error> {
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
    if response.status_code == expected_status_code {
        if let Ok(output) = from_str::<Output>(body) {
            trace!(
                "Request Succeed\nurl: {},\nmethod: {:?},\nstatus code: {}\nbody: {}\n",
                url,
                method,
                response.status_code,
                body
            );
            return Ok(from_str::<Output>(body).unwrap());
        }
    }

    error!(
        "Failed request\nurl: {},\nmethod: {:?},\nstatus code: {}\nbody: {}\n",
        url, method, response.status_code, body
    );
    Err(Error::from(response.as_str()?))
}
