use crate::{
    errors::Error,
    indexes::Index,
    request::{request, Method},
    tasks::Task,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Struct reprensenting a set of settings.
/// You can build this struct using the builder syntax.
///
/// # Example
///
/// ```
/// # use meilisearch_sdk::settings::Settings;
/// let settings = Settings::new()
///     .with_stop_words(["a", "the", "of"]);
///
/// // OR
///
/// let stop_words: Vec<String> = vec!["a".to_string(), "the".to_string(), "of".to_string()];
/// let mut settings = Settings::new();
/// settings.stop_words = Some(stop_words);
///
/// // OR
///
/// let stop_words: Vec<String> = vec!["a".to_string(), "the".to_string(), "of".to_string()];
/// let settings = Settings {
///     stop_words: Some(stop_words),
///     ..Settings::new()
/// };
/// ```
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// List of associated words treated similarly
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<HashMap<String, Vec<String>>>,
    /// List of words ignored by Meilisearch when present in search queries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_words: Option<Vec<String>>,
    /// List of [ranking rules](https://docs.meilisearch.com/learn/core_concepts/relevancy.html#order-of-the-rules) sorted by order of importance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranking_rules: Option<Vec<String>>,
    /// Attributes to use for [filtering and faceted search](https://docs.meilisearch.com/reference/features/filtering_and_faceted_search.html)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filterable_attributes: Option<Vec<String>>,
    /// Attributes to sort
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sortable_attributes: Option<Vec<String>>,
    /// Search returns documents with distinct (different) values of the given field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distinct_attribute: Option<String>,
    /// Fields in which to search for matching query words sorted by order of importance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchable_attributes: Option<Vec<String>>,
    /// Fields displayed in the returned documents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub displayed_attributes: Option<Vec<String>>,
}

#[allow(missing_docs)]
impl Settings {
    /// Create undefined settings
    pub fn new() -> Settings {
        Settings {
            synonyms: None,
            stop_words: None,
            ranking_rules: None,
            filterable_attributes: None,
            sortable_attributes: None,
            distinct_attribute: None,
            searchable_attributes: None,
            displayed_attributes: None,
        }
    }
    pub fn with_synonyms<S, U, V>(self, synonyms: HashMap<S, U>) -> Settings
    where
        S: AsRef<str>,
        V: AsRef<str>,
        U: IntoIterator<Item = V>,
    {
        Settings {
            synonyms: Some(
                synonyms
                    .into_iter()
                    .map(|(key, value)| {
                        (
                            key.as_ref().to_string(),
                            value.into_iter().map(|v| v.as_ref().to_string()).collect(),
                        )
                    })
                    .collect(),
            ),
            ..self
        }
    }

