use std::collections::HashMap;

use serde::Serialize;

use crate::{client::Client, errors::Error, request::*, task_info::TaskInfo};

/// Represents an export query, used to migrate between Meilisearch instances.
///
/// Body fields can be added via the builder pattern.
/// See [this page](https://www.meilisearch.com/docs/reference/api/export) for details.
///
/// # Example
///
/// ```
/// # use serde::{Serialize, Deserialize};
/// # use meilisearch_sdk::{client::Client, export::*, indexes::Index, tasks::TaskType, task_info::*};
/// #
/// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
/// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
/// #
/// # let MEILISEARCH_EXPORT_URL = option_env!("MEILISEARCH_EXPORT_URL").unwrap_or("http://localhost:7701");
/// # let MEILISEARCH_EXPORT_API_KEY = option_env!("MEILISEARCH_EXPORT_API_KEY").unwrap_or("masterKey");
/// #
/// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
/// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
/// let task_info = ExportQuery::new(
///         &client,
///         MEILISEARCH_EXPORT_URL,
///         MEILISEARCH_EXPORT_API_KEY,
/// )
/// .with_payload_size("50 MiB")
/// .execute()
/// .await
/// .unwrap();
///
/// assert!(matches!(
///     task_info,
///     TaskInfo {
///         update_type: TaskType::Export { .. },
///         ..
///     }
/// ));
/// #
/// # });
/// ```
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExportQuery<'a, Http: HttpClient> {
    #[serde(skip_serializing)]
    pub client: &'a Client<Http>,
    pub url: String,
    pub api_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexes: Option<HashMap<String, ExportQueryIndexOptions>>,
}

/// Export options for indexes
#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExportQueryIndexOptions {
    /// a [filter expression](https://www.meilisearch.com/docs/learn/filtering_and_sorting/filter_expression_reference) defining the subset of documents to export. Defaults to `null`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,

    ///  if `true`, configures indexes in the target instance with the origin instance settings. Defaults to `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_settings: Option<bool>,
}

impl<'a, Http: HttpClient> ExportQuery<'a, Http> {
    /// Create a new `ExportQuery`
    ///
    /// See [this page](https://www.meilisearch.com/docs/reference/api/export) for details.
    ///
    /// # Example
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::{client::Client, export::*, indexes::Index, tasks::TaskType, task_info::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// let export_query = ExportQuery::new(
    ///         &client,
    ///         "https://some-remote-meilisearch.instance",
    ///         "masterKey",
    /// );
    /// # });
    /// ```
    #[must_use]
    pub fn new(
        client: &'a Client<Http>,
        target_url: impl AsRef<str>,
        target_api_key: impl AsRef<str>,
    ) -> Self {
        Self {
            client,
            url: target_url.as_ref().to_string(),
            api_key: target_api_key.as_ref().to_string(),
            payload_size: None,
            indexes: None,
        }
    }

    /// The maximum size of each single data payload in a human-readable format such as `"100MiB"`.
    pub fn with_payload_size(&mut self, payload_size: impl AsRef<str>) -> &mut Self {
        self.payload_size = Some(payload_size.as_ref().to_string());

        self
    }

    /// Patterns matching indexes you want to export, along with export settings for each one
    /// # Example
    ///
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::{client::Client, export::*, indexes::Index, tasks::TaskType, task_info::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// let export_query = ExportQuery::new(
    ///         &client,
    ///         "https://some-remote-meilisearch.instance",
    ///         "masterKey",
    /// )
    /// .with_indexes([
    ///     ("user_*", ExportQueryIndexOptions {
    ///         filter: Some("name EXISTS".to_string()),
    ///         override_settings: Some(false)
    ///     }),
    ///     ("movies", ExportQueryIndexOptions {
    ///         filter: Some("genres = horror OR genres = comedy".to_string()),
    ///         override_settings: Some(true)
    ///     }),
    ///     ("orders", ExportQueryIndexOptions::default())
    /// ]);
    /// # });
    /// ```
    pub fn with_indexes(
        &mut self,
        indexes: impl IntoIterator<Item = (impl AsRef<str>, ExportQueryIndexOptions)>,
    ) -> &mut Self {
        self.indexes = Some(
            indexes
                .into_iter()
                .map(|(index, options)| (index.as_ref().to_string(), options))
                .collect(),
        );

        self
    }

    /// Execute the export query
    ///
    /// # Example
    /// ```
    /// # use serde::{Serialize, Deserialize};
    /// # use meilisearch_sdk::{client::Client, export::*, indexes::Index, tasks::TaskType, task_info::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # let MEILISEARCH_EXPORT_URL = option_env!("MEILISEARCH_EXPORT_URL").unwrap_or("http://localhost:7701");
    /// # let MEILISEARCH_EXPORT_API_KEY = option_env!("MEILISEARCH_EXPORT_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// let task_info = ExportQuery::new(
    ///         &client,
    ///         MEILISEARCH_EXPORT_URL,
    ///         MEILISEARCH_EXPORT_API_KEY,
    /// )
    /// .with_payload_size("50 MiB")
    /// .execute()
    /// .await
    /// .unwrap();
    ///
    /// assert!(matches!(
    ///     task_info,
    ///     TaskInfo {
    ///         update_type: TaskType::Export { .. },
    ///         ..
    ///     }
    /// ));
    /// #
    /// # });
    /// ```
    pub async fn execute(&self) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), &Self, TaskInfo>(
                &format!("{}/export", self.client.host),
                Method::Post {
                    query: (),
                    body: self,
                },
                200,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{client::*, indexes::Index};
    use meilisearch_test_macro::meilisearch_test;

    fn get_export_client() -> Result<Client, Error> {
        let export_url = option_env!("MEILISEARCH_EXPORT_URL").unwrap_or("http://localhost:7701");
        let export_api_key = option_env!("MEILISEARCH_EXPORT_API_KEY").unwrap_or("masterKey");

        Client::new(export_url, Some(export_api_key))
    }

    #[meilisearch_test]
    async fn test_export_with_default_options(client: Client, index: Index) -> Result<(), Error> {
        let export_client = get_export_client()?;

        let export_result = ExportQuery::new(
            &client,
            &export_client.host,
            export_client.api_key.as_ref().unwrap(),
        )
        .execute()
        .await
        .unwrap()
        .wait_for_completion(&client, None, None)
        .await;

        assert!(export_result.is_ok());

        let exported_index = export_client.get_index(&index.uid).await;
        assert!(exported_index.is_ok());

        export_client
            .wait_for_task(exported_index.unwrap().delete().await.unwrap(), None, None)
            .await
            .unwrap();

        Ok(())
    }

    #[meilisearch_test]
    async fn test_export(client: Client, index: Index) -> Result<(), Error> {
        let export_client = get_export_client()?;

        let export_result = ExportQuery::new(
            &client,
            &export_client.host,
            export_client.api_key.as_ref().unwrap(),
        )
        .with_indexes([(&index.uid, ExportQueryIndexOptions::default())])
        .with_payload_size("50 MiB")
        .execute()
        .await
        .unwrap()
        .wait_for_completion(&client, None, None)
        .await;

        assert!(export_result.is_ok());

        let exported_index = export_client.get_index(&index.uid).await;
        assert!(exported_index.is_ok());

        export_client
            .wait_for_task(exported_index.unwrap().delete().await.unwrap(), None, None)
            .await
            .unwrap();

        Ok(())
    }
}
