use std::{
    pin::Pin,
    task::{Context, Poll},
};

use async_trait::async_trait;
use bytes::{Bytes, BytesMut};
use futures_core::Stream;
use futures_io::AsyncRead;
use pin_project_lite::pin_project;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    errors::Error,
    request::{parse_response, HttpClient, Method},
};

#[derive(Debug, Clone, Default)]
pub struct ReqwestClient {
    client: reqwest::Client,
}

impl ReqwestClient {
    pub fn new(api_key: Option<&str>) -> Result<Self, Error> {
        use reqwest::{header, ClientBuilder};

        let builder = ClientBuilder::new();
        let mut headers = header::HeaderMap::new();
        #[cfg(not(target_arch = "wasm32"))]
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str(&qualified_version()).unwrap(),
        );
        #[cfg(target_arch = "wasm32")]
        headers.insert(
            header::HeaderName::from_static("x-meilisearch-client"),
            header::HeaderValue::from_str(&qualified_version()).unwrap(),
        );

        if let Some(api_key) = api_key {
            headers.insert(
                header::AUTHORIZATION,
                header::HeaderValue::from_str(&format!("Bearer {api_key}")).unwrap(),
            );
        }

        let builder = builder.default_headers(headers);
        let client = builder.build()?;

        Ok(ReqwestClient { client })
    }

    pub(crate) fn inner(&self) -> &reqwest::Client {
        &self.client
    }
}

#[cfg_attr(feature = "futures-unsend", async_trait(?Send))]
#[cfg_attr(not(feature = "futures-unsend"), async_trait)]
impl HttpClient for ReqwestClient {
    async fn stream_request<
        Query: Serialize + Send + Sync,
        Body: futures_io::AsyncRead + Send + Sync + 'static,
        Output: DeserializeOwned + 'static,
    >(
        &self,
        url: &str,
        method: Method<Query, Body>,
        content_type: &str,
        expected_status_code: u16,
    ) -> Result<Output, Error> {
        use reqwest::header;

        let query = method.query();
        let query = yaup::to_string(query)?;

        let url = if query.is_empty() {
            url.to_string()
        } else {
            format!("{url}{query}")
        };

        let mut request = self.client.request(verb(&method), &url);

        if let Some(body) = method.into_body() {
            // TODO: Currently reqwest doesn't support streaming data in wasm so we need to collect everything in RAM
            #[cfg(not(target_arch = "wasm32"))]
            {
                let stream = ReaderStream::new(body);
                let body = reqwest::Body::wrap_stream(stream);

                request = request
                    .header(header::CONTENT_TYPE, content_type)
                    .body(body);
            }
            #[cfg(target_arch = "wasm32")]
            {
                use futures_util::AsyncReadExt;

                let mut buf = Vec::new();
                let mut body = std::pin::pin!(body);
                body.read_to_end(&mut buf)
                    .await
                    .map_err(|err| Error::Other(Box::new(err)))?;
                request = request.header(header::CONTENT_TYPE, content_type).body(buf);
            }
        }

        let response = self.client.execute(request.build()?).await?;
        let status = response.status().as_u16();
        let mut body = response.text().await?;

        if body.is_empty() {
            body = "null".to_string();
        }

        parse_response(status, expected_status_code, &body, url.to_string())
    }

    fn is_tokio(&self) -> bool {
        true
    }
}

fn verb<Q, B>(method: &Method<Q, B>) -> reqwest::Method {
    match method {
        Method::Get { .. } => reqwest::Method::GET,
        Method::Delete { .. } => reqwest::Method::DELETE,
        Method::Post { .. } => reqwest::Method::POST,
        Method::Put { .. } => reqwest::Method::PUT,
        Method::Patch { .. } => reqwest::Method::PATCH,
    }
}

pub fn qualified_version() -> String {
    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

    format!("Meilisearch Rust (v{})", VERSION.unwrap_or("unknown"))
}

pin_project! {
    #[derive(Debug)]
    pub struct ReaderStream<R: AsyncRead> {
        #[pin]
        reader: R,
        buf: BytesMut,
        capacity: usize,
    }
}

impl<R: AsyncRead> ReaderStream<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf: BytesMut::new(),
            // 8KiB of capacity, the default capacity used by `BufReader` in the std
            capacity: 8 * 1024,
        }
    }
}

impl<R: AsyncRead> Stream for ReaderStream<R> {
    type Item = std::io::Result<Bytes>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.as_mut().project();

        if this.buf.capacity() == 0 {
            this.buf.resize(*this.capacity, 0);
        }

        match AsyncRead::poll_read(this.reader, cx, this.buf) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(err)) => Poll::Ready(Some(Err(err))),
            Poll::Ready(Ok(0)) => Poll::Ready(None),
            Poll::Ready(Ok(i)) => {
                let chunk = this.buf.split_to(i);
                Poll::Ready(Some(Ok(chunk.freeze())))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use futures::{io::Cursor, StreamExt};

    use super::ReaderStream;

    #[tokio::test]
    async fn request_bodies_are_read_through_an_8_kib_buffer() {
        let chunk = ReaderStream::new(Cursor::new(vec![1; 16 * 1024]))
            .next()
            .await
            .expect("first chunk")
            .expect("successful read");

        assert_eq!(chunk.len(), 8 * 1024);
    }

    #[tokio::test]
    async fn request_bodies_preserve_all_bytes_across_buffer_boundaries() {
        for size in [0, 1, 8191, 8192, 8193, 16384, 16385, 8 * 1024 * 1024 + 1] {
            let payload: Vec<u8> = (0..=250).cycle().take(size).collect();
            let mut stream = ReaderStream::new(Cursor::new(payload.clone()));
            let mut received = Vec::new();

            while let Some(chunk) = stream.next().await {
                received.extend_from_slice(&chunk.expect("successful read"));
            }

            assert_eq!(received, payload, "body size: {size}");
            assert!(stream.next().await.is_none());
        }
    }
}