    pub fn with_stop_words(
        self,
        stop_words: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Settings {
        Settings {
            stop_words: Some(
                stop_words
                    .into_iter()
                    .map(|v| v.as_ref().to_string())
                    .collect(),
            ),
            ..self
        }
    }

    pub fn with_ranking_rules(
        self,
        ranking_rules: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Settings {
        Settings {
            ranking_rules: Some(
                ranking_rules
                    .into_iter()
                    .map(|v| v.as_ref().to_string())
                    .collect(),
            ),
            ..self
        }
    }

    pub fn with_filterable_attributes(
        self,
        filterable_attributes: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Settings {
        Settings {
            filterable_attributes: Some(
                filterable_attributes
                    .into_iter()
                    .map(|v| v.as_ref().to_string())
                    .collect(),
            ),
            ..self
        }
    }

    pub fn with_sortable_attributes(
        self,
        sortable_attributes: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Settings {
        Settings {
            sortable_attributes: Some(
                sortable_attributes
                    .into_iter()
                    .map(|v| v.as_ref().to_string())
                    .collect(),
            ),
            ..self
        }
    }

    pub fn with_distinct_attribute(self, distinct_attribute: impl AsRef<str>) -> Settings {
        Settings {
            distinct_attribute: Some(distinct_attribute.as_ref().to_string()),
            ..self
        }
    }

    pub fn with_searchable_attributes(
        self,
        searchable_attributes: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Settings {
        Settings {
            searchable_attributes: Some(
                searchable_attributes
                    .into_iter()
                    .map(|v| v.as_ref().to_string())
                    .collect(),
            ),
            ..self
        }
    }

    pub fn with_displayed_attributes(
        self,
        displayed_attributes: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Settings {
        Settings {
            displayed_attributes: Some(
                displayed_attributes
                    .into_iter()
                    .map(|v| v.as_ref().to_string())
                    .collect(),
            ),
            ..self
        }
    }
}

impl Index {
    /// Get [Settings] of the [Index].
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// # client.create_index("get_settings", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let index = client.index("get_settings");
    /// let settings = index.get_settings().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_settings(&self) -> Result<Settings, Error> {
        request::<(), Settings>(
            &format!("{}/indexes/{}/settings", self.client.host, self.uid),
            &self.client.api_key,
            Method::Get,
            200,
        )
        .await
    }

    /// Get [synonyms](https://docs.meilisearch.com/reference/features/synonyms.html) of the [Index].
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", Some("masterKey"));
    /// # client.create_index("get_synonyms", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let index = client.index("get_synonyms");
    /// let synonyms = index.get_synonyms().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_synonyms(&self) -> Result<HashMap<String, Vec<String>>, Error> {
        request::<(), HashMap<String, Vec<String>>>(
            &format!(
                "{}/indexes/{}/settings/synonyms",
                self.client.host, self.uid
            ),
            &self.client.api_key,
            Method::Get,
            200,
        )
        .await
    }

    /// Get [stop-words](https://docs.meilisearch.com/reference/features/stop_words.html) of the [Index].
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", Some("masterKey"));
    /// # client.create_index("get_stop_words", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let index = client.index("get_stop_words");
    /// let stop_words = index.get_stop_words().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_stop_words(&self) -> Result<Vec<String>, Error> {
        request::<(), Vec<String>>(
            &format!(
                "{}/indexes/{}/settings/stop-words",
                self.client.host, self.uid
            ),
            &self.client.api_key,
            Method::Get,
            200,
        )
        .await
    }

    /// Get [ranking rules](https://docs.meilisearch.com/learn/core_concepts/relevancy.html#ranking-rules) of the [Index].
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", Some("masterKey"));
    /// # client.create_index("get_ranking_rules", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let index = client.index("get_ranking_rules");
    /// let ranking_rules = index.get_ranking_rules().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_ranking_rules(&self) -> Result<Vec<String>, Error> {
        request::<(), Vec<String>>(
            &format!(
                "{}/indexes/{}/settings/ranking-rules",
                self.client.host, self.uid
            ),
            &self.client.api_key,
            Method::Get,
            200,
        )
        .await
    }

    /// Get [filterable attributes](https://docs.meilisearch.com/reference/features/filtering_and_faceted_search.html) of the [Index].
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", Some("masterKey"));
    /// # client.create_index("get_filterable_attributes", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let index = client.index("get_filterable_attributes");
    /// let filterable_attributes = index.get_filterable_attributes().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_filterable_attributes(&self) -> Result<Vec<String>, Error> {
        request::<(), Vec<String>>(
            &format!(
                "{}/indexes/{}/settings/filterable-attributes",
                self.client.host, self.uid
            ),
            &self.client.api_key,
            Method::Get,
            200,
        )
        .await
    }

    /// Get [sortable attributes](https://docs.meilisearch.com/reference/features/sorting.html) of the [Index].
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", Some("masterKey"));
    /// # client.create_index("get_sortable_attributes", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let index = client.index("get_sortable_attributes");
    /// let sortable_attributes = index.get_sortable_attributes().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_sortable_attributes(&self) -> Result<Vec<String>, Error> {
        request::<(), Vec<String>>(
            &format!(
                "{}/indexes/{}/settings/sortable-attributes",
                self.client.host, self.uid
            ),
            &self.client.api_key,
            Method::Get,
            200,
        )
        .await
    }

