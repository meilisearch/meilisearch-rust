use serde::Deserialize;
use std::time::Duration;
use time::OffsetDateTime;

use crate::{client::Client, errors::Error, tasks::*};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskInfo {
    #[serde(with = "time::serde::rfc3339")]
    pub enqueued_at: OffsetDateTime,
    pub index_uid: String,
    pub status: String,
    #[serde(flatten)]
    pub update_type: TaskType,
    pub task_uid: u32,
}

impl AsRef<u32> for TaskInfo {
    fn as_ref(&self) -> &u32 {
        &self.task_uid
    }
}

impl AsRef<str> for TaskInfo {
    fn as_ref(&self) -> &str {
        self.get_index_uid()
    }
}

impl TaskInfo {
    pub fn get_task_uid(&self) -> u32 {
        self.task_uid
    }

    pub fn get_index_uid(&self) -> &str {
        &self.index_uid
    }

    /// Wait until Meilisearch processes a task provided by [TaskInfo], and get its status.
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
    /// # use meilisearch_sdk::{client::*, indexes::*, tasks::Task, task_info::TaskInfo};
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
    /// let client = Client::new("http://localhost:7700", "masterKey");
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
    ) -> Result<Task, Error> {
        client.wait_for_task(self, interval, timeout).await
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        client::*,
        errors::{ErrorCode, ErrorType},
        indexes::Index,
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
    fn test_deserialize_task_info() {
        let datetime = OffsetDateTime::parse(
            "2022-02-03T13:02:38.369634Z",
            &::time::format_description::well_known::Rfc3339,
        )
        .unwrap();

        let task_info: TaskInfo = serde_json::from_str(
            r#"
{
  "enqueuedAt": "2022-02-03T13:02:38.369634Z",
  "indexUid": "mieli",
  "status": "enqueued",
  "type": "documentAdditionOrUpdate",
  "taskUid": 12
}"#,
        )
        .unwrap();

        assert!(matches!(
            task_info,
            TaskInfo {
                enqueued_at,
                index_uid,
                task_uid: 12,
                update_type: TaskType::DocumentAdditionOrUpdate { details: None },
                status: _,
            }
        if enqueued_at == datetime && index_uid == "mieli"));
    }

    #[meilisearch_test]
    async fn test_wait_for_pending_updates_with_args(
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
    async fn test_wait_for_pending_updates_time_out(
        client: Client,
        movies: Index,
    ) -> Result<(), Error> {
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

        let error = client
            .wait_for_task(
                task_info,
                Some(Duration::from_millis(1)),
                Some(Duration::from_nanos(1)),
            )
            .await
            .unwrap_err();

        assert!(matches!(error, Error::Timeout));
        Ok(())
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
