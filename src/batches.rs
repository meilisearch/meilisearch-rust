use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{client::Client, errors::Error, request::HttpClient};

/// Types and queries for the Meilisearch Batches API.
///
/// See: https://www.meilisearch.com/docs/reference/api/batches
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Batch {
    /// Unique identifier of the batch.
    #[serde(default)]
    pub uid: u32,
    /// When the batch was enqueued.
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub enqueued_at: Option<OffsetDateTime>,
    /// When the batch started processing.
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub started_at: Option<OffsetDateTime>,
    /// When the batch finished processing.
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub finished_at: Option<OffsetDateTime>,
    /// Index uid related to this batch (if applicable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_uid: Option<String>,
    /// The task uids that are part of this batch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_uids: Option<Vec<u32>>,
    /// The strategy that caused the autobatcher to stop batching tasks.
    ///
    /// Introduced in Meilisearch v1.15.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_strategy: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchesResults {
    pub results: Vec<Batch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<u32>,
}

/// Query builder for listing batches.
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BatchesQuery<'a, Http: HttpClient> {
    #[serde(skip_serializing)]
    client: &'a Client<Http>,
    /// Maximum number of batches to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    /// The first batch uid that should be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<u32>,
}

impl<'a, Http: HttpClient> BatchesQuery<'a, Http> {
    #[must_use]
    pub fn new(client: &'a Client<Http>) -> BatchesQuery<'a, Http> {
        BatchesQuery {
            client,
            limit: None,
            from: None,
        }
    }

    #[must_use]
    pub fn with_limit(&mut self, limit: u32) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    #[must_use]
    pub fn with_from(&mut self, from: u32) -> &mut Self {
        self.from = Some(from);
        self
    }

    /// Execute the query and list batches.
    pub async fn execute(&self) -> Result<BatchesResults, Error> {
        self.client.get_batches_with(self).await
    }
}

#[cfg(test)]
mod tests {
    use crate::client::Client;

    #[tokio::test]
    async fn test_get_batches_parses_batch_strategy() {
        let mut s = mockito::Server::new_async().await;
        let base = s.url();

        let response_body = serde_json::json!({
            "results": [
                {
                    "uid": 42,
                    "enqueuedAt": "2024-10-11T11:49:53.000Z",
                    "startedAt": "2024-10-11T11:49:54.000Z",
                    "finishedAt": "2024-10-11T11:49:55.000Z",
                    "indexUid": "movies",
                    "taskUids": [1, 2, 3],
                    "batchStrategy": "time_limit_reached"
                }
            ],
            "limit": 20,
            "from": null,
            "next": null,
            "total": 1
        })
        .to_string();

        let _m = s
            .mock("GET", "/batches")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(response_body)
            .create_async()
            .await;

        let client = Client::new(base, None::<String>).unwrap();
        let batches = client.get_batches().await.expect("list batches failed");
        assert_eq!(batches.results.len(), 1);
        let b = &batches.results[0];
        assert_eq!(b.uid, 42);
        assert_eq!(b.batch_strategy.as_deref(), Some("time_limit_reached"));
    }

    #[tokio::test]
    async fn test_get_batch_by_uid_parses_batch_strategy() {
        let mut s = mockito::Server::new_async().await;
        let base = s.url();

        let response_body = serde_json::json!({
            "uid": 99,
            "batchStrategy": "size_limit_reached",
            "taskUids": [10, 11]
        })
        .to_string();

        let _m = s
            .mock("GET", "/batches/99")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(response_body)
            .create_async()
            .await;

        let client = Client::new(base, None::<String>).unwrap();
        let batch = client.get_batch(99).await.expect("get batch failed");
        assert_eq!(batch.uid, 99);
        assert_eq!(batch.batch_strategy.as_deref(), Some("size_limit_reached"));
    }
}
