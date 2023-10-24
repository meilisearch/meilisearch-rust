use log::error;
use meilisearch_sdk::{Error, RequestClient};
use serde::Serialize;
use strum::Display;
use thiserror::Error;
use url::Url;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, RequestInit};

const BROWSER_CONTEXT: &str = "not in a browser context";

pub struct BrowserRequestClient {
    url: String,
    headers: Headers,
    request: RequestInit,
}

#[async_trait::async_trait]
impl<'a, B: Serialize + 'a + Send> RequestClient<'a, B> for BrowserRequestClient {
    type Request = JsFuture;
    type Response = web_sys::Response;
    type HttpError = HttpError;

    fn new(url: Url) -> Self {
        Self {
            url: url.to_string(),
            headers: Headers::new().expect(BROWSER_CONTEXT),
            request: RequestInit::new(),
        }
    }

    fn append_header(mut self, name: http::HeaderName, value: http::HeaderValue) -> Self {
        self.headers
            .append(
                name.as_str(),
                value.to_str().expect("header valued not sanitized"),
            )
            .expect(BROWSER_CONTEXT);
        self.request.headers(&self.headers);
        self
    }

    fn with_method(mut self, http_method: http::Method) -> Self {
        self.request.method(http_method.as_str());
        self
    }

    fn add_body(mut self, body: Option<B>) -> Self::Request {
        if let Some(body) = body {
            self.request.body(Some(&JsValue::from_str(
                &serde_json::to_string(&body).expect(BROWSER_CONTEXT),
            )));
        }

        JsFuture::from(
            web_sys::window()
                .expect(BROWSER_CONTEXT)
                .fetch_with_str_and_init(&self.url, &self.request),
        )
    }

    async fn send_request(request: Self::Request) -> Result<Self::Response, Error> {
        request.await.map(Self::Response::from).map_err(|e| {
            error!("Network error: {:?}", e);
            Error::UnreachableServer
        })
    }

    fn extract_status_code(response: &Self::Response) -> u16 {
        response.status()
    }

    async fn response_to_text(response: Self::Response) -> Result<String, Error> {
        let text = response.text().map_err(invalid_response)?;
        let text = JsFuture::from(text).await.map_err(invalid_response)?;
        return text.as_string().ok_or_else(|| {
            error!("Invalid response");
            Error::from(HttpError::InvalidUtf8)
        });

        fn invalid_response(e: wasm_bindgen::JsValue) -> Error {
            error!("Invalid response: {:?}", e);
            Error::from(HttpError::InvalidResponse)
        }
    }
}

#[derive(Debug, Error, Display)]
#[strum(serialize_all = "snake_case")]
pub enum HttpError {
    InvalidResponse,
    InvalidUtf8,
}
impl From<HttpError> for Error {
    fn from(error: HttpError) -> Error {
        Error::Http(Box::new(error))
    }
}