    /// Get the [distinct attribute](https://docs.meilisearch.com/reference/features/settings.html#distinct-attribute) of the [Index].
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", Some("masterKey"));
    /// # client.create_index("get_distinct_attribute", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let index = client.index("get_distinct_attribute");
    /// let distinct_attribute = index.get_distinct_attribute().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_distinct_attribute(&self) -> Result<Option<String>, Error> {
        request::<(), Option<String>>(
            &format!(
                "{}/indexes/{}/settings/distinct-attribute",
                self.client.host, self.uid
            ),
            &self.client.api_key,
            Method::Get,
            200,
        )
        .await
    }

    /// Get [searchable attributes](https://docs.meilisearch.com/reference/features/field_properties.html#searchable-fields) of the [Index].
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", Some("masterKey"));
    /// # client.create_index("get_searchable_attributes", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let index = client.index("get_searchable_attributes");
    /// let searchable_attributes = index.get_searchable_attributes().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_searchable_attributes(&self) -> Result<Vec<String>, Error> {
        request::<(), Vec<String>>(
            &format!(
                "{}/indexes/{}/settings/searchable-attributes",
                self.client.host, self.uid
            ),
            &self.client.api_key,
            Method::Get,
            200,
        )
        .await
    }

    /// Get [displayed attributes](https://docs.meilisearch.com/reference/features/settings.html#displayed-attributes) of the [Index].
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", Some("masterKey"));
    /// # client.create_index("get_displayed_attributes", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let index = client.index("get_displayed_attributes");
    /// let displayed_attributes = index.get_displayed_attributes().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_displayed_attributes(&self) -> Result<Vec<String>, Error> {
        request::<(), Vec<String>>(
            &format!(
                "{}/indexes/{}/settings/displayed-attributes",
                self.client.host, self.uid
            ),
            &self.client.api_key,
            Method::Get,
            200,
        )
        .await
    }

    /// Update [settings](../settings/struct.Settings.html) of the [Index].
    /// Updates in the settings are partial. This means that any parameters corresponding to a `None` value will be left unchanged.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", Some("masterKey"));
    /// # client.create_index("set_settings", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("set_settings");
    ///
    /// let stop_words = vec![String::from("a"), String::from("the"), String::from("of")];
    /// let settings = Settings::new()
    ///     .with_stop_words(stop_words.clone());
    ///
    /// let task = index.set_settings(&settings).await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn set_settings(&self, settings: &Settings) -> Result<Task, Error> {
        request::<&Settings, Task>(
            &format!("{}/indexes/{}/settings", self.client.host, self.uid),
            &self.client.api_key,
            Method::Post(settings),
            202,
        )
        .await
    }

    /// Update [synonyms](https://docs.meilisearch.com/reference/features/synonyms.html) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", Some("masterKey"));
    /// # client.create_index("set_synonyms", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("set_synonyms");
    ///
    /// let mut synonyms = std::collections::HashMap::new();
    /// synonyms.insert(String::from("wolverine"), vec![String::from("xmen"), String::from("logan")]);
    /// synonyms.insert(String::from("logan"), vec![String::from("xmen"), String::from("wolverine")]);
    /// synonyms.insert(String::from("wow"), vec![String::from("world of warcraft")]);
    ///
    /// let task = index.set_synonyms(&synonyms).await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn set_synonyms(
        &self,
        synonyms: &HashMap<String, Vec<String>>,
    ) -> Result<Task, Error> {
        request::<&HashMap<String, Vec<String>>, Task>(
            &format!(
                "{}/indexes/{}/settings/synonyms",
                self.client.host, self.uid
            ),
            &self.client.api_key,
            Method::Post(synonyms),
            202,
        )
        .await
    }

    /// Update [stop-words](https://docs.meilisearch.com/reference/features/stop_words.html) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", Some("masterKey"));
    /// # client.create_index("set_stop_words", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("set_stop_words");
    ///
    /// let stop_words = ["the", "of", "to"];
    /// let task = index.set_stop_words(&stop_words).await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn set_stop_words(
        &self,
        stop_words: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<Task, Error> {
        request::<Vec<String>, Task>(
            &format!(
                "{}/indexes/{}/settings/stop-words",
                self.client.host, self.uid
            ),
            &self.client.api_key,
            Method::Post(
                stop_words
                    .into_iter()
                    .map(|v| v.as_ref().to_string())
                    .collect(),
            ),
            202,
        )
        .await
    }

    /// Update [ranking rules](https://docs.meilisearch.com/learn/core_concepts/relevancy.html#ranking-rules) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", Some("masterKey"));
    /// # client.create_index("set_ranking_rules", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("set_ranking_rules");
    ///
    /// let ranking_rules = [
    ///     "words",
    ///     "typo",
    ///     "proximity",
    ///     "attribute",
    ///     "sort",
    ///     "exactness",
    ///     "release_date:asc",
    ///     "rank:desc",
    /// ];
    /// let task = index.set_ranking_rules(ranking_rules).await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn set_ranking_rules(
        &self,
        ranking_rules: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<Task, Error> {
        request::<Vec<String>, Task>(
            &format!(
                "{}/indexes/{}/settings/ranking-rules",
                self.client.host, self.uid
            ),
            &self.client.api_key,
            Method::Post(
                ranking_rules
                    .into_iter()
                    .map(|v| v.as_ref().to_string())
                    .collect(),
            ),
            202,
        )
        .await
    }

    /// Update [filterable attributes](https://docs.meilisearch.com/reference/features/filtering_and_faceted_search.html) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", Some("masterKey"));
    /// # client.create_index("set_filterable_attributes", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("set_filterable_attributes");
    ///
    /// let filterable_attributes = ["genre", "director"];
    /// let task = index.set_filterable_attributes(&filterable_attributes).await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn set_filterable_attributes(
        &self,
        filterable_attributes: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<Task, Error> {
        request::<Vec<String>, Task>(
            &format!(
                "{}/indexes/{}/settings/filterable-attributes",
                self.client.host, self.uid
            ),
            &self.client.api_key,
            Method::Post(
                filterable_attributes
                    .into_iter()
                    .map(|v| v.as_ref().to_string())
                    .collect(),
            ),
            202,
        )
        .await
    }

    /// Update [sortable attributes](https://docs.meilisearch.com/reference/features/sorting.html) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", Some("masterKey"));
    /// # client.create_index("set_sortable_attributes", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("set_sortable_attributes");
    ///
    /// let sortable_attributes = ["genre", "director"];
    /// let task = index.set_sortable_attributes(&sortable_attributes).await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn set_sortable_attributes(
        &self,
        sortable_attributes: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<Task, Error> {
        request::<Vec<String>, Task>(
            &format!(
                "{}/indexes/{}/settings/sortable-attributes",
                self.client.host, self.uid
            ),
            &self.client.api_key,
            Method::Post(
                sortable_attributes
                    .into_iter()
                    .map(|v| v.as_ref().to_string())
                    .collect(),
            ),
            202,
        )
        .await
    }

    /// Update the [distinct attribute](https://docs.meilisearch.com/reference/features/settings.html#distinct-attribute) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", Some("masterKey"));
    /// # client.create_index("set_distinct_attribute", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("set_distinct_attribute");
    ///
    /// let task = index.set_distinct_attribute("movie_id").await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn set_distinct_attribute(
        &self,
        distinct_attribute: impl AsRef<str>,
    ) -> Result<Task, Error> {
        request::<String, Task>(
            &format!(
                "{}/indexes/{}/settings/distinct-attribute",
                self.client.host, self.uid
            ),
            &self.client.api_key,
            Method::Post(distinct_attribute.as_ref().to_string()),
            202,
        )
        .await
    }

    /// Update [searchable attributes](https://docs.meilisearch.com/reference/features/field_properties.html#searchable-fields) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", Some("masterKey"));
    /// # client.create_index("set_searchable_attributes", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("set_searchable_attributes");
    ///
    /// let task = index.set_searchable_attributes(["title", "description", "uid"]).await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn set_searchable_attributes(
        &self,
        searchable_attributes: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<Task, Error> {
        request::<Vec<String>, Task>(
            &format!(
                "{}/indexes/{}/settings/searchable-attributes",
                self.client.host, self.uid
            ),
            &self.client.api_key,
            Method::Post(
                searchable_attributes
                    .into_iter()
                    .map(|v| v.as_ref().to_string())
                    .collect(),
            ),
            202,
        )
        .await
    }

    /// Update [displayed attributes](https://docs.meilisearch.com/reference/features/settings.html#displayed-attributes) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", Some("masterKey"));
    /// # client.create_index("set_displayed_attributes", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("set_displayed_attributes");
    ///
    /// let task = index.set_displayed_attributes(["title", "description", "release_date", "rank", "poster"]).await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn set_displayed_attributes(
        &self,
        displayed_attributes: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<Task, Error> {
        request::<Vec<String>, Task>(
            &format!(
                "{}/indexes/{}/settings/displayed-attributes",
                self.client.host, self.uid
            ),
            &self.client.api_key,
            Method::Post(
                displayed_attributes
                    .into_iter()
                    .map(|v| v.as_ref().to_string())
                    .collect(),
            ),
            202,
        )
        .await
    }

    /// Reset [Settings] of the [Index].
    /// All settings will be reset to their [default value](https://docs.meilisearch.com/reference/api/settings.html#reset-settings).
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", Some("masterKey"));
    /// # client.create_index("reset_settings", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("reset_settings");
    ///
    /// let task = index.reset_settings().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn reset_settings(&self) -> Result<Task, Error> {
        request::<(), Task>(
            &format!("{}/indexes/{}/settings", self.client.host, self.uid),
            &self.client.api_key,
            Method::Delete,
            202,
        )
        .await
    }

    /// Reset [synonyms](https://docs.meilisearch.com/reference/features/synonyms.html) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", Some("masterKey"));
    /// # client.create_index("reset_synonyms", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("reset_synonyms");
    ///
    /// let task = index.reset_synonyms().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn reset_synonyms(&self) -> Result<Task, Error> {
        request::<(), Task>(
            &format!(
                "{}/indexes/{}/settings/synonyms",
                self.client.host, self.uid
            ),
            &self.client.api_key,
            Method::Delete,
            202,
        )
        .await
    }

    /// Reset [stop-words](https://docs.meilisearch.com/reference/features/stop_words.html) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", Some("masterKey"));
    /// # client.create_index("reset_stop_words", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("reset_stop_words");
    ///
    /// let task = index.reset_stop_words().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn reset_stop_words(&self) -> Result<Task, Error> {
        request::<(), Task>(
            &format!(
                "{}/indexes/{}/settings/stop-words",
                self.client.host, self.uid
            ),
            &self.client.api_key,
            Method::Delete,
            202,
        )
        .await
    }

    /// Reset [ranking rules](https://docs.meilisearch.com/learn/core_concepts/relevancy.html#ranking-rules) of the [Index] to default value.
    /// Default value: `["words", "typo", "proximity", "attribute", "sort", "exactness"]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", Some("masterKey"));
    /// # client.create_index("reset_ranking_rules", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("reset_ranking_rules");
    ///
    /// let task = index.reset_ranking_rules().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn reset_ranking_rules(&self) -> Result<Task, Error> {
        request::<(), Task>(
            &format!(
                "{}/indexes/{}/settings/ranking-rules",
                self.client.host, self.uid
            ),
            &self.client.api_key,
            Method::Delete,
            202,
        )
        .await
    }

    /// Reset [filterable attributes](https://docs.meilisearch.com/reference/features/filtering_and_faceted_search.html) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", Some("masterKey"));
    /// # client.create_index("reset_filterable_attributes", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("reset_filterable_attributes");
    ///
    /// let task = index.reset_filterable_attributes().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn reset_filterable_attributes(&self) -> Result<Task, Error> {
        request::<(), Task>(
            &format!(
                "{}/indexes/{}/settings/filterable-attributes",
                self.client.host, self.uid
            ),
            &self.client.api_key,
            Method::Delete,
            202,
        )
        .await
    }

    /// Reset [sortable attributes](https://docs.meilisearch.com/reference/features/sorting.html) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// # futures::executor::block_on(async move {
   /// let client = Client::new("http://localhost:7700", Some("masterKey"));
    /// # client.create_index("reset_sortable_attributes", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("reset_sortable_attributes");
    ///
    /// let task = index.reset_sortable_attributes().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn reset_sortable_attributes(&self) -> Result<Task, Error> {
        request::<(), Task>(
            &format!(
                "{}/indexes/{}/settings/sortable-attributes",
                self.client.host, self.uid
            ),
            &self.client.api_key,
            Method::Delete,
            202,
        )
        .await
    }

    /// Reset the [distinct attribute](https://docs.meilisearch.com/reference/features/settings.html#distinct-attribute) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", Some("masterKey"));
    /// # client.create_index("reset_distinct_attribute", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("reset_distinct_attribute");
    ///
    /// let task = index.reset_distinct_attribute().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn reset_distinct_attribute(&self) -> Result<Task, Error> {
        request::<(), Task>(
            &format!(
                "{}/indexes/{}/settings/distinct-attribute",
                self.client.host, self.uid
            ),
            &self.client.api_key,
            Method::Delete,
            202,
        )
        .await
    }

    /// Reset [searchable attributes](https://docs.meilisearch.com/reference/features/field_properties.html#searchable-fields) of the [Index] (enable all attributes).
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", Some("masterKey"));
    /// # client.create_index("reset_searchable_attributes", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("reset_searchable_attributes");
    ///
    /// let task = index.reset_searchable_attributes().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn reset_searchable_attributes(&self) -> Result<Task, Error> {
        request::<(), Task>(
            &format!(
                "{}/indexes/{}/settings/searchable-attributes",
                self.client.host, self.uid
            ),
            &self.client.api_key,
            Method::Delete,
            202,
        )
        .await
    }

    /// Reset [displayed attributes](https://docs.meilisearch.com/reference/features/settings.html#displayed-attributes) of the [Index] (enable all attributes).
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", Some("masterKey"));
    /// # client.create_index("reset_displayed_attributes", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("reset_displayed_attributes");
    ///
    /// let task = index.reset_displayed_attributes().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn reset_displayed_attributes(&self) -> Result<Task, Error> {
        request::<(), Task>(
            &format!(
                "{}/indexes/{}/settings/displayed-attributes",
                self.client.host, self.uid
            ),
            &self.client.api_key,
            Method::Delete,
            202,
        )
        .await
    }
}
