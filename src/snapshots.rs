//! The `snapshots` module allows the creation of database snapshots.
//!
//! - snapshots are `.snapshots` files that can be used to launch Meilisearch.
//!
//! - snapshots are not compatible between Meilisearch versions.
//!
//! # Example
//!
//! ```
//! # use meilisearch_sdk::{client::*, errors::*, snapshots::*, snapshots::*, task_info::*, tasks::*};
//! # use futures_await_test::async_test;
//! # use std::{thread::sleep, time::Duration};
//! # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
//! #
//! # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
//! # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
//! #
//! # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
//!
//! // Create a snapshot
//! let task_info = client.create_snapshot().await.unwrap();
//! assert!(matches!(
//!     task_info,
//!     TaskInfo {
//!         update_type: TaskType::SnapshotCreation { .. },
//!         ..
//!     }
//! ));
//! # });
//! ```

use crate::{client::Client, errors::Error, request::*, task_info::TaskInfo};

/// Snapshots related methods.
/// See the [snapshots](crate::snapshots) module.
impl<Http: HttpClient> Client<Http> {
    /// Triggers a snapshots creation process.
    ///
    /// Once the process is complete, a snapshots is created in the [snapshots directory].
    /// If the snapshots directory does not exist yet, it will be created.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, errors::*, snapshots::*, snapshots::*, task_info::*, tasks::*};
    /// # use futures_await_test::async_test;
    /// # use std::{thread::sleep, time::Duration};
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// #
    /// let task_info = client.create_snapshot().await.unwrap();
    ///
    /// assert!(matches!(
    ///     task_info,
    ///     TaskInfo {
    ///         update_type: TaskType::SnapshotCreation { .. },
    ///         ..
    ///     }
    /// ));
    /// # });
    /// ```
    pub async fn create_snapshot(&self) -> Result<TaskInfo, Error> {
        self.http_client
            .request::<(), (), TaskInfo>(
                &format!("{}/snapshots", self.host),
                Method::Post {
                    query: (),
                    body: (),
                },
                202,
            )
            .await
    }
}

/// Alias for [`create_snapshot`](Client::create_snapshot).
pub async fn create_snapshot<Http: HttpClient>(client: &Client<Http>) -> Result<TaskInfo, Error> {
    client.create_snapshot().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{client::*, tasks::*};
    use meilisearch_test_macro::meilisearch_test;

    #[meilisearch_test]
    async fn test_snapshot_success_creation(client: Client) -> Result<(), Error> {
        let task = client
            .create_snapshot()
            .await?
            .wait_for_completion(&client, None, None)
            .await?;

        assert!(matches!(task, Task::Succeeded { .. }));
        Ok(())
    }

    #[meilisearch_test]
    async fn test_snapshot_correct_update_type(client: Client) -> Result<(), Error> {
        let task_info = client.create_snapshot().await.unwrap();

        assert!(matches!(
            task_info,
            TaskInfo {
                update_type: TaskType::SnapshotCreation { .. },
                ..
            }
        ));
        Ok(())
    }
}
