use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, RequestInit};

use super::*;

const BROWSER_CONTEXT: &str = "not in a browser context";

pub struct BrowserRequestClient {
    url: String,
    headers: Headers,
    request: RequestInit,
}

impl<B0: Serialize> RequestClient<B0> for BrowserRequestClient {
    type Request = JsFuture;
    type Response = web_sys::Response;

    fn new(url: String) -> Self {
        Self {
            url,
            headers: Headers::new().expect(BROWSER_CONTEXT),
            request: RequestInit::new(),
        }
    }

    fn with_authorization_header(mut self, bearer_token_value: &str) -> Self {
        self.headers
            .append("Authorization", bearer_token_value)
            .expect(BROWSER_CONTEXT);
        self.request.headers(&self.headers);
        self
    }

    fn with_user_agent_header(mut self, user_agent_value: &str) -> Self {
        self.headers
            .append("X-Meilisearch-Client", user_agent_value)
            .expect(BROWSER_CONTEXT);
        self.request.headers(&self.headers);
        self
    }

    fn with_method(mut self, http_method: http::Method) -> Self {
        self.request.method(http_method.as_str());
        self
    }

    fn add_body<Q>(mut self, method: Method<Q, B0>, content_type: &str) -> Self::Request {
        match &method {
            Method::Patch { body, .. } | Method::Put { body, .. } | Method::Post { body, .. } => {
                self.headers
                    .append("Content-Type", content_type)
                    .expect(BROWSER_CONTEXT);
                self.request.headers(&self.headers);
                self.request.body(Some(&JsValue::from_str(
                    &serde_json::to_string(&body).expect(BROWSER_CONTEXT),
                )));
            }
            _ => (),
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
        response.status() as u16
    }

    async fn response_to_text(response: Self::Response) -> Result<String, Error> {
        let text = response.text().map_err(invalid_response)?;
        let text = JsFuture::from(text).await.map_err(invalid_response)?;
        return text.as_string().ok_or_else(|| {
            error!("Invalid response");
            Error::HttpError("Invalid utf8".to_string())
        });

        fn invalid_response(e: wasm_bindgen::JsValue) -> Error {
            error!("Invalid response: {:?}", e);
            Error::HttpError("Invalid response".to_string())
        }
    }
}
