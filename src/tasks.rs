use serde::{Deserialize, Deserializer, Serialize};
use std::time::Duration;
use time::OffsetDateTime;

use crate::{
    client::Client, errors::Error, errors::MeilisearchError, indexes::Index, settings::Settings,
};

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
        details: Option<Settings>,
    },
}

#[derive(Debug, Clone, Deserialize)]
pub struct TasksResults {
    pub results: Vec<Task>,
    pub limit: u32,
    pub from: u32,
    pub next: u32,
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
    pub deleted_documents: Option<usize>,
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
    let iso_duration = iso8601_duration::Duration::parse(&s).map_err(serde::de::Error::custom)?;
    Ok(iso_duration.to_std())
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
    pub index_uid: Option<String>,
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
    /// `interval` = The frequency at which the server should be polled. Default = 50ms
    /// `timeout` = The maximum time to wait for processing to complete. Default = 5000ms
    ///
    /// If the waited time exceeds `timeout` then an [Error::Timeout] will be returned.
    ///
    /// See also [Client::wait_for_task, Index::wait_for_task].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, tasks::Task};
    /// # use serde::{Serialize, Deserialize};
    /// #
    /// # let MEILISEARCH_HOST = option_env!("MEILISEARCH_HOST").unwrap_or("http://localhost:7700");
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
    /// let client = Client::new(MEILISEARCH_HOST, MEILISEARCH_API_KEY);
    /// let movies = client.index("movies_wait_for_completion");
    ///
    /// let status = movies.add_documents(&[
    ///     Document { id: 0, kind: "title".into(), value: "The Social Network".to_string() },
    ///     Document { id: 1, kind: "title".into(), value: "Harry Potter and the Sorcerer's Stone".to_string() },
    /// ], None)
    ///   .await
    ///   .unwrap()
    ///   .wait_for_completion(&client, None, None)
    ///   .await
    ///   .unwrap();
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
    /// # let MEILISEARCH_HOST = option_env!("MEILISEARCH_HOST").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// // create the client
    /// let client = Client::new(MEILISEARCH_HOST, MEILISEARCH_API_KEY);
    ///
    /// let task = client.create_index("try_make_index", None).await.unwrap();
    /// let index = client.wait_for_task(task, None, None).await.unwrap().try_make_index(&client).unwrap();
    ///
    /// // and safely access it
    /// assert_eq!(index.as_ref(), "try_make_index");
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
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

    /// Unwrap the [MeilisearchError] from a [Self::Failed] [Task].
    ///
    /// Will panic if the task was not [Self::Failed].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, errors::ErrorCode};
    /// #
    /// # let MEILISEARCH_HOST = option_env!("MEILISEARCH_HOST").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new(MEILISEARCH_HOST, MEILISEARCH_API_KEY);
    /// # let task = client.create_index("unwrap_failure", None).await.unwrap();
    /// # let index = client.wait_for_task(task, None, None).await.unwrap().try_make_index(&client).unwrap();
    ///
    ///
    /// let task = index.set_ranking_rules(["wrong_ranking_rule"])
    ///   .await
    ///   .unwrap()
    ///   .wait_for_completion(&client, None, None)
    ///   .await
    ///   .unwrap();
    ///
    /// assert!(task.is_failure());
    ///
    /// let failure = task.unwrap_failure();
    ///
    /// assert_eq!(failure.error_code, ErrorCode::InvalidRankingRule);
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
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

    /// Returns `true` if the [Task] is [Self::Failed].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, errors::ErrorCode};
    /// #
    /// # let MEILISEARCH_HOST = option_env!("MEILISEARCH_HOST").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new(MEILISEARCH_HOST, MEILISEARCH_API_KEY);
    /// # let task = client.create_index("is_failure", None).await.unwrap();
    /// # let index = client.wait_for_task(task, None, None).await.unwrap().try_make_index(&client).unwrap();
    ///
    ///
    /// let task = index.set_ranking_rules(["wrong_ranking_rule"])
    ///   .await
    ///   .unwrap()
    ///   .wait_for_completion(&client, None, None)
    ///   .await
    ///   .unwrap();
    ///
    /// assert!(task.is_failure());
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub fn is_failure(&self) -> bool {
        matches!(self, Self::Failed { .. })
    }

