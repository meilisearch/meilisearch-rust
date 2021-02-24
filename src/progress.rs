#![allow(missing_docs)]

use crate::{errors::Error, indexes::Index, request::*};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::time::Duration;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProgressJson {
    pub(crate) update_id: usize,
}

impl ProgressJson {
    pub(crate) fn into_progress<'a>(self, index: &'a Index) -> Progress<'a> {
        Progress {
            id: self.update_id,
            index,
        }
    }
}

/// A struct used to track the progress of some async operations.
pub struct Progress<'a> {
    id: usize,
    index: &'a Index<'a>,
}

impl<'a> Progress<'a> {
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movies_index = client.get_or_create("movies").await.unwrap();
    /// let progress = movies_index.delete_all_documents().await.unwrap();
    /// let status = progress.get_status().await.unwrap();
    /// # client.delete_index("movies").await.unwrap();
    /// # });
    /// ```
    pub async fn get_status(&self) -> Result<UpdateStatus, Error> {
        request::<(), UpdateStatus>(
            &format!(
                "{}/indexes/{}/updates/{}",
                self.index.client.host, self.index.uid, self.id
            ),
            self.index.client.apikey,
            Method::Get,
            200,
        )
        .await
    }

    /// Wait until MeiliSearch processes an update, and get its status.
    /// 
    /// interval_ms = The frequency at which the server should be polled. Default = 50ms
    /// timeout_ms = The maximum time to wait for processing to complete. Default = 5000ms
    /// 
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, document, indexes::*, progress::*};
    /// # use serde::{Serialize, Deserialize};
    /// #
    /// # #[derive(Debug, Serialize, Deserialize, PartialEq)]
    /// # struct Document {
    /// #    id: usize,
    /// #    value: String,
    /// #    kind: String,
    /// # }
    /// # 
    /// # impl document::Document for Document {
    /// #    type UIDType = usize;
    /// #
    /// #    fn get_uid(&self) -> &Self::UIDType {
    /// #        &self.id
    /// #    }
    /// # }
    /// #
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let movies = client.create_index("movies_wait_for_pending", None).await.unwrap();
    /// 
    /// let progress = movies.add_documents(&[
    ///     Document { id: 0, kind: "title".into(), value: "The Social Network".to_string() },
    ///     Document { id: 1, kind: "title".into(), value: "Harry Potter and the Sorcerer's Stone".to_string() },
    /// ], None).await.unwrap();
    /// 
    /// let status = progress.wait_for_pending_update(None, None).await.unwrap();
    /// 
    /// # client.delete_index("movies_wait_for_pending").await.unwrap();
    /// assert!(matches!(status, UpdateStatus::Processed { .. }));
    /// # });
    /// ```
    pub async fn wait_for_pending_update(
        &self,
        interval_ms: Option<Duration>,
        timeout_ms: Option<Duration>,
    ) -> Result<UpdateStatus, Error> {
        let interval: Duration;
        let timeout: Duration;

        match interval_ms {
            Some(v) => interval = v,
            None => interval = Duration::from_millis(50),
        }

        match timeout_ms {
            Some(v) => timeout = v,
            None => timeout = Duration::from_millis(5000),
        }

        let mut elapsed_time = Duration::new(0, 0);
        let mut status: UpdateStatus;

        while timeout > elapsed_time {
            status = self.get_status().await?;

            match status {
                UpdateStatus::Failed { .. } | UpdateStatus::Processed { .. } => {
                    return self.get_status().await;
                },
                UpdateStatus::Enqueued { .. } => {
                    elapsed_time += interval;
                    async_sleep(interval).await;
                },
            };
        }

        Err(
            Error::MeiliSearchTimeoutError {
                message: format!(
                    "timeout of {:?}ms has been exceeded when waiting for pending update to resolve.",
                    timeout,
                ),
            }
        )
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
    use wasm_bindgen_futures::JsFuture;

    JsFuture::from(js_sys::Promise::new(&mut |yes, _| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &yes,
                interval.as_millis() as i32,
            )
            .unwrap();
    })).await.unwrap();
}

#[derive(Debug, Clone, Deserialize)]
pub enum RankingRule {
    Typo,
    Words,
    Proximity,
    Attribute,
    WordsPosition,
    Exactness,
    Asc(String),
    Desc(String),
}

#[derive(Debug, Clone, Deserialize)]
pub enum UpdateState<T> {
    Update(T),
    Clear,
    Nothing,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SettingsUpdate {
    pub ranking_rules: UpdateState<Vec<RankingRule>>,
    pub distinct_attribute: UpdateState<String>,
    pub searchable_attributes: UpdateState<Vec<String>>,
    pub displayed_attributes: UpdateState<BTreeSet<String>>,
    pub stop_words: UpdateState<BTreeSet<String>>,
    pub synonyms: UpdateState<BTreeMap<String, Vec<String>>>,
    pub attributes_for_faceting: UpdateState<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "name")]
pub enum UpdateType {
    ClearAll,
    Customs,
    DocumentsAddition { number: usize },
    DocumentsPartial { number: usize },
    DocumentsDeletion { number: usize },
    Settings { settings: Box<SettingsUpdate> },
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProcessedUpdateResult {
    pub update_id: u64,
    #[serde(rename = "type")]
    pub update_type: UpdateType,
    pub error: Option<String>,
    pub error_type: Option<String>,
    pub error_code: Option<String>,
    pub error_link: Option<String>,
    pub duration: f64,        // in seconds
    pub enqueued_at: String,  // TODO deserialize to datetime
    pub processed_at: String, // TODO deserialize to datetime
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnqueuedUpdateResult {
    pub update_id: u64,
    #[serde(rename = "type")]
    pub update_type: UpdateType,
    pub enqueued_at: String, // TODO deserialize to datetime
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", tag = "status")]
pub enum UpdateStatus {
    Enqueued {
        #[serde(flatten)]
        content: EnqueuedUpdateResult,
    },
    Failed {
        #[serde(flatten)]
        content: ProcessedUpdateResult,
    },
    Processed {
        #[serde(flatten)]
        content: ProcessedUpdateResult,
    },
}

#[cfg(test)]
mod test {
    use crate::{client::*, document, progress::*};
    use serde::{Serialize, Deserialize};
    use futures_await_test::async_test;
    use std::time;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Document {
       id: usize,
       value: String,
       kind: String,
    }
    
    impl document::Document for Document {
       type UIDType = usize;
    
       fn get_uid(&self) -> &Self::UIDType {
           &self.id
       }
    }

    #[async_test]
    async fn test_wait_for_pending_updates_with_args() {
        let client = Client::new("http://localhost:7700", "masterKey");
        let movies = client.create_index("movies_wait_for_pending_args", None).await.unwrap();
        let progress = movies.add_documents(&[
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
        ], None).await.unwrap();
        let status = progress.wait_for_pending_update(
            Some(Duration::from_millis(1)), Some(Duration::from_millis(6000))
        ).await.unwrap();
    
        client.delete_index("movies_wait_for_pending_args").await.unwrap();
        assert!(matches!(status, UpdateStatus::Processed { .. }));
    }

    #[async_test]
    async fn test_async_sleep() {
        let sleep_duration = time::Duration::from_millis(10);
        let now = time::Instant::now();

        async_sleep(sleep_duration).await;

        assert!(now.elapsed() >= sleep_duration);
    }
}
