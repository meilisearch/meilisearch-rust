use serde::{Deserialize, Deserializer, Serialize};
use std::time::Duration;
use time::OffsetDateTime;

use crate::{Client, Error, Index, MeilisearchError, Settings, SwapIndexes, TaskInfo};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum TaskType {
    Customs,
    DocumentAdditionOrUpdate {
        details: Option<DocumentAdditionOrUpdate>,
    },
    DocumentDeletion {
        details: Option<DocumentDeletion>,
    },
    IndexCreation {
        details: Option<IndexCreation>,
    },
    IndexUpdate {
        details: Option<IndexUpdate>,
    },
    IndexDeletion {
        details: Option<IndexDeletion>,
    },
    SettingsUpdate {
        details: Box<Option<Settings>>,
    },
    DumpCreation {
        details: Option<DumpCreation>,
    },
    IndexSwap {
        details: Option<IndexSwap>,
    },
    TaskCancelation {
        details: Option<TaskCancelation>,
    },
    TaskDeletion {
        details: Option<TaskDeletion>,
    },
    SnapshotCreation {
        details: Option<SnapshotCreation>,
    },
}

#[derive(Debug, Clone, Deserialize)]
pub struct TasksResults {
    pub results: Vec<Task>,
    pub limit: u32,
    pub from: Option<u32>,
    pub next: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentAdditionOrUpdate {
    pub indexed_documents: Option<usize>,
    pub received_documents: usize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentDeletion {
    pub provided_ids: Option<usize>,
    pub deleted_documents: Option<usize>,
    pub original_filter: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexCreation {
    pub primary_key: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexUpdate {
    pub primary_key: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexDeletion {
    pub deleted_documents: Option<usize>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotCreation {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DumpCreation {
    pub dump_uid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexSwap {
    pub swaps: Vec<SwapIndexes>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskCancelation {
    pub matched_tasks: usize,
    pub canceled_tasks: usize,
    pub original_filter: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskDeletion {
    pub matched_tasks: usize,
    pub deleted_tasks: usize,
    pub original_filter: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FailedTask {
    pub error: MeilisearchError,
    #[serde(flatten)]
    pub task: SucceededTask,
}

impl AsRef<u32> for FailedTask {
    fn as_ref(&self) -> &u32 {
        &self.task.uid
    }
}

fn deserialize_duration<'de, D>(deserializer: D) -> Result<std::time::Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let iso_duration = iso8601::duration(&s).map_err(serde::de::Error::custom)?;
    Ok(iso_duration.into())
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SucceededTask {
    #[serde(deserialize_with = "deserialize_duration")]
    pub duration: Duration,
    #[serde(with = "time::serde::rfc3339")]
    pub enqueued_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub started_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub finished_at: OffsetDateTime,
    pub canceled_by: Option<usize>,
    pub index_uid: Option<String>,
    pub error: Option<MeilisearchError>,
    #[serde(flatten)]
    pub update_type: TaskType,
    pub uid: u32,
}

impl AsRef<u32> for SucceededTask {
    fn as_ref(&self) -> &u32 {
        &self.uid
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnqueuedTask {
    #[serde(with = "time::serde::rfc3339")]
    pub enqueued_at: OffsetDateTime,
    pub index_uid: Option<String>,
    #[serde(flatten)]
    pub update_type: TaskType,
    pub uid: u32,
}

impl AsRef<u32> for EnqueuedTask {
    fn as_ref(&self) -> &u32 {
        &self.uid
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", tag = "status")]
pub enum Task {
    Enqueued {
        #[serde(flatten)]
        content: EnqueuedTask,
    },
    Processing {
        #[serde(flatten)]
        content: EnqueuedTask,
    },
    Failed {
        #[serde(flatten)]
        content: FailedTask,
    },
    Succeeded {
        #[serde(flatten)]
        content: SucceededTask,
    },
}

impl Task {
    pub fn get_uid(&self) -> u32 {
        match self {
            Self::Enqueued { content } | Self::Processing { content } => *content.as_ref(),
            Self::Failed { content } => *content.as_ref(),
            Self::Succeeded { content } => *content.as_ref(),
        }
    }

    /// Wait until Meilisearch processes a [Task], and get its status.
    ///
    /// `interval` = The frequency at which the server should be polled. **Default = 50ms**
    ///
    /// `timeout` = The maximum time to wait for processing to complete. **Default = 5000ms**
    ///
    /// If the waited time exceeds `timeout` then an [`Error::Timeout`] will be returned.
    ///
    /// See also [`Client::wait_for_task`, `Index::wait_for_task`].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, Task};
    /// # use serde::{Serialize, Deserialize};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # #[derive(Debug, Serialize, Deserialize, PartialEq)]
    /// # struct Document {
    /// #    id: usize,
    /// #    value: String,
    /// #    kind: String,
    /// # }
    /// #
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY));
    /// let movies = client.index("movies_wait_for_completion");
    ///
    /// let status = movies.add_documents(&[
    ///     Document { id: 0, kind: "title".into(), value: "The Social Network".to_string() },
    ///     Document { id: 1, kind: "title".into(), value: "Harry Potter and the Sorcerer's Stone".to_string() },
    /// ], None)
    ///     .await
    ///     .unwrap()
    ///     .wait_for_completion(&client, None, None)
    ///     .await
    ///     .unwrap();
    ///
    /// assert!(matches!(status, Task::Succeeded { .. }));
    /// # movies.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn wait_for_completion(
        self,
        client: &Client,
        interval: Option<Duration>,
        timeout: Option<Duration>,
    ) -> Result<Self, Error> {
        client.wait_for_task(self, interval, timeout).await
    }

    /// Extract the [Index] from a successful `IndexCreation` task.
    ///
    /// If the task failed or was not an `IndexCreation` task it return itself.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # // create the client
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY));
    /// let task = client.create_index("try_make_index", None).await.unwrap();
    /// let index = client.wait_for_task(task, None, None).await.unwrap().try_make_index(&client).unwrap();
    ///
    /// // and safely access it
    /// assert_eq!(index.as_ref(), "try_make_index");
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    #[allow(clippy::result_large_err)] // Since `self` has been consumed, this is not an issue
    pub fn try_make_index(self, client: &Client) -> Result<Index, Self> {
        match self {
            Self::Succeeded {
                content:
                    SucceededTask {
                        index_uid,
                        update_type: TaskType::IndexCreation { .. },
                        ..
                    },
            } => Ok(client.index(index_uid.unwrap())),
            _ => Err(self),
        }
    }

    /// Unwrap the [`MeilisearchError`] from a [`Self::Failed`] [Task].
    ///
    /// Will panic if the task was not [`Self::Failed`].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, ErrorCode};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY));
    /// let task = client.create_index("unwrap_failure", None).await.unwrap();
    /// let task = client
    ///     .create_index("unwrap_failure", None)
    ///     .await
    ///     .unwrap()
    ///     .wait_for_completion(&client, None, None)
    ///     .await
    ///     .unwrap();
    ///
    /// assert!(task.is_failure());
    ///
    /// let failure = task.unwrap_failure();
    ///
    /// assert_eq!(failure.error_code, ErrorCode::IndexAlreadyExists);
    /// # client.index("unwrap_failure").delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub fn unwrap_failure(self) -> MeilisearchError {
        match self {
            Self::Failed {
                content: FailedTask { error, .. },
            } => error,
            _ => panic!("Called `unwrap_failure` on a non `Failed` task."),
        }
    }

    /// Returns `true` if the [Task] is [`Self::Failed`].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, ErrorCode};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY));
    /// let task = client.create_index("is_failure", None).await.unwrap();
    /// // create an index with a conflicting uid
    /// let task = client
    ///     .create_index("is_failure", None)
    ///     .await
    ///     .unwrap()
    ///     .wait_for_completion(&client, None, None)
    ///     .await
    ///     .unwrap();
    ///
    /// assert!(task.is_failure());
    /// # client.index("is_failure").delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub fn is_failure(&self) -> bool {
        matches!(self, Self::Failed { .. })
    }

    /// Returns `true` if the [Task] is [`Self::Succeeded`].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, ErrorCode};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY));
    /// let task = client
    ///     .create_index("is_success", None)
    ///     .await
    ///     .unwrap()
    ///     .wait_for_completion(&client, None, None)
    ///     .await
    ///     .unwrap();
    ///
    /// assert!(task.is_success());
    /// # task.try_make_index(&client).unwrap().delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Succeeded { .. })
    }

    /// Returns `true` if the [Task] is pending ([`Self::Enqueued`] or [`Self::Processing`]).
    ///
    /// # Example
    /// ```no_run
    /// # // The test is not run because it checks for an enqueued or processed status
    /// # // and the task might already be processed when checking the status after the get_task call
    /// # use meilisearch_sdk::{client::*, indexes::*, ErrorCode};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY));
    /// let task_info = client
    ///     .create_index("is_pending", None)
    ///     .await
    ///     .unwrap();
    /// let task = client.get_task(task_info).await.unwrap();
    ///
    /// assert!(task.is_pending());
    /// # task.wait_for_completion(&client, None, None).await.unwrap().try_make_index(&client).unwrap().delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub fn is_pending(&self) -> bool {
        matches!(self, Self::Enqueued { .. } | Self::Processing { .. })
    }
}

impl AsRef<u32> for Task {
    fn as_ref(&self) -> &u32 {
        match self {
            Self::Enqueued { content } | Self::Processing { content } => content.as_ref(),
            Self::Succeeded { content } => content.as_ref(),
            Self::Failed { content } => content.as_ref(),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct TasksPaginationFilters {
    // Maximum number of tasks to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    // The first task uid that should be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<u32>,
}

#[derive(Debug, Serialize, Clone)]
pub struct TasksCancelFilters {}

#[derive(Debug, Serialize, Clone)]
pub struct TasksDeleteFilters {}

pub type TasksSearchQuery<'a> = TasksQuery<'a, TasksPaginationFilters>;
pub type TasksCancelQuery<'a> = TasksQuery<'a, TasksCancelFilters>;
pub type TasksDeleteQuery<'a> = TasksQuery<'a, TasksDeleteFilters>;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TasksQuery<'a, T> {
    #[serde(skip_serializing)]
    client: &'a Client,
    // Index uids array to only retrieve the tasks of the indexes.
    #[serde(skip_serializing_if = "Option::is_none")]
    index_uids: Option<Vec<&'a str>>,
    // Statuses array to only retrieve the tasks with these statuses.
    #[serde(skip_serializing_if = "Option::is_none")]
    statuses: Option<Vec<&'a str>>,
    // Types array to only retrieve the tasks with these [TaskType].
    #[serde(skip_serializing_if = "Option::is_none", rename = "types")]
    task_types: Option<Vec<&'a str>>,
    // Uids of the tasks to retrieve.
    #[serde(skip_serializing_if = "Option::is_none")]
    uids: Option<Vec<&'a u32>>,
    // Uids of the tasks that canceled other tasks.
    #[serde(skip_serializing_if = "Option::is_none")]
    canceled_by: Option<Vec<&'a u32>>,
    // Date to retrieve all tasks that were enqueued before it.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "time::serde::rfc3339::option::serialize"
    )]
    before_enqueued_at: Option<OffsetDateTime>,
    // Date to retrieve all tasks that were enqueued after it.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "time::serde::rfc3339::option::serialize"
    )]
    after_enqueued_at: Option<OffsetDateTime>,
    // Date to retrieve all tasks that were started before it.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "time::serde::rfc3339::option::serialize"
    )]
    before_started_at: Option<OffsetDateTime>,
    // Date to retrieve all tasks that were started after it.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "time::serde::rfc3339::option::serialize"
    )]
    after_started_at: Option<OffsetDateTime>,
    // Date to retrieve all tasks that were finished before it.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "time::serde::rfc3339::option::serialize"
    )]
    before_finished_at: Option<OffsetDateTime>,
    // Date to retrieve all tasks that were finished after it.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "time::serde::rfc3339::option::serialize"
    )]
    after_finished_at: Option<OffsetDateTime>,

    #[serde(flatten)]
    pagination: T,
}