    /// Returns `true` if the [Task] is [Self::Succeeded].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, errors::ErrorCode};
    /// #
    /// # let MEILISEARCH_HOST = option_env!("MEILISEARCH_HOST").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new(MEILISEARCH_HOST, MEILISEARCH_API_KEY);
    /// let task = client
    ///   .create_index("is_success", None)
    ///   .await
    ///   .unwrap()
    ///   .wait_for_completion(&client, None, None)
    ///   .await
    ///   .unwrap();
    ///
    /// assert!(task.is_success());
    /// # task.try_make_index(&client).unwrap().delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Succeeded { .. })
    }

    /// Returns `true` if the [Task] is pending ([Self::Enqueued] or [Self::Processing]).
    ///
    /// # Example
    /// ```no_run
    /// # // The test is not run because it checks for an enqueued or processed status
    /// # // and the task might already be processed when checking the status after the get_task call
    /// # use meilisearch_sdk::{client::*, indexes::*, errors::ErrorCode};
    /// #
    /// # let MEILISEARCH_HOST = option_env!("MEILISEARCH_HOST").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # futures::executor::block_on(async move {
    /// # let client = Client::new(MEILISEARCH_HOST, MEILISEARCH_API_KEY);
    /// let task_info = client
    ///   .create_index("is_pending", None)
    ///   .await
    ///   .unwrap();
    /// let task = client.get_task(task_info).await.unwrap();
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
#[serde(rename_all = "camelCase")]
pub struct TasksQuery<'a> {
    // Index uids array to only retrieve the tasks of the indexes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_uid: Option<&'a [&'a str]>,
    // Statuses array to only retrieve the tasks with these statuses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<&'a [&'a str]>,
    // Types array to only retrieve the tasks with these [TaskType].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<&'a [&'a str]>,
}

#[allow(missing_docs)]
impl<'a> TasksQuery<'a> {
    pub fn new() -> TasksQuery<'a> {
        TasksQuery {
            index_uid: None,
            status: None,
            r#type: None,
        }
    }
    pub fn with_index_uid<'b>(&'b mut self, index_uid: &'a [&'a str]) -> &'b mut TasksQuery<'a> {
        self.index_uid = Some(index_uid);
        self
    }
    pub fn with_status<'b>(&'b mut self, status: &'a [&'a str]) -> &'b mut TasksQuery<'a> {
        self.status = Some(status);
        self
    }
    pub fn with_type<'b>(&'b mut self, r#type: &'a [&'a str]) -> &'b mut TasksQuery<'a> {
        self.r#type = Some(r#type);
        self
    }
    // execute
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        client::*,
        errors::{ErrorCode, ErrorType},
    };
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
            if duration == Duration::from_secs_f32(10.848957061)
        ));
    }

    #[meilisearch_test]
    async fn test_wait_for_task_with_args(client: Client, movies: Index) -> Result<(), Error> {
        let task_info = movies
            .add_documents(
                &[
                    Document {
                        id: 0,
                        kind: "title".into(),
                        value: "The Social Network".to_string(),
                    },
                    Document {
                        id: 1,
                        kind: "title".into(),
                        value: "Harry Potter and the Sorcerer's Stone".to_string(),
                    },
                ],
                None,
            )
            .await?;

        let task = client
            .get_task(task_info)
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
    // TODO: failing because settings routes now uses PUT instead of POST as http method
    async fn test_failing_task(client: Client, movies: Index) -> Result<(), Error> {
        let task_info = movies.set_ranking_rules(["wrong_ranking_rule"]).await?;

        let task = client.get_task(task_info).await?;
        let task = client.wait_for_task(task, None, None).await?;

        let error = task.unwrap_failure();
        assert_eq!(error.error_code, ErrorCode::InvalidRankingRule);
        assert_eq!(error.error_type, ErrorType::InvalidRequest);
        Ok(())
    }
}
