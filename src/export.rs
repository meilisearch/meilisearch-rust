//! Interact with the [`/export`](https://www.meilisearch.com/docs/reference/api/export) API.
//!
//! Export tasks let you migrate data from the current Meilisearch instance to a
//! remote instance without downloading a dump to disk.
//! The [`Client::create_export`](crate::client::Client::create_export) method
//! enqueues a new export task.
//!
//! # Example
//!
//! ```
//! # use meilisearch_sdk::{client::*, export::*, task_info::*, tasks::*};
//! # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
//! #
//! # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
//! # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
//! #
//! # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
//! let task_info = client
//!     .create_export(
//!         ExportPayload::new("https://ms-cloud.example.com")
//!             .with_api_key("integration_api_key")
//!             .with_payload_size("32MiB")
//!             .with_index(
//!                 "*",
//!                 ExportIndexOptions::new().with_filter(serde_json::json!("genres = action")),
//!             ),
//!     )
//!     .await
//!     .unwrap();
//!
//! assert!(matches!(
//!     task_info,
//!     TaskInfo {
//!         update_type: TaskType::Export { .. },
//!         ..
//!     }
//! ));
//! # });
//! ```

use std::collections::BTreeMap;

use serde::Serialize;
use serde_json::Value;

use crate::{client::Client, errors::Error, request::*, task_info::TaskInfo};

/// Payload sent to the `/export` endpoint.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportPayload {
    /// URL of the remote Meilisearch instance, including protocol.
    pub url: String,
    /// API key used to authenticate on the remote instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// Maximum payload size transferred in a single request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_size: Option<ExportPayloadSize>,
    /// Index patterns and their export configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexes: Option<BTreeMap<String, ExportIndexOptions>>,
}

impl ExportPayload {
    /// Creates a payload targeting the provided remote URL.
    #[must_use]
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            api_key: None,
            payload_size: None,
            indexes: None,
        }
    }

    /// Sets the API key used to authenticate against the remote Meilisearch instance.
    #[must_use]
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets the maximum payload size transferred per HTTP request.
    #[must_use]
    pub fn with_payload_size<T: Into<ExportPayloadSize>>(mut self, payload_size: T) -> Self {
        self.payload_size = Some(payload_size.into());
        self
    }

    /// Replaces the configured index patterns.
    #[must_use]
    pub fn with_indexes(mut self, indexes: BTreeMap<String, ExportIndexOptions>) -> Self {
        self.indexes = if indexes.is_empty() {
            None
        } else {
            Some(indexes)
        };
        self
    }

    /// Adds or replaces a single index pattern configuration.
    #[must_use]
    pub fn with_index(mut self, pattern: impl Into<String>, settings: ExportIndexOptions) -> Self {
        self.indexes
            .get_or_insert_with(BTreeMap::new)
            .insert(pattern.into(), settings);
        self
    }
}

/// Export configuration for a specific index pattern.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExportIndexOptions {
    /// Optional filter limiting exported documents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Value>,
    /// Whether to override target settings with the origin settings.
    #[serde(default, skip_serializing_if = "crate::export::is_false")]
    pub override_settings: bool,
}

impl ExportIndexOptions {
    /// Creates default index export options.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the filter used to select exported documents.
    #[must_use]
    pub fn with_filter(mut self, filter: Value) -> Self {
        self.filter = Some(filter);
        self
    }

    /// Toggles whether to override the remote index settings.
    #[must_use]
    pub fn with_override_settings(mut self, override_settings: bool) -> Self {
        self.override_settings = override_settings;
        self
    }
}

/// Represents the payload size allowed per export batch.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ExportPayloadSize {
    /// Payload size expressed in bytes.
    Bytes(u64),
    /// Human readable payload size following Meilisearch conventions (for example `"32MiB"`).
    HumanReadable(String),
}