#[allow(missing_docs)]
impl<'a, T> TasksQuery<'a, T> {
    pub fn with_index_uids<'b>(
        &'b mut self,
        index_uids: impl IntoIterator<Item = &'a str>,
    ) -> &'b mut TasksQuery<'a, T> {
        self.index_uids = Some(index_uids.into_iter().collect());
        self
    }
    pub fn with_statuses<'b>(
        &'b mut self,
        statuses: impl IntoIterator<Item = &'a str>,
    ) -> &'b mut TasksQuery<'a, T> {
        self.statuses = Some(statuses.into_iter().collect());
        self
    }
    pub fn with_types<'b>(
        &'b mut self,
        task_types: impl IntoIterator<Item = &'a str>,
    ) -> &'b mut TasksQuery<'a, T> {
        self.task_types = Some(task_types.into_iter().collect());
        self
    }
    pub fn with_uids<'b>(
        &'b mut self,
        uids: impl IntoIterator<Item = &'a u32>,
    ) -> &'b mut TasksQuery<'a, T> {
        self.uids = Some(uids.into_iter().collect());
        self
    }
    pub fn with_before_enqueued_at<'b>(
        &'b mut self,
        before_enqueued_at: &'a OffsetDateTime,
    ) -> &'b mut TasksQuery<'a, T> {
        self.before_enqueued_at = Some(*before_enqueued_at);
        self
    }
    pub fn with_after_enqueued_at<'b>(
        &'b mut self,
        after_enqueued_at: &'a OffsetDateTime,
    ) -> &'b mut TasksQuery<'a, T> {
        self.after_enqueued_at = Some(*after_enqueued_at);
        self
    }
    pub fn with_before_started_at<'b>(
        &'b mut self,
        before_started_at: &'a OffsetDateTime,
    ) -> &'b mut TasksQuery<'a, T> {
        self.before_started_at = Some(*before_started_at);
        self
    }
    pub fn with_after_started_at<'b>(
        &'b mut self,
        after_started_at: &'a OffsetDateTime,
    ) -> &'b mut TasksQuery<'a, T> {
        self.after_started_at = Some(*after_started_at);
        self
    }
    pub fn with_before_finished_at<'b>(
        &'b mut self,
        before_finished_at: &'a OffsetDateTime,
    ) -> &'b mut TasksQuery<'a, T> {
        self.before_finished_at = Some(*before_finished_at);
        self
    }
    pub fn with_after_finished_at<'b>(
        &'b mut self,
        after_finished_at: &'a OffsetDateTime,
    ) -> &'b mut TasksQuery<'a, T> {
        self.after_finished_at = Some(*after_finished_at);
        self
    }
    pub fn with_canceled_by<'b>(
        &'b mut self,
        task_uids: impl IntoIterator<Item = &'a u32>,
    ) -> &'b mut TasksQuery<'a, T> {
        self.canceled_by = Some(task_uids.into_iter().collect());
        self
    }
}

