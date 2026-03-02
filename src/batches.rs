use crate::{client::Client, errors::Error, request::HttpClient};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::OffsetDateTime;

/// Types and queries for the Meilisearch Batches API.
///
/// See: https://www.meilisearch.com/docs/reference/api/batches
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Batch {
    /// Unique identifier of the batch.
    pub uid: i64,
    /// Batch progress.
    pub progress: Option<BatchProgress>,
    /// Batch stats.
    pub stats: BatchStats,
    /// The total elapsed time the batch spent in the processing state, in ISO 8601 format.
    pub duration: Option<String>,
    /// When the batch started processing.
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub started_at: Option<OffsetDateTime>,
    /// When the batch finished processing.
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub finished_at: Option<OffsetDateTime>,
    /// The strategy that caused the autobatcher to stop batching tasks.
    /// Introduced in Meilisearch v1.15.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_strategy: Option<BatchStrategy>,
}

/// Reason why the auto batcher stopped batching tasks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum BatchStrategy {
    /// The batch reached its configured size threshold.
    SizeLimitReached,
    /// The batch reached its configured time window threshold.
    TimeLimitReached,
    /// Unknown strategy (forward-compatibility).
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchesResults {
    pub results: Vec<Batch>,
    pub total: u32,
    pub limit: u32,
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
    ///Select batches containing the tasks with the specified uids.
    /// Separate multiple task uids with a comma
    #[serde(skip_serializing_if = "Vec::is_empty")]
    uids: Vec<i64>,
    /// Filter batches by their uid. Separate multiple batch uids with a comma
    #[serde(skip_serializing_if = "Vec::is_empty")]
    batch_uids: Vec<i64>,
    /// Select batches containing tasks affecting the specified indexes.
    /// Separate multiple indexUids with a comma
    #[serde(skip_serializing_if = "Vec::is_empty")]
    index_uids: Vec<String>,
    /// Select batches containing tasks with the specified status.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    statuses: Vec<Status>,
    /// Select batches containing tasks with the specified type.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    types: Vec<Type>,
    /// Maximum number of batches to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    /// The first batch uid that should be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<u32>,
    /// If true, returns results in the reverse order, from oldest to most recent
    reverse: bool,
    /// Select batches containing tasks with the specified enqueuedAt field
    #[serde(skip_serializing_if = "Option::is_none")]
    before_enqueued_at: Option<OffsetDateTime>,
    /// Select batches containing tasks with the specified startedAt field
    #[serde(skip_serializing_if = "Option::is_none")]
    before_started_at: Option<OffsetDateTime>,
    /// Select batches containing tasks with the specified finishedAt field
    #[serde(skip_serializing_if = "Option::is_none")]
    before_finished_at: Option<OffsetDateTime>,
    /// Select batches containing tasks with the specified enqueuedAt field
    #[serde(skip_serializing_if = "Option::is_none")]
    after_enqueued_at: Option<OffsetDateTime>,
    /// Select batches containing tasks with the specified startedAt field
    #[serde(skip_serializing_if = "Option::is_none")]
    after_started_at: Option<OffsetDateTime>,
    /// Select batches containing tasks with the specified finishedAt field
    #[serde(skip_serializing_if = "Option::is_none")]
    after_finished_at: Option<OffsetDateTime>,
}

