use std::marker::PhantomData;

use isahc::{
    http::{header, request::Builder},
    AsyncBody, AsyncReadResponseExt, Request, RequestExt, Response,
};

use self::body_transform::{BodyTransform, ReadBodyTransform};

use super::*;

pub(super) use body_transform::SerializeBodyTransform;
mod body_transform {
    use isahc::AsyncBody;
    use serde::Serialize;

    pub trait BodyTransform<B> {
        fn body_transform(body: B) -> AsyncBody;
    }

    pub struct SerializeBodyTransform;
    impl<B: Serialize> BodyTransform<B> for SerializeBodyTransform {
        fn body_transform(body: B) -> AsyncBody {
            AsyncBody::from_bytes_static(
                serde_json::to_string(&body).expect("unable to serialize body"),
            )
        }
    }

    pub struct ReadBodyTransform;
    impl<B: futures_io::AsyncRead + Send + Sync + 'static> BodyTransform<B> for ReadBodyTransform {
        fn body_transform(body: B) -> AsyncBody {
            AsyncBody::from_reader(body)
        }
    }
}

pub struct NativeRequestClient<T: BodyTransform<B>, B>(Builder, PhantomData<T>, PhantomData<B>);

impl<B0, T: BodyTransform<B0>> RequestClient<B0> for NativeRequestClient<T, B0> {
    type Request = Result<Request<AsyncBody>, isahc::http::Error>;
    type Response = Response<AsyncBody>;

    fn new(url: String) -> Self {
        Self(Builder::new().uri(url), PhantomData, PhantomData)
    }

    fn with_authorization_header(mut self, bearer_token_value: &str) -> Self {
        self.0 = self.0.header(header::AUTHORIZATION, bearer_token_value);
        self
    }

    fn with_user_agent_header(mut self, user_agent_value: &str) -> Self {
        self.0 = self.0.header(header::USER_AGENT, user_agent_value);
        self
    }

    fn with_method(mut self, http_method: http::Method) -> Self {
        self.0 = self.0.method(http_method);
        self
    }

    fn add_body<Q>(self, method: Method<Q, B0>, content_type: &str) -> Self::Request {
        match method {
            Method::Put { body, .. } | Method::Post { body, .. } | Method::Patch { body, .. } => {
                self.0
                    .header(header::CONTENT_TYPE, content_type)
                    .body(T::body_transform(body))
            }
            _ => self.0.body(AsyncBody::empty()),
        }
    }

    async fn send_request(request: Self::Request) -> Result<Self::Response, Error> {
        request
            .map_err(|_| crate::errors::Error::InvalidRequest)?
            .send_async()
            .await
            .map_err(Error::from)
    }

    fn extract_status_code(response: &Self::Response) -> u16 {
        response.status().as_u16()
    }

    async fn response_to_text(mut response: Self::Response) -> Result<String, Error> {
        response
            .text()
            .await
            .map_err(|e| Error::HttpError(isahc::Error::from(e)))
    }
}

pub(crate) async fn stream_request<
    Q: Serialize,
    B: futures_io::AsyncRead + Send + Sync + 'static,
    Output: DeserializeOwned + 'static,
>(
    url: &str,
    apikey: Option<&str>,
    method: Method<Q, B>,
    content_type: &str,
    expected_status_code: u16,
) -> Result<Output, Error> {
    NativeRequestClient::<ReadBodyTransform, _>::request(
        url,
        apikey,
        method,
        content_type,
        expected_status_code,
    )
    .await
}