impl<'a> TasksQuery<'a, TasksCancelFilters> {
    pub fn new(client: &'a Client) -> TasksQuery<'a, TasksCancelFilters> {
        TasksQuery {
            client,
            index_uids: None,
            statuses: None,
            task_types: None,
            uids: None,
            canceled_by: None,
            before_enqueued_at: None,
            after_enqueued_at: None,
            before_started_at: None,
            after_started_at: None,
            before_finished_at: None,
            after_finished_at: None,
            pagination: TasksCancelFilters {},
        }
    }

    pub async fn execute(&'a self) -> Result<TaskInfo, Error> {
        self.client.cancel_tasks_with(self).await
    }
}

impl<'a> TasksQuery<'a, TasksDeleteFilters> {
    pub fn new(client: &'a Client) -> TasksQuery<'a, TasksDeleteFilters> {
        TasksQuery {
            client,
            index_uids: None,
            statuses: None,
            task_types: None,
            uids: None,
            canceled_by: None,
            before_enqueued_at: None,
            after_enqueued_at: None,
            before_started_at: None,
            after_started_at: None,
            before_finished_at: None,
            after_finished_at: None,
            pagination: TasksDeleteFilters {},
        }
    }

    pub async fn execute(&'a self) -> Result<TaskInfo, Error> {
        self.client.delete_tasks_with(self).await
    }
}

impl<'a> TasksQuery<'a, TasksPaginationFilters> {
    pub fn new(client: &'a Client) -> TasksQuery<'a, TasksPaginationFilters> {
        TasksQuery {
            client,
            index_uids: None,
            statuses: None,
            task_types: None,
            uids: None,
            canceled_by: None,
            before_enqueued_at: None,
            after_enqueued_at: None,
            before_started_at: None,
            after_started_at: None,
            before_finished_at: None,
            after_finished_at: None,
            pagination: TasksPaginationFilters {
                limit: None,
                from: None,
            },
        }
    }
    pub fn with_limit<'b>(
        &'b mut self,
        limit: u32,
    ) -> &'b mut TasksQuery<'a, TasksPaginationFilters> {
        self.pagination.limit = Some(limit);
        self
    }
    pub fn with_from<'b>(
        &'b mut self,
        from: u32,
    ) -> &'b mut TasksQuery<'a, TasksPaginationFilters> {
        self.pagination.from = Some(from);
        self
    }
    pub async fn execute(&'a self) -> Result<TasksResults, Error> {
        self.client.get_tasks_with(self).await
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{client::*, ErrorCode, ErrorType};
    use big_s::S;
    use meilisearch_test_macro::meilisearch_test;
    use serde::{Deserialize, Serialize};
    use std::time::Duration;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Document {
        id: usize,
        value: String,
        kind: String,
    }

    #[test]
    fn test_deserialize_task() {
        let datetime = OffsetDateTime::parse(
            "2022-02-03T13:02:38.369634Z",
            &::time::format_description::well_known::Rfc3339,
        )
        .unwrap();

        let task: Task = serde_json::from_str(
            r#"
{
  "enqueuedAt": "2022-02-03T13:02:38.369634Z",
  "indexUid": "mieli",
  "status": "enqueued",
  "type": "documentAdditionOrUpdate",
  "uid": 12
}"#,
        )
        .unwrap();

        assert!(matches!(
            task,
            Task::Enqueued {
                content: EnqueuedTask {
                    enqueued_at,
                    index_uid: Some(index_uid),
                    update_type: TaskType::DocumentAdditionOrUpdate { details: None },
                    uid: 12,
                }
            }
        if enqueued_at == datetime && index_uid == "mieli"));

        let task: Task = serde_json::from_str(
            r#"
{
  "details": {
    "indexedDocuments": null,
    "receivedDocuments": 19547
  },
  "duration": null,
  "enqueuedAt": "2022-02-03T15:17:02.801341Z",
  "finishedAt": null,
  "indexUid": "mieli",
  "startedAt": "2022-02-03T15:17:02.812338Z",
  "status": "processing",
  "type": "documentAdditionOrUpdate",
  "uid": 14
}"#,
        )
        .unwrap();

        assert!(matches!(
            task,
            Task::Processing {
                content: EnqueuedTask {
                    update_type: TaskType::DocumentAdditionOrUpdate {
                        details: Some(DocumentAdditionOrUpdate {
                            received_documents: 19547,
                            indexed_documents: None,
                        })
                    },
                    uid: 14,
                    ..
                }
            }
        ));

        let task: Task = serde_json::from_str(
            r#"
{
  "details": {
    "indexedDocuments": 19546,
    "receivedDocuments": 19547
  },
  "duration": "PT10.848957S",
  "enqueuedAt": "2022-02-03T15:17:02.801341Z",
  "finishedAt": "2022-02-03T15:17:13.661295Z",
  "indexUid": "mieli",
  "startedAt": "2022-02-03T15:17:02.812338Z",
  "status": "succeeded",
  "type": "documentAdditionOrUpdate",
  "uid": 14
}"#,
        )
        .unwrap();

        assert!(matches!(
            task,
            Task::Succeeded {
                content: SucceededTask {
                    update_type: TaskType::DocumentAdditionOrUpdate {
                        details: Some(DocumentAdditionOrUpdate {
                            received_documents: 19547,
                            indexed_documents: Some(19546),
                        })
                    },
                    uid: 14,
                    duration,
                    ..
                }
            }
            if duration == Duration::from_millis(10_848)
        ));
    }

    #[meilisearch_test]
    async fn test_wait_for_task_with_args(client: Client, movies: Index) -> Result<(), Error> {
        let task = movies
            .add_documents(
                &[
                    Document {
                        id: 0,
                        kind: "title".into(),
                        value: S("The Social Network"),
                    },
                    Document {
                        id: 1,
                        kind: "title".into(),
                        value: S("Harry Potter and the Sorcerer's Stone"),
                    },
                ],
                None,
            )
            .await?
            .wait_for_completion(
                &client,
                Some(Duration::from_millis(1)),
                Some(Duration::from_millis(6000)),
            )
            .await?;

        assert!(matches!(task, Task::Succeeded { .. }));
        Ok(())
    }

    #[meilisearch_test]
    async fn test_get_tasks_no_params() -> Result<(), Error> {
        let mut s = mockito::Server::new_async().await;
        let mock_server_url = s.url();
        let client = Client::new(mock_server_url, Some("masterKey"));
        let path = "/tasks";

        let mock_res = s.mock("GET", path).with_status(200).create_async().await;
        let _ = client.get_tasks().await;
        mock_res.assert_async().await;

        Ok(())
    }

    #[meilisearch_test]
    async fn test_get_tasks_with_params() -> Result<(), Error> {
        let mut s = mockito::Server::new_async().await;
        let mock_server_url = s.url();
        let client = Client::new(mock_server_url, Some("masterKey"));
        let path =
            "/tasks?indexUids=movies,test&statuses=equeued&types=documentDeletion&uids=1&limit=0&from=1";

        let mock_res = s.mock("GET", path).with_status(200).create_async().await;

        let mut query = TasksSearchQuery::new(&client);
        query
            .with_index_uids(["movies", "test"])
            .with_statuses(["equeued"])
            .with_types(["documentDeletion"])
            .with_from(1)
            .with_limit(0)
            .with_uids([&1]);

        let _ = client.get_tasks_with(&query).await;

        mock_res.assert_async().await;

        Ok(())
    }

    #[meilisearch_test]
    async fn test_get_tasks_with_date_params() -> Result<(), Error> {
        let mut s = mockito::Server::new_async().await;
        let mock_server_url = s.url();
        let client = Client::new(mock_server_url, Some("masterKey"));
        let path = "/tasks?\
            beforeEnqueuedAt=2022-02-03T13%3A02%3A38.369634Z\
            &afterEnqueuedAt=2023-02-03T13%3A02%3A38.369634Z\
            &beforeStartedAt=2024-02-03T13%3A02%3A38.369634Z\
            &afterStartedAt=2025-02-03T13%3A02%3A38.369634Z\
            &beforeFinishedAt=2026-02-03T13%3A02%3A38.369634Z\
            &afterFinishedAt=2027-02-03T13%3A02%3A38.369634Z";

        let mock_res = s.mock("GET", path).with_status(200).create_async().await;

        let before_enqueued_at = OffsetDateTime::parse(
            "2022-02-03T13:02:38.369634Z",
            &::time::format_description::well_known::Rfc3339,
        )
        .unwrap();
        let after_enqueued_at = OffsetDateTime::parse(
            "2023-02-03T13:02:38.369634Z",
            &::time::format_description::well_known::Rfc3339,
        )
        .unwrap();
        let before_started_at = OffsetDateTime::parse(
            "2024-02-03T13:02:38.369634Z",
            &::time::format_description::well_known::Rfc3339,
        )
        .unwrap();

        let after_started_at = OffsetDateTime::parse(
            "2025-02-03T13:02:38.369634Z",
            &::time::format_description::well_known::Rfc3339,
        )
        .unwrap();

        let before_finished_at = OffsetDateTime::parse(
            "2026-02-03T13:02:38.369634Z",
            &::time::format_description::well_known::Rfc3339,
        )
        .unwrap();

        let after_finished_at = OffsetDateTime::parse(
            "2027-02-03T13:02:38.369634Z",
            &::time::format_description::well_known::Rfc3339,
        )
        .unwrap();

        let mut query = TasksSearchQuery::new(&client);
        query
            .with_before_enqueued_at(&before_enqueued_at)
            .with_after_enqueued_at(&after_enqueued_at)
            .with_before_started_at(&before_started_at)
            .with_after_started_at(&after_started_at)
            .with_before_finished_at(&before_finished_at)
            .with_after_finished_at(&after_finished_at);

        let _ = client.get_tasks_with(&query).await;

        mock_res.assert_async().await;

        Ok(())
    }

    #[meilisearch_test]
    async fn test_get_tasks_on_struct_with_params() -> Result<(), Error> {
        let mut s = mockito::Server::new_async().await;
        let mock_server_url = s.url();
        let client = Client::new(mock_server_url, Some("masterKey"));
        let path =
            "/tasks?indexUids=movies,test&statuses=equeued&types=documentDeletion&canceledBy=9";

        let mock_res = s.mock("GET", path).with_status(200).create_async().await;

        let mut query = TasksSearchQuery::new(&client);
        let _ = query
            .with_index_uids(["movies", "test"])
            .with_statuses(["equeued"])
            .with_types(["documentDeletion"])
            .with_canceled_by([&9])
            .execute()
            .await;

        mock_res.assert_async().await;

        Ok(())
    }

    #[meilisearch_test]
    async fn test_get_tasks_with_none_existant_index_uids(client: Client) -> Result<(), Error> {
        let mut query = TasksSearchQuery::new(&client);
        query.with_index_uids(["no_name"]);
        let tasks = client.get_tasks_with(&query).await.unwrap();

        assert_eq!(tasks.results.len(), 0);
        Ok(())
    }

    #[meilisearch_test]
    async fn test_get_tasks_with_execute(client: Client) -> Result<(), Error> {
        let tasks = TasksSearchQuery::new(&client)
            .with_index_uids(["no_name"])
            .execute()
            .await
            .unwrap();

        assert_eq!(tasks.results.len(), 0);
        Ok(())
    }

    #[meilisearch_test]
    async fn test_failing_task(client: Client, index: Index) -> Result<(), Error> {
        let task_info = client.create_index(index.uid, None).await.unwrap();
        let task = client.get_task(task_info).await?;
        let task = client.wait_for_task(task, None, None).await?;

        let error = task.unwrap_failure();
        assert_eq!(error.error_code, ErrorCode::IndexAlreadyExists);
        assert_eq!(error.error_type, ErrorType::InvalidRequest);
        Ok(())
    }

    #[meilisearch_test]
    async fn test_cancel_tasks_with_params() -> Result<(), Error> {
        let mut s = mockito::Server::new_async().await;
        let mock_server_url = s.url();
        let client = Client::new(mock_server_url, Some("masterKey"));
        let path =
            "/tasks/cancel?indexUids=movies,test&statuses=equeued&types=documentDeletion&uids=1";

        let mock_res = s.mock("POST", path).with_status(200).create_async().await;

        let mut query = TasksCancelQuery::new(&client);
        query
            .with_index_uids(["movies", "test"])
            .with_statuses(["equeued"])
            .with_types(["documentDeletion"])
            .with_uids([&1]);

        let _ = client.cancel_tasks_with(&query).await;

        mock_res.assert_async().await;

        Ok(())
    }

    #[meilisearch_test]
    async fn test_cancel_tasks_with_params_execute() -> Result<(), Error> {
        let mut s = mockito::Server::new_async().await;
        let mock_server_url = s.url();
        let client = Client::new(mock_server_url, Some("masterKey"));
        let path =
            "/tasks/cancel?indexUids=movies,test&statuses=equeued&types=documentDeletion&uids=1";

        let mock_res = s.mock("POST", path).with_status(200).create_async().await;

        let mut query = TasksCancelQuery::new(&client);
        let _ = query
            .with_index_uids(["movies", "test"])
            .with_statuses(["equeued"])
            .with_types(["documentDeletion"])
            .with_uids([&1])
            .execute()
            .await;

        mock_res.assert_async().await;

        Ok(())
    }

    #[meilisearch_test]
    async fn test_delete_tasks_with_params() -> Result<(), Error> {
        let mut s = mockito::Server::new_async().await;
        let mock_server_url = s.url();
        let client = Client::new(mock_server_url, Some("masterKey"));
        let path = "/tasks?indexUids=movies,test&statuses=equeued&types=documentDeletion&uids=1";

        let mock_res = s.mock("DELETE", path).with_status(200).create_async().await;

        let mut query = TasksDeleteQuery::new(&client);
        query
            .with_index_uids(["movies", "test"])
            .with_statuses(["equeued"])
            .with_types(["documentDeletion"])
            .with_uids([&1]);

        let _ = client.delete_tasks_with(&query).await;

        mock_res.assert_async().await;

        Ok(())
    }

    #[meilisearch_test]
    async fn test_delete_tasks_with_params_execute() -> Result<(), Error> {
        let mut s = mockito::Server::new_async().await;
        let mock_server_url = s.url();
        let client = Client::new(mock_server_url, Some("masterKey"));
        let path = "/tasks?indexUids=movies,test&statuses=equeued&types=documentDeletion&uids=1";

        let mock_res = s.mock("DELETE", path).with_status(200).create_async().await;

        let mut query = TasksDeleteQuery::new(&client);
        let _ = query
            .with_index_uids(["movies", "test"])
            .with_statuses(["equeued"])
            .with_types(["documentDeletion"])
            .with_uids([&1])
            .execute()
            .await;

        mock_res.assert_async().await;

        Ok(())
    }
}