impl<'a, Http: HttpClient> BatchesQuery<'a, Http> {
    #[must_use]
    pub fn new(client: &'a Client<Http>) -> BatchesQuery<'a, Http> {
        BatchesQuery {
            client,
            uids: vec![],
            batch_uids: vec![],
            index_uids: vec![],
            statuses: vec![],
            types: vec![],
            limit: None,
            from: None,
            reverse: false,
            before_enqueued_at: None,
            before_started_at: None,
            before_finished_at: None,
            after_enqueued_at: None,
            after_started_at: None,
            after_finished_at: None,
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

    #[must_use]
    pub fn with_uids(&mut self, uids: Vec<i64>) -> &mut Self {
        self.uids = uids;
        self
    }

    #[must_use]
    pub fn with_batch_uids(&mut self, batch_uids: Vec<i64>) -> &mut Self {
        self.batch_uids = batch_uids;
        self
    }

    #[must_use]
    pub fn with_index_uids(&mut self, index_uids: Vec<String>) -> &mut Self {
        self.index_uids = index_uids;
        self
    }

    #[must_use]
    pub fn with_statuses(&mut self, statuses: Vec<Status>) -> &mut Self {
        self.statuses = statuses;
        self
    }

    #[must_use]
    pub fn with_types(&mut self, types: Vec<Type>) -> &mut Self {
        self.types = types;
        self
    }

    #[must_use]
    pub fn with_reverse(&mut self, reverse: bool) -> &mut Self {
        self.reverse = reverse;
        self
    }

    #[must_use]
    pub fn with_before_enqueued_at(&mut self, before_enqueued_at: OffsetDateTime) -> &mut Self {
        self.before_enqueued_at = Some(before_enqueued_at);
        self
    }

    #[must_use]
    pub fn with_before_started_at(&mut self, before_started_at: OffsetDateTime) -> &mut Self {
        self.before_started_at = Some(before_started_at);
        self
    }

    #[must_use]
    pub fn with_before_finished_at(&mut self, before_finished_at: OffsetDateTime) -> &mut Self {
        self.before_finished_at = Some(before_finished_at);
        self
    }

    #[must_use]
    pub fn with_after_enqueued_at(&mut self, after_enqueued_at: OffsetDateTime) -> &mut Self {
        self.after_enqueued_at = Some(after_enqueued_at);
        self
    }

    #[must_use]
    pub fn with_after_started_at(&mut self, after_started_at: OffsetDateTime) -> &mut Self {
        self.after_started_at = Some(after_started_at);
        self
    }

    #[must_use]
    pub fn with_after_finished_at(&mut self, after_finished_at: OffsetDateTime) -> &mut Self {
        self.after_finished_at = Some(after_finished_at);
        self
    }

    /// Execute the query and list batches.
    pub async fn execute(&self) -> Result<BatchesResults, Error> {
        self.client.get_batches_with(self).await
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchProgress {
    pub steps: Vec<BatchProgressStep>,
    pub percentage: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchProgressStep {
    pub current_step: String,
    pub finished: i32,
    pub total: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchStats {
    pub total_nb_tasks: i32,
    pub status: HashMap<Status, i32>,
    pub types: HashMap<Type, i32>,
    pub indexed_uids: HashMap<String, i32>,
    pub progress_trace: HashMap<String, String>,
    pub write_channel_congestion: HashMap<String, String>,
    pub internal_database_sizes: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Enqueued,
    Processing,
    Succeeded,
    Failed,
    Canceled,
}

#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Type {
    DocumentAdditionOrUpdate,
    DocumentEdition,
    DocumentDeletion,
    SettingsUpdate,
    IndexCreation,
    IndexDeletion,
    IndexUpdate,
    IndexSwap,
    TaskCancellation,
    TaskDeletion,
    DumpCreation,
    SnapshotCreation,
    Export,
    UpgradeDatabase,
    IndexCompaction,
    NetworkTopologyChange,
}

#[cfg(test)]
mod tests {
    use crate::batches::BatchStrategy;
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
        assert_eq!(b.batch_strategy, Some(BatchStrategy::TimeLimitReached));
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
        assert_eq!(batch.batch_strategy, Some(BatchStrategy::SizeLimitReached));
    }

    #[tokio::test]
    async fn test_query_serialization_for_batches() {
        use mockito::Matcher;
        let mut s = mockito::Server::new_async().await;
        let base = s.url();

        let _m = s
            .mock("GET", "/batches")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("limit".into(), "2".into()),
                Matcher::UrlEncoded("from".into(), "40".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"results":[],"limit":2,"total":0}"#)
            .create_async()
            .await;

        let client = Client::new(base, None::<String>).unwrap();
        let mut q = crate::batches::BatchesQuery::new(&client);
        let _ = q.with_limit(2).with_from(40);
        let res = client.get_batches_with(&q).await.expect("request failed");
        assert_eq!(res.limit, 2);
    }
}
