use std::marker::PhantomData;

use reqwest::{Client, Request, RequestBuilder, Response};
use url::Url;

use crate::Error;

use self::body_transform::BodyTransform;

use super::RequestClient;

lazy_static::lazy_static! {
    pub static ref CLIENT: Client = {
        Client::new()
    };
}

pub struct ReqwestClient<T: BodyTransform<B>, B>(Request, PhantomData<T>, PhantomData<B>);

#[async_trait::async_trait]
impl<'a, B: 'a + Send, T: BodyTransform<B>> RequestClient<'a, B> for ReqwestClient<T, B> {
    type Request = Request;
    type Response = Response;

    fn new(url: Url) -> Self {
        Self(
            Request::new(http::Method::GET, url),
            PhantomData,
            PhantomData,
        )
    }

    fn append_header(mut self, name: http::HeaderName, value: http::HeaderValue) -> Self {
        let headers = self.0.headers_mut();
        headers.insert(name, value);
        self
    }

    fn with_method(mut self, http_method: http::Method) -> Self {
        *self.0.method_mut() = http_method;
        self
    }

    fn add_body(mut self, body: Option<B>) -> Self::Request {
        if let Some(body) = body {
            *self.0.body_mut() = Some(T::body_transform(body));
        }
        self.0
    }

    async fn send_request(request: Self::Request) -> Result<Self::Response, Error> {
        RequestBuilder::from_parts(CLIENT.clone(), request)
            .send()
            .await
            .map_err(Error::from)
    }

    fn extract_status_code(response: &Self::Response) -> u16 {
        response.status().as_u16()
    }

    async fn response_to_text(response: Self::Response) -> Result<String, Error> {
        response.text().await.map_err(Error::from)
    }
}

pub(super) use body_transform::{ReadBodyTransform, SerializeBodyTransform};
mod body_transform {
    use std::pin::pin;

    use futures::AsyncReadExt;
    use reqwest::Body;
    use serde::Serialize;

    pub trait BodyTransform<B> {
        fn body_transform(body: B) -> Body;
    }

    pub struct SerializeBodyTransform;
    impl<B: Serialize> BodyTransform<B> for SerializeBodyTransform {
        fn body_transform(body: B) -> Body {
            Body::from(serde_json::to_string(&body).expect("unable to serialize body"))
        }
    }

    pub struct ReadBodyTransform;
    impl<B: futures_io::AsyncRead + Send + Sync> BodyTransform<B> for ReadBodyTransform {
        fn body_transform(body: B) -> Body {
            let bytes = futures::executor::block_on(async move {
                let mut output = Vec::new();
                let mut body = pin!(body);
                body.read_to_end(&mut output)
                    .await
                    .expect("unable to read entrire request body");
                output
            });

            Body::from(bytes)
        }
    }
}
