//! The `dumps` module allows the creation of database dumps.
//!
//! - Dumps are `.dump` files that can be used to launch Meilisearch.
//!
//! - Dumps are compatible between Meilisearch versions.
//!
//! - Creating a dump is also referred to as exporting it, whereas launching Meilisearch with a dump is referred to as importing it.
//!
//! - During a [dump export](Client::create_dump), all [indexes](crate::indexes::Index) of the current instance are exported—together with their documents and settings—and saved as a single `.dump` file.
//!
//! - During a dump import, all indexes contained in the indicated `.dump` file are imported along with their associated documents and [settings](crate::settings::Settings).
//! Any existing [index](crate::indexes::Index) with the same uid as an index in the dump file will be overwritten.
//!
//! - Dump imports are [performed at launch](https://www.meilisearch.com/docs/learn/configuration/instance_options#import-dump) using an option.
//!
//! # Example
//!
//! ```no_run
//! # use meilisearch_sdk::{client::*, errors::*, dumps::*, dumps::*, task_info::*, tasks::*};
//! # use futures_await_test::async_test;
//! # use std::{thread::sleep, time::Duration};
//! # futures::executor::block_on(async move {
//! #
//! # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
//! # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
//! #
//! # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY));
//!
//! // Create a dump
//! let task_info = client.create_dump().await.unwrap();
//! assert!(matches!(
//!     task_info,
//!     TaskInfo {
//!         update_type: TaskType::DumpCreation { .. },
//!         ..
//!     }
//! ));
//! # });
//! ```

use crate::{request::*, Client, Error, TaskInfo};

/// Dump related methods.
/// See the [dumps](crate::dumps) module.
impl Client {
    /// Triggers a dump creation process.
    ///
    /// Once the process is complete, a dump is created in the [dumps directory](https://www.meilisearch.com/docs/learn/configuration/instance_options#dump-directory).
    /// If the dumps directory does not exist yet, it will be created.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use meilisearch_sdk::{client::*, errors::*, dumps::*, dumps::*, task_info::*, tasks::*};
    /// # use futures_await_test::async_test;
    /// # use std::{thread::sleep, time::Duration};
    /// # futures::executor::block_on(async move {
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY));
    /// #
    /// let task_info = client.create_dump().await.unwrap();
    ///
    /// assert!(matches!(
    ///     task_info,
    ///     TaskInfo {
    ///         update_type: TaskType::DumpCreation { .. },
    ///         ..
    ///     }
    /// ));
    /// # });
    /// ```
    pub async fn create_dump(&self) -> Result<TaskInfo, Error> {
        request::<(), (), TaskInfo>(
            &format!("{}/dumps", self.host),
            self.get_api_key(),
            Method::Post {
                query: (),
                body: (),
            },
            202,
        )
        .await
    }
}

/// Alias for [`create_dump`](Client::create_dump).
pub async fn create_dump(client: &Client) -> Result<TaskInfo, Error> {
    client.create_dump().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{client::*, tasks::*};
    use meilisearch_test_macro::meilisearch_test;
    use std::time::Duration;

    #[meilisearch_test]
    async fn test_dumps_success_creation(client: Client) -> Result<(), Error> {
        let task = client
            .create_dump()
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
    async fn test_dumps_correct_update_type(client: Client) -> Result<(), Error> {
        let task_info = client.create_dump().await.unwrap();

        assert!(matches!(
            task_info,
            TaskInfo {
                update_type: TaskType::DumpCreation { .. },
                ..
            }
        ));
        Ok(())
    }
}