impl From<u64> for ExportPayloadSize {
    fn from(value: u64) -> Self {
        ExportPayloadSize::Bytes(value)
    }
}

impl From<String> for ExportPayloadSize {
    fn from(value: String) -> Self {
        ExportPayloadSize::HumanReadable(value)
    }
}

impl From<&str> for ExportPayloadSize {
    fn from(value: &str) -> Self {
        ExportPayloadSize::HumanReadable(value.to_string())
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_false(value: &bool) -> bool {
    !*value
}

/// Export related methods.
impl<Http: HttpClient> Client<Http> {
    /// Enqueues an export task.
    ///
    /// The created task can be tracked through the [`TaskInfo`] it returns and
    /// later through the [`tasks`](crate::tasks) endpoints.
    pub async fn create_export(&self, payload: ExportPayload) -> Result<TaskInfo, Error> {
        self.http_client
            .request::<(), ExportPayload, TaskInfo>(
                &format!("{}/export", self.host),
                Method::Post {
                    query: (),
                    body: payload,
                },
                202,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{client::*, errors::Error, tasks::*};
    use mockito::Matcher;
    use serde_json::json;

    #[cfg(feature = "reqwest")]
    #[tokio::test]
    async fn test_create_export_returns_task() -> Result<(), Error> {
        let mut server = mockito::Server::new_async().await;
        let base = server.url();

        let response = json!({
            "enqueuedAt": "2024-01-01T00:00:00.000Z",
            "status": "enqueued",
            "taskUid": 1,
            "type": "export",
            "details": {
                "url": "https://ms-cloud.example.com"
            }
        })
        .to_string();

        let _mock = server
            .mock("POST", "/export")
            .match_header("authorization", "Bearer masterKey")
            .match_header("content-type", "application/json")
            .match_body(Matcher::Json(json!({
                "url": "https://ms-cloud.example.com"
            })))
            .with_status(202)
            .with_body(response)
            .create_async()
            .await;

        let client = Client::new(base, Some("masterKey")).unwrap();
        let task_info = client
            .create_export(ExportPayload::new("https://ms-cloud.example.com"))
            .await?;

        assert!(matches!(task_info.update_type, TaskType::Export { .. }));

        Ok(())
    }

    #[cfg(feature = "reqwest")]
    #[tokio::test]
    async fn test_create_export_with_index_configuration() -> Result<(), Error> {
        let mut server = mockito::Server::new_async().await;
        let base = server.url();

        let response = json!({
            "enqueuedAt": "2024-01-01T00:00:00.000Z",
            "status": "enqueued",
            "taskUid": 2,
            "type": "export",
            "details": {
                "url": "https://ms-cloud.example.com",
                "indexes": {
                    "movies": {
                        "filter": "genres = action",
                        "overrideSettings": true,
                        "matchedDocuments": null
                    }
                }
            }
        })
        .to_string();

        let expected_body = json!({
            "url": "https://ms-cloud.example.com",
            "apiKey": "integration_api_key",
            "payloadSize": 1_048_576_u64,
            "indexes": {
                "movies": {
                    "filter": "genres = action",
                    "overrideSettings": true
                }
            }
        });

        let _mock = server
            .mock("POST", "/export")
            .match_header("authorization", "Bearer masterKey")
            .match_header("content-type", "application/json")
            .match_body(Matcher::Json(expected_body.clone()))
            .with_status(202)
            .with_body(response)
            .create_async()
            .await;

        let client = Client::new(base, Some("masterKey")).unwrap();
        let payload = ExportPayload::new("https://ms-cloud.example.com")
            .with_api_key("integration_api_key")
            .with_payload_size(1_048_576_u64)
            .with_index(
                "movies",
                ExportIndexOptions::new()
                    .with_filter(json!("genres = action"))
                    .with_override_settings(true),
            );

        let task_info = client.create_export(payload).await?;

        assert!(matches!(task_info.update_type, TaskType::Export { .. }));

        Ok(())
    }
}
