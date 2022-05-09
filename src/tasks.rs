use serde::{Deserialize, Deserializer};
use std::time::Duration;
use time::OffsetDateTime;

use crate::{
    client::Client, errors::Error, errors::MeilisearchError, indexes::Index, settings::Settings,
};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum TaskType {
    ClearAll,
    Customs,
    DocumentAddition { details: Option<DocumentAddition> },
    DocumentPartial { details: Option<DocumentAddition> },
    DocumentDeletion { details: Option<DocumentDeletion> },
    IndexCreation { details: Option<IndexCreation> },
    IndexUpdate { details: Option<IndexUpdate> },
    IndexDeletion { details: Option<IndexDeletion> },
    SettingsUpdate { details: Option<Settings> },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentAddition {
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
    pub task: ProcessedTask,
}

impl AsRef<u64> for FailedTask {
    fn as_ref(&self) -> &u64 {
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
pub struct ProcessedTask {
    #[serde(deserialize_with = "deserialize_duration")]
    pub duration: Duration,
    #[serde(with = "time::serde::rfc3339")]
    pub enqueued_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub started_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub finished_at: OffsetDateTime,
    pub index_uid: String,
    #[serde(flatten)]
    pub update_type: TaskType,
    pub uid: u64,
}

impl AsRef<u64> for ProcessedTask {
    fn as_ref(&self) -> &u64 {
        &self.uid
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnqueuedTask {
    #[serde(with = "time::serde::rfc3339")]
    pub enqueued_at: OffsetDateTime,
    pub index_uid: String,
    #[serde(flatten)]
    pub update_type: TaskType,
    pub uid: u64,
}

impl AsRef<u64> for EnqueuedTask {
    fn as_ref(&self) -> &u64 {
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
        content: ProcessedTask,
    },
}

impl Task {
    pub fn get_uid(&self) -> u64 {
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
    /// # #[derive(Debug, Serialize, Deserialize, PartialEq)]
    /// # struct Document {
    /// #    id: usize,
    /// #    value: String,
    /// #    kind: String,
    /// # }
    /// #
    /// #
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", Some("masterKey")); 
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
    /// # futures::executor::block_on(async move {
    /// // create the client
    /// let client = Client::new("http://localhost:7700", Some("masterKey"));
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
                    ProcessedTask {
                        index_uid,
                        update_type: TaskType::IndexCreation { .. },
                        ..
                    },
            } => Ok(client.index(index_uid)),
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
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", Some("masterKey"));
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
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", Some("masterKey"));
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
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", Some("masterKey"));
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
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Succeeded { .. })
    }

    /// Returns `true` if the [Task] is pending ([Self::Enqueued] or [Self::Processing]).
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, errors::ErrorCode};
    /// #
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", Some("masterKey"));
    /// let task = client
    ///   .create_index("is_pending", None)
    ///   .await
    ///   .unwrap();
    ///
    /// assert!(task.is_pending());
    /// # task.wait_for_completion(&client, None, None).await.unwrap().try_make_index(&client).unwrap().delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    pub fn is_pending(&self) -> bool {
        matches!(self, Self::Enqueued { .. } | Self::Processing { .. })
    }
}

impl AsRef<u64> for Task {
    fn as_ref(&self) -> &u64 {
        match self {
            Self::Enqueued { content } | Self::Processing { content } => content.as_ref(),
            Self::Succeeded { content } => content.as_ref(),
            Self::Failed { content } => content.as_ref(),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) async fn async_sleep(interval: Duration) {
    let (sender, receiver) = futures::channel::oneshot::channel::<()>();
    std::thread::spawn(move || {
        std::thread::sleep(interval);
        let _ = sender.send(());
    });
    let _ = receiver.await;
}

#[cfg(target_arch = "wasm32")]
pub(crate) async fn async_sleep(interval: Duration) {
    use std::convert::TryInto;
    use wasm_bindgen_futures::JsFuture;

    JsFuture::from(js_sys::Promise::new(&mut |yes, _| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &yes,
                interval.as_millis().try_into().unwrap(),
            )
            .unwrap();
    }))
    .await
    .unwrap();
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
    use std::time::{self, Duration};

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
  "type": "documentAddition",
  "uid": 12
}"#,
        )
        .unwrap();

        assert!(matches!(
            task,
            Task::Enqueued {
                content: EnqueuedTask {
                    enqueued_at,
                    index_uid,
                    update_type: TaskType::DocumentAddition { details: None },
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
  "type": "documentAddition",
  "uid": 14
}"#,
        )
        .unwrap();

        assert!(matches!(
            task,
            Task::Processing {
                content: EnqueuedTask {
                    update_type: TaskType::DocumentAddition {
                        details: Some(DocumentAddition {
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
  "type": "documentAddition",
  "uid": 14
}"#,
        )
        .unwrap();

        assert!(matches!(
            task,
            Task::Succeeded {
                content: ProcessedTask {
                    update_type: TaskType::DocumentAddition {
                        details: Some(DocumentAddition {
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
    async fn test_wait_for_pending_updates_with_args(
        client: Client,
        movies: Index,
    ) -> Result<(), Error> {
        let status = movies
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
            .await?
            .wait_for_completion(
                &client,
                Some(Duration::from_millis(1)),
                Some(Duration::from_millis(6000)),
            )
            .await?;

        assert!(matches!(status, Task::Succeeded { .. }));
        Ok(())
    }

    #[meilisearch_test]
    async fn test_wait_for_pending_updates_time_out(
        client: Client,
        movies: Index,
    ) -> Result<(), Error> {
        let task = movies
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

        let error = client
            .wait_for_task(
                task,
                Some(Duration::from_millis(1)),
                Some(Duration::from_nanos(1)),
            )
            .await
            .unwrap_err();

        assert!(matches!(error, Error::Timeout));
        Ok(())
    }

    #[meilisearch_test]
    async fn test_async_sleep() {
        let sleep_duration = time::Duration::from_millis(10);
        let now = time::Instant::now();

        async_sleep(sleep_duration).await;

        assert!(now.elapsed() >= sleep_duration);
    }

    #[meilisearch_test]
    async fn test_failing_update(client: Client, movies: Index) -> Result<(), Error> {
        let task = movies.set_ranking_rules(["wrong_ranking_rule"]).await?;
        let status = client.wait_for_task(task, None, None).await?;

        let error = status.unwrap_failure();
        assert_eq!(error.error_code, ErrorCode::InvalidRankingRule);
        assert_eq!(error.error_type, ErrorType::InvalidRequest);
        Ok(())
    }
}
