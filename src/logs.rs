use crate::client::Client;
use crate::errors::Error;
use bytes::Bytes;
use futures_core::Stream;
use serde::Serialize;

#[derive(Serialize)]
pub struct NewLogLevel {
    pub target: String,
}

#[derive(Serialize)]
pub struct LogStreamRequest {
    pub target: String,
    pub mode: LogMode,
}

#[derive(Debug, Default, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum LogMode {
    /// Output the logs in a human-readable form
    #[default]
    Human,
    /// Output the logs in JSON format
    Json,
    /// Output the logs in Firefox profiler format for visualization
    Profile,
}

impl Client<crate::reqwest::ReqwestClient> {
    /// Opens a continuous stream of logs for focused debugging sessions.
    /// The stream will continue to run indefinitely until you
    /// [interrupt](Client::interrupt_log_stream) it.
    ///
    /// The function returns a [`Stream`] that you can iterate over with
    ///  the [`StreamExt`] trait to process the logs.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, logs::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// let logs_config = LogStreamRequest {
    ///                     target: "info".to_string(),
    ///                     mode: LogMode::Human
    /// };
    /// let byte_stream = client.open_log_stream(logs_config).await.unwrap();
    ///# });
    /// ```
    pub async fn open_log_stream(
        &self,
        get_logs: LogStreamRequest,
    ) -> Result<impl Stream<Item = Result<Bytes, reqwest::Error>>, Error> {
        let res = self
            .http_client
            .inner()
            .post(format!("{}/logs/stream", self.host))
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&get_logs)?)
            .send()
            .await?;
        res.error_for_status_ref()?;
        Ok(res.bytes_stream())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use meilisearch_test_macro::meilisearch_test;
    #[meilisearch_test]
    async fn test_open_log_stream(client: Client) {
        let logs_config = LogStreamRequest {
            mode: LogMode::Human,
            target: "info".to_string(),
        };
        assert!(client.open_log_stream(logs_config).await.is_ok());
        assert!(client.interrupt_log_stream().await.is_ok());
    }
}
