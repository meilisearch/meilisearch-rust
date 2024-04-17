use crate::{
    errors::Error,
    indexes::Index,
    request::{HttpClient, Method},
    task_info::TaskInfo,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "camelCase")]
pub struct PaginationSetting {
    pub max_total_hits: usize,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MinWordSizeForTypos {
    pub one_typo: Option<u8>,
    pub two_typos: Option<u8>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct TypoToleranceSettings {
    pub enabled: Option<bool>,
    pub disable_on_attributes: Option<Vec<String>>,
    pub disable_on_words: Option<Vec<String>>,
    pub min_word_size_for_typos: Option<MinWordSizeForTypos>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Eq, PartialEq, Copy)]
#[serde(rename_all = "camelCase")]
pub struct FacetingSettings {
    pub max_values_per_facet: usize,
}

/// EXPERIMENTAL
/// Allows configuring semantic seaarching
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase", tag = "source")]
pub enum Embedder {
    /// Compute embeddings locally.
    /// This is a resource-intensive operation and might affect indexing performance.
    HuggingFace(HuggingFaceEmbedderSettings),
    /// Use OpenAi's API to generate embeddings
    OpenAi(OpenapiEmbedderSettings),
    /// Provide custom embeddings.
    /// In this case, you must manually update your embeddings when adding, updating, and removing documents to your index.
    UserProvided(UserProvidedEmbedderSettings),
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HuggingFaceEmbedderSettings {
    /// the BERT embedding model you want to use from HuggingFace
    /// Defaults to `BAAI/bge-base-en-v1.5`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    /// if present, document_template must be a [Liquid template](https://shopify.github.io/liquid/).
    /// Use `{{ doc.attribute }}` to access document field values.
    /// Meilisearch also exposes a `{{ fields }}` array containing one object per document field, which you may access with `{{ field.name }}` and `{{ field.value }}`.
    ///
    /// For best results, use short strings indicating the type of document in that index, only include highly relevant document fields, and truncate long fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_template: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OpenapiEmbedderSettings {
    /// API key used to authorize against OpenAI.
    /// [Generate an API key](https://platform.openai.com/api-keys) from your OpenAI account.
    /// Use [tier 2 keys](https://platform.openai.com/docs/guides/rate-limits/usage-tiers?context=tier-two) or above for optimal performance.
    pub api_key: String,
    /// The openapi model name
    /// Default: `text-embedding-ada-002`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Defaults to the default for said model name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<usize>,
    /// if present, document_template must be a [Liquid template](https://shopify.github.io/liquid/).
    /// Use `{{ doc.attribute }}` to access document field values.
    /// Meilisearch also exposes a `{{ fields }}` array containing one object per document field, which you may access with `{{ field.name }}` and `{{ field.value }}`.
    ///
    /// For best results, use short strings indicating the type of document in that index, only include highly relevant document fields, and truncate long fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_template: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Eq, PartialEq, Copy)]
pub struct UserProvidedEmbedderSettings {
    /// dimensions of your custom embedding
    pub dimensions: usize,
}

/// Struct reprensenting a set of settings.
///
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
    /// List of associated words treated similarly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<HashMap<String, Vec<String>>>,
    /// List of words ignored by Meilisearch when present in search queries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_words: Option<Vec<String>>,
    /// List of [ranking rules](https://www.meilisearch.com/docs/learn/core_concepts/relevancy#order-of-the-rules) sorted by order of importance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranking_rules: Option<Vec<String>>,
    /// Attributes to use for [filtering](https://www.meilisearch.com/docs/learn/advanced/filtering).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filterable_attributes: Option<Vec<String>>,
    /// Attributes to sort.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sortable_attributes: Option<Vec<String>>,
    /// Search returns documents with distinct (different) values of the given field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distinct_attribute: Option<String>,
    /// Fields in which to search for matching query words sorted by order of importance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchable_attributes: Option<Vec<String>>,
    /// Fields displayed in the returned documents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub displayed_attributes: Option<Vec<String>>,
    /// Pagination settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationSetting>,
    /// Faceting settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faceting: Option<FacetingSettings>,
    /// TypoTolerance settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typo_tolerance: Option<TypoToleranceSettings>,
    /// Dictionary settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dictionary: Option<Vec<String>>,
    /// Proximity precision settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_precision: Option<String>,
    /// Settings how the embeddings for the experimental vector search feature are generated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedders: Option<HashMap<String, Embedder>>,
}

#[allow(missing_docs)]
impl Settings {
    /// Create undefined settings.
    #[must_use]
    pub fn new() -> Settings {
        Self::default()
    }

    #[must_use]
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

    #[must_use]
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

    #[must_use]
    pub fn with_pagination(self, pagination_settings: PaginationSetting) -> Settings {
        Settings {
            pagination: Some(pagination_settings),
            ..self
        }
    }

    #[must_use]
    pub fn with_typo_tolerance(self, typo_tolerance_settings: TypoToleranceSettings) -> Settings {
        Settings {
            typo_tolerance: Some(typo_tolerance_settings),
            ..self
        }
    }

    #[must_use]
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

    #[must_use]
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

    #[must_use]
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

    #[must_use]
    pub fn with_distinct_attribute(self, distinct_attribute: impl AsRef<str>) -> Settings {
        Settings {
            distinct_attribute: Some(distinct_attribute.as_ref().to_string()),
            ..self
        }
    }

    #[must_use]
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

    #[must_use]
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

    #[must_use]
    pub fn with_faceting(self, faceting: &FacetingSettings) -> Settings {
        Settings {
            faceting: Some(*faceting),
            ..self
        }
    }

    #[must_use]
    pub fn with_dictionary(
        self,
        dictionary: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Settings {
        Settings {
            dictionary: Some(
                dictionary
                    .into_iter()
                    .map(|v| v.as_ref().to_string())
                    .collect(),
            ),
            ..self
        }
    }

    pub fn with_proximity_precision(self, proximity_precision: impl AsRef<str>) -> Settings {
        Settings {
            proximity_precision: Some(proximity_precision.as_ref().to_string()),
            ..self
        }
    }

    /// EXPERIMENTAL
    /// Set the [embedders](https://www.meilisearch.com/docs/learn/experimental/vector_search) of the [Index].
    #[must_use]
    pub fn with_embedders<S>(self, embedders: HashMap<S, Embedder>) -> Settings
    where
        S: AsRef<str>,
    {
        Settings {
            embedders: Some(
                embedders
                    .into_iter()
                    .map(|(key, value)| (key.as_ref().to_string(), value))
                    .collect(),
            ),
            ..self
        }
    }
}

impl<Http: HttpClient> Index<Http> {
    /// Get [Settings] of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("get_settings", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let index = client.index("get_settings");
    ///
    /// let settings = index.get_settings().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_settings(&self) -> Result<Settings, Error> {
        self.client
            .http_client
            .request::<(), (), Settings>(
                &format!("{}/indexes/{}/settings", self.client.host, self.uid),
                Method::Get { query: () },
                200,
            )
            .await
    }

    /// Get [synonyms](https://www.meilisearch.com/docs/reference/api/settings#get-synonyms) of the [Index].
    ///
    /// # Example
    ///
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("get_synonyms", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let index = client.index("get_synonyms");
    ///
    /// let synonyms = index.get_synonyms().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_synonyms(&self) -> Result<HashMap<String, Vec<String>>, Error> {
        self.client
            .http_client
            .request::<(), (), HashMap<String, Vec<String>>>(
                &format!(
                    "{}/indexes/{}/settings/synonyms",
                    self.client.host, self.uid
                ),
                Method::Get { query: () },
                200,
            )
            .await
    }

    /// Get [pagination](https://www.meilisearch.com/docs/reference/api/settings#pagination) of the [Index].
    ///
    /// # Example
    ///
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("get_pagination", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let index = client.index("get_pagination");
    ///
    /// let pagination = index.get_pagination().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_pagination(&self) -> Result<PaginationSetting, Error> {
        self.client
            .http_client
            .request::<(), (), PaginationSetting>(
                &format!(
                    "{}/indexes/{}/settings/pagination",
                    self.client.host, self.uid
                ),
                Method::Get { query: () },
                200,
            )
            .await
    }

    /// Get [stop-words](https://www.meilisearch.com/docs/reference/api/settings#stop-words) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("get_stop_words", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let index = client.index("get_stop_words");
    ///
    /// let stop_words = index.get_stop_words().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_stop_words(&self) -> Result<Vec<String>, Error> {
        self.client
            .http_client
            .request::<(), (), Vec<String>>(
                &format!(
                    "{}/indexes/{}/settings/stop-words",
                    self.client.host, self.uid
                ),
                Method::Get { query: () },
                200,
            )
            .await
    }

    /// Get [ranking rules](https://www.meilisearch.com/docs/reference/api/settings#ranking-rules) of the [Index].
    ///
    /// # Example
    ///
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("get_ranking_rules", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let index = client.index("get_ranking_rules");
    ///
    /// let ranking_rules = index.get_ranking_rules().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_ranking_rules(&self) -> Result<Vec<String>, Error> {
        self.client
            .http_client
            .request::<(), (), Vec<String>>(
                &format!(
                    "{}/indexes/{}/settings/ranking-rules",
                    self.client.host, self.uid
                ),
                Method::Get { query: () },
                200,
            )
            .await
    }

    /// Get [filterable attributes](https://www.meilisearch.com/docs/reference/api/settings#filterable-attributes) of the [Index].
    ///
    /// # Example
    ///
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("get_filterable_attributes", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let index = client.index("get_filterable_attributes");
    ///
    /// let filterable_attributes = index.get_filterable_attributes().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_filterable_attributes(&self) -> Result<Vec<String>, Error> {
        self.client
            .http_client
            .request::<(), (), Vec<String>>(
                &format!(
                    "{}/indexes/{}/settings/filterable-attributes",
                    self.client.host, self.uid
                ),
                Method::Get { query: () },
                200,
            )
            .await
    }

    /// Get [sortable attributes](https://www.meilisearch.com/docs/reference/api/settings#sortable-attributes) of the [Index].
    ///
    /// # Example
    ///
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("get_sortable_attributes", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let index = client.index("get_sortable_attributes");
    ///
    /// let sortable_attributes = index.get_sortable_attributes().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_sortable_attributes(&self) -> Result<Vec<String>, Error> {
        self.client
            .http_client
            .request::<(), (), Vec<String>>(
                &format!(
                    "{}/indexes/{}/settings/sortable-attributes",
                    self.client.host, self.uid
                ),
                Method::Get { query: () },
                200,
            )
            .await
    }

    /// Get the [distinct attribute](https://www.meilisearch.com/docs/reference/api/settings#distinct-attribute) of the [Index].
    ///
    /// # Example
    ///
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("get_distinct_attribute", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let index = client.index("get_distinct_attribute");
    ///
    /// let distinct_attribute = index.get_distinct_attribute().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_distinct_attribute(&self) -> Result<Option<String>, Error> {
        self.client
            .http_client
            .request::<(), (), Option<String>>(
                &format!(
                    "{}/indexes/{}/settings/distinct-attribute",
                    self.client.host, self.uid
                ),
                Method::Get { query: () },
                200,
            )
            .await
    }

    /// Get [searchable attributes](https://www.meilisearch.com/docs/reference/api/settings#searchable-attributes) of the [Index].
    ///
    /// # Example
    ///
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("get_searchable_attributes", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let index = client.index("get_searchable_attributes");
    ///
    /// let searchable_attributes = index.get_searchable_attributes().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_searchable_attributes(&self) -> Result<Vec<String>, Error> {
        self.client
            .http_client
            .request::<(), (), Vec<String>>(
                &format!(
                    "{}/indexes/{}/settings/searchable-attributes",
                    self.client.host, self.uid
                ),
                Method::Get { query: () },
                200,
            )
            .await
    }

    /// Get [displayed attributes](https://www.meilisearch.com/docs/reference/api/settings#displayed-attributes) of the [Index].
    ///
    /// # Example
    ///
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("get_displayed_attributes", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let index = client.index("get_displayed_attributes");
    ///
    /// let displayed_attributes = index.get_displayed_attributes().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_displayed_attributes(&self) -> Result<Vec<String>, Error> {
        self.client
            .http_client
            .request::<(), (), Vec<String>>(
                &format!(
                    "{}/indexes/{}/settings/displayed-attributes",
                    self.client.host, self.uid
                ),
                Method::Get { query: () },
                200,
            )
            .await
    }

    /// Get [faceting](https://www.meilisearch.com/docs/reference/api/settings#faceting) settings of the [Index].
    ///
    /// # Example
    ///
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("get_faceting", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let index = client.index("get_faceting");
    ///
    /// let faceting = index.get_faceting().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_faceting(&self) -> Result<FacetingSettings, Error> {
        self.client
            .http_client
            .request::<(), (), FacetingSettings>(
                &format!(
                    "{}/indexes/{}/settings/faceting",
                    self.client.host, self.uid
                ),
                Method::Get { query: () },
                200,
            )
            .await
    }

    /// Get [dictionary](https://www.meilisearch.com/docs/reference/api/settings#dictionary) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("get_dictionary", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let index = client.index("get_dictionary");
    ///
    /// let dictionary = index.get_dictionary().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_dictionary(&self) -> Result<Vec<String>, Error> {
        self.client
            .http_client
            .request::<(), (), Vec<String>>(
                &format!(
                    "{}/indexes/{}/settings/dictionary",
                    self.client.host, self.uid
                ),
                Method::Get { query: () },
                200,
            )
            .await
    }

    /// Get [proximity_precision](https://www.meilisearch.com/docs/reference/api/settings#proximity-precision) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("get_proximity_precision", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let index = client.index("get_proximity_precision");
    ///
    /// let proximity_precision = index.get_proximity_precision().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_proximity_precision(&self) -> Result<String, Error> {
        self.client
            .http_client
            .request::<(), (), String>(
                &format!(
                    "{}/indexes/{}/settings/proximity-precision",
                    self.client.host, self.uid
                ),
                Method::Get { query: () },
                200,
            )
            .await
    }

    /// Get [typo tolerance](https://www.meilisearch.com/docs/learn/configuration/typo_tolerance#typo-tolerance) of the [Index].
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("get_typo_tolerance", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let index = client.index("get_typo_tolerance");
    ///
    /// let typo_tolerance = index.get_typo_tolerance().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_typo_tolerance(&self) -> Result<TypoToleranceSettings, Error> {
        self.client
            .http_client
            .request::<(), (), TypoToleranceSettings>(
                &format!(
                    "{}/indexes/{}/settings/typo-tolerance",
                    self.client.host, self.uid
                ),
                Method::Get { query: () },
                200,
            )
            .await
    }

    /// EXPERIMENTAL
    /// Get [embedders](https://www.meilisearch.com/docs/learn/experimental/vector_search) of the [Index].
    ///
    /// ```
    /// # use std::collections::HashMap;
    /// # use std::string::String;
    /// # use meilisearch_sdk::{indexes::*,features::ExperimentalFeatures,settings::Embedder,settings::UserProvidedEmbedderSettings,settings::Settings,client::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("get_embedders", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let index = client.index("get_embedders");
    ///
    /// # let mut features = ExperimentalFeatures::new(&client);
    /// # features.set_vector_store(true);
    /// # let res = features.update().await.unwrap();
    /// #
    /// # let t=index.set_settings(&Settings{
    /// #     embedders:Some(HashMap::from([(String::from("default"),Embedder::UserProvided(UserProvidedEmbedderSettings{dimensions:1}))])),
    /// #     ..Settings::default()
    /// # }).await.unwrap();
    /// # t.wait_for_completion(&client, None, None).await.unwrap();
    /// let embedders = index.get_embedders().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn get_embedders(&self) -> Result<HashMap<String, Embedder>, Error> {
        self.client
            .http_client
            .request::<(), (), Option<HashMap<String, Embedder>>>(
                &format!(
                    "{}/indexes/{}/settings/embedders",
                    self.client.host, self.uid
                ),
                Method::Get { query: () },
                200,
            )
            .await
            .map(|r| r.unwrap_or_default())
    }

    /// Update [settings](../settings/struct.Settings) of the [Index].
    ///
    /// Updates in the settings are partial. This means that any parameters corresponding to a `None` value will be left unchanged.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("set_settings", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("set_settings");
    ///
    /// let stop_words = vec![String::from("a"), String::from("the"), String::from("of")];
    /// let settings = Settings::new()
    ///     .with_stop_words(stop_words.clone())
    ///     .with_pagination(PaginationSetting {max_total_hits: 100}
    /// );
    ///
    /// let task = index.set_settings(&settings).await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn set_settings(&self, settings: &Settings) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), &Settings, TaskInfo>(
                &format!("{}/indexes/{}/settings", self.client.host, self.uid),
                Method::Patch {
                    query: (),
                    body: settings,
                },
                202,
            )
            .await
    }

    /// Update [synonyms](https://www.meilisearch.com/docs/reference/api/settings#synonyms) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
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
    ) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), &HashMap<String, Vec<String>>, TaskInfo>(
                &format!(
                    "{}/indexes/{}/settings/synonyms",
                    self.client.host, self.uid
                ),
                Method::Put {
                    query: (),
                    body: synonyms,
                },
                202,
            )
            .await
    }

    /// Update [pagination](https://www.meilisearch.com/docs/reference/api/settings#pagination) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::*};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("set_pagination", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("set_pagination");
    ///
    /// let pagination = PaginationSetting {max_total_hits:100};
    /// let task = index.set_pagination(pagination).await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn set_pagination(&self, pagination: PaginationSetting) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), &PaginationSetting, TaskInfo>(
                &format!(
                    "{}/indexes/{}/settings/pagination",
                    self.client.host, self.uid
                ),
                Method::Patch {
                    query: (),
                    body: &pagination,
                },
                202,
            )
            .await
    }

    /// Update [stop-words](https://www.meilisearch.com/docs/reference/api/settings#stop-words) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
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
    ) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), Vec<String>, TaskInfo>(
                &format!(
                    "{}/indexes/{}/settings/stop-words",
                    self.client.host, self.uid
                ),
                Method::Put {
                    query: (),
                    body: stop_words
                        .into_iter()
                        .map(|v| v.as_ref().to_string())
                        .collect(),
                },
                202,
            )
            .await
    }

    /// Update [ranking rules](https://www.meilisearch.com/docs/reference/api/settings#ranking-rules) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
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
    ) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), Vec<String>, TaskInfo>(
                &format!(
                    "{}/indexes/{}/settings/ranking-rules",
                    self.client.host, self.uid
                ),
                Method::Put {
                    query: (),
                    body: ranking_rules
                        .into_iter()
                        .map(|v| v.as_ref().to_string())
                        .collect(),
                },
                202,
            )
            .await
    }

    /// Update [filterable attributes](https://www.meilisearch.com/docs/reference/api/settings#filterable-attributes) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
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
    ) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), Vec<String>, TaskInfo>(
                &format!(
                    "{}/indexes/{}/settings/filterable-attributes",
                    self.client.host, self.uid
                ),
                Method::Put {
                    query: (),
                    body: filterable_attributes
                        .into_iter()
                        .map(|v| v.as_ref().to_string())
                        .collect(),
                },
                202,
            )
            .await
    }

    /// Update [sortable attributes](https://www.meilisearch.com/docs/reference/api/settings#sortable-attributes) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
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
    ) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), Vec<String>, TaskInfo>(
                &format!(
                    "{}/indexes/{}/settings/sortable-attributes",
                    self.client.host, self.uid
                ),
                Method::Put {
                    query: (),
                    body: sortable_attributes
                        .into_iter()
                        .map(|v| v.as_ref().to_string())
                        .collect(),
                },
                202,
            )
            .await
    }

    /// Update the [distinct attribute](https://www.meilisearch.com/docs/reference/api/settings#distinct-attribute) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
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
    ) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), String, TaskInfo>(
                &format!(
                    "{}/indexes/{}/settings/distinct-attribute",
                    self.client.host, self.uid
                ),
                Method::Put {
                    query: (),
                    body: distinct_attribute.as_ref().to_string(),
                },
                202,
            )
            .await
    }

    /// Update [searchable attributes](https://www.meilisearch.com/docs/reference/api/settings#searchable-attributes) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
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
    ) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), Vec<String>, TaskInfo>(
                &format!(
                    "{}/indexes/{}/settings/searchable-attributes",
                    self.client.host, self.uid
                ),
                Method::Put {
                    query: (),
                    body: searchable_attributes
                        .into_iter()
                        .map(|v| v.as_ref().to_string())
                        .collect(),
                },
                202,
            )
            .await
    }

    /// Update [displayed attributes](https://www.meilisearch.com/docs/reference/api/settings#displayed-attributes) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
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
    ) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), Vec<String>, TaskInfo>(
                &format!(
                    "{}/indexes/{}/settings/displayed-attributes",
                    self.client.host, self.uid
                ),
                Method::Put {
                    query: (),
                    body: displayed_attributes
                        .into_iter()
                        .map(|v| v.as_ref().to_string())
                        .collect(),
                },
                202,
            )
            .await
    }

    /// Update [faceting](https://www.meilisearch.com/docs/reference/api/settings#faceting) settings of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings, settings::FacetingSettings};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("set_faceting", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("set_faceting");
    ///
    /// let mut faceting = FacetingSettings {
    ///     max_values_per_facet: 12,
    /// };
    ///
    /// let task = index.set_faceting(&faceting).await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn set_faceting(&self, faceting: &FacetingSettings) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), &FacetingSettings, TaskInfo>(
                &format!(
                    "{}/indexes/{}/settings/faceting",
                    self.client.host, self.uid
                ),
                Method::Patch {
                    query: (),
                    body: faceting,
                },
                202,
            )
            .await
    }

    /// Update [dictionary](https://www.meilisearch.com/docs/reference/api/settings#dictionary) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("set_dictionary", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("set_dictionary");
    ///
    /// let task = index.set_dictionary(["J. K.", "J. R. R."]).await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn set_dictionary(
        &self,
        dictionary: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), Vec<String>, TaskInfo>(
                &format!(
                    "{}/indexes/{}/settings/dictionary",
                    self.client.host, self.uid
                ),
                Method::Put {
                    query: (),
                    body: dictionary
                        .into_iter()
                        .map(|v| v.as_ref().to_string())
                        .collect(),
                },
                202,
            )
            .await
    }

    /// Update [typo tolerance](https://www.meilisearch.com/docs/learn/configuration/typo_tolerance#typo-tolerance) settings of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings, settings::{TypoToleranceSettings, MinWordSizeForTypos}};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("set_typo_tolerance", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("set_typo_tolerance");
    ///
    /// let typo_tolerance = TypoToleranceSettings{
    ///     enabled: Some(true),
    ///     disable_on_attributes: Some(vec!["title".to_string()]),
    ///     disable_on_words: Some(vec![]),
    ///     min_word_size_for_typos: Some(MinWordSizeForTypos::default()),
    /// };
    ///
    /// let task = index.set_typo_tolerance(&typo_tolerance).await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn set_typo_tolerance(
        &self,
        typo_tolerance: &TypoToleranceSettings,
    ) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), &TypoToleranceSettings, TaskInfo>(
                &format!(
                    "{}/indexes/{}/settings/typo-tolerance",
                    self.client.host, self.uid
                ),
                Method::Patch {
                    query: (),
                    body: typo_tolerance,
                },
                202,
            )
            .await
    }

    /// Update [proximity-precision](https://www.meilisearch.com/docs/learn/configuration/proximity-precision) settings of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("set_proximity_precision", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("set_proximity_precision");
    ///
    /// let task = index.set_proximity_precision("byWord".to_string()).await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn set_proximity_precision(
        &self,
        proximity_precision: String,
    ) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), String, TaskInfo>(
                &format!(
                    "{}/indexes/{}/settings/proximity-precision",
                    self.client.host, self.uid
                ),
                Method::Put {
                    query: (),
                    body: proximity_precision,
                },
                202,
            )
            .await
    }

    /// Reset [Settings] of the [Index].
    ///
    /// All settings will be reset to their [default value](https://www.meilisearch.com/docs/reference/api/settings#reset-settings).
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("reset_settings", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("reset_settings");
    ///
    /// let task = index.reset_settings().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn reset_settings(&self) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), (), TaskInfo>(
                &format!("{}/indexes/{}/settings", self.client.host, self.uid),
                Method::Delete { query: () },
                202,
            )
            .await
    }

    /// Reset [synonyms](https://www.meilisearch.com/docs/reference/api/settings#synonyms) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("reset_synonyms", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("reset_synonyms");
    ///
    /// let task = index.reset_synonyms().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn reset_synonyms(&self) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), (), TaskInfo>(
                &format!(
                    "{}/indexes/{}/settings/synonyms",
                    self.client.host, self.uid
                ),
                Method::Delete { query: () },
                202,
            )
            .await
    }

    /// Reset [pagination](https://www.meilisearch.com/docs/learn/configuration/settings#pagination) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("reset_pagination", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("reset_pagination");
    ///
    /// let task = index.reset_pagination().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn reset_pagination(&self) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), (), TaskInfo>(
                &format!(
                    "{}/indexes/{}/settings/pagination",
                    self.client.host, self.uid
                ),
                Method::Delete { query: () },
                202,
            )
            .await
    }
    /// Reset [stop-words](https://www.meilisearch.com/docs/reference/api/settings#stop-words) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("reset_stop_words", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("reset_stop_words");
    ///
    /// let task = index.reset_stop_words().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn reset_stop_words(&self) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), (), TaskInfo>(
                &format!(
                    "{}/indexes/{}/settings/stop-words",
                    self.client.host, self.uid
                ),
                Method::Delete { query: () },
                202,
            )
            .await
    }

    /// Reset [ranking rules](https://www.meilisearch.com/docs/learn/core_concepts/relevancy#ranking-rules) of the [Index] to default value.
    ///
    /// **Default value: `["words", "typo", "proximity", "attribute", "sort", "exactness"]`.**
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("reset_ranking_rules", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("reset_ranking_rules");
    ///
    /// let task = index.reset_ranking_rules().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn reset_ranking_rules(&self) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), (), TaskInfo>(
                &format!(
                    "{}/indexes/{}/settings/ranking-rules",
                    self.client.host, self.uid
                ),
                Method::Delete { query: () },
                202,
            )
            .await
    }

    /// Reset [filterable attributes](https://www.meilisearch.com/docs/reference/api/settings#filterable-attributes) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("reset_filterable_attributes", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("reset_filterable_attributes");
    ///
    /// let task = index.reset_filterable_attributes().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn reset_filterable_attributes(&self) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), (), TaskInfo>(
                &format!(
                    "{}/indexes/{}/settings/filterable-attributes",
                    self.client.host, self.uid
                ),
                Method::Delete { query: () },
                202,
            )
            .await
    }

    /// Reset [sortable attributes](https://www.meilisearch.com/docs/reference/api/settings#sortable-attributes) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("reset_sortable_attributes", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("reset_sortable_attributes");
    ///
    /// let task = index.reset_sortable_attributes().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn reset_sortable_attributes(&self) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), (), TaskInfo>(
                &format!(
                    "{}/indexes/{}/settings/sortable-attributes",
                    self.client.host, self.uid
                ),
                Method::Delete { query: () },
                202,
            )
            .await
    }

    /// Reset the [distinct attribute](https://www.meilisearch.com/docs/reference/api/settings#distinct-attribute) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("reset_distinct_attribute", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("reset_distinct_attribute");
    ///
    /// let task = index.reset_distinct_attribute().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn reset_distinct_attribute(&self) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), (), TaskInfo>(
                &format!(
                    "{}/indexes/{}/settings/distinct-attribute",
                    self.client.host, self.uid
                ),
                Method::Delete { query: () },
                202,
            )
            .await
    }

    /// Reset [searchable attributes](https://www.meilisearch.com/docs/learn/configuration/displayed_searchable_attributes#searchable-fields) of
    /// the [Index] (enable all attributes).
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("reset_searchable_attributes", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("reset_searchable_attributes");
    ///
    /// let task = index.reset_searchable_attributes().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn reset_searchable_attributes(&self) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), (), TaskInfo>(
                &format!(
                    "{}/indexes/{}/settings/searchable-attributes",
                    self.client.host, self.uid
                ),
                Method::Delete { query: () },
                202,
            )
            .await
    }

    /// Reset [displayed attributes](https://www.meilisearch.com/docs/reference/api/settings#displayed-attributes) of the [Index] (enable all attributes).
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("reset_displayed_attributes", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("reset_displayed_attributes");
    ///
    /// let task = index.reset_displayed_attributes().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn reset_displayed_attributes(&self) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), (), TaskInfo>(
                &format!(
                    "{}/indexes/{}/settings/displayed-attributes",
                    self.client.host, self.uid
                ),
                Method::Delete { query: () },
                202,
            )
            .await
    }

    /// Reset [faceting](https://www.meilisearch.com/docs/reference/api/settings#faceting) settings of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("reset_faceting", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("reset_faceting");
    ///
    /// let task = index.reset_faceting().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn reset_faceting(&self) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), (), TaskInfo>(
                &format!(
                    "{}/indexes/{}/settings/faceting",
                    self.client.host, self.uid
                ),
                Method::Delete { query: () },
                202,
            )
            .await
    }

    /// Reset [dictionary](https://www.meilisearch.com/docs/reference/api/settings#dictionary) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("reset_dictionary", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("reset_dictionary");
    ///
    /// let task = index.reset_dictionary().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn reset_dictionary(&self) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), (), TaskInfo>(
                &format!(
                    "{}/indexes/{}/settings/dictionary",
                    self.client.host, self.uid
                ),
                Method::Delete { query: () },
                202,
            )
            .await
    }

    /// Reset [typo tolerance](https://www.meilisearch.com/docs/learn/configuration/typo_tolerance#typo-tolerance) settings of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("reset_typo_tolerance", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("reset_typo_tolerance");
    ///
    /// let task = index.reset_typo_tolerance().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn reset_typo_tolerance(&self) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), (), TaskInfo>(
                &format!(
                    "{}/indexes/{}/settings/typo-tolerance",
                    self.client.host, self.uid
                ),
                Method::Delete { query: () },
                202,
            )
            .await
    }

    /// Reset [proximity precision](https://www.meilisearch.com/docs/learn/configuration/typo_tolerance#typo-tolerance) settings of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("reset_proximity_precision", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("reset_proximity_precision");
    ///
    /// let task = index.reset_proximity_precision().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn reset_proximity_precision(&self) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), (), TaskInfo>(
                &format!(
                    "{}/indexes/{}/settings/proximity-precision",
                    self.client.host, self.uid
                ),
                Method::Delete { query: () },
                202,
            )
            .await
    }

    /// EXPERIMENTAL
    /// Reset [embedders](https://www.meilisearch.com/docs/learn/experimental/vector_search) of the [Index].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, settings::Settings};
    /// #
    /// # let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    /// # let MEILISEARCH_API_KEY = option_env!("MEILISEARCH_API_KEY").unwrap_or("masterKey");
    /// #
    /// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
    /// # let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();
    /// # client.create_index("reset_embedders", None).await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// let mut index = client.index("reset_embedders");
    ///
    /// let task = index.reset_embedders().await.unwrap();
    /// # index.delete().await.unwrap().wait_for_completion(&client, None, None).await.unwrap();
    /// # });
    /// ```
    pub async fn reset_embedders(&self) -> Result<TaskInfo, Error> {
        self.client
            .http_client
            .request::<(), (), TaskInfo>(
                &format!(
                    "{}/indexes/{}/settings/embedders",
                    self.client.host, self.uid
                ),
                Method::Delete { query: () },
                202,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::client::*;
    use meilisearch_test_macro::meilisearch_test;

    #[meilisearch_test]
    async fn test_set_faceting_settings(client: Client, index: Index) {
        let faceting = FacetingSettings {
            max_values_per_facet: 5,
        };
        let settings = Settings::new().with_faceting(&faceting);

        let task_info = index.set_settings(&settings).await.unwrap();
        client.wait_for_task(task_info, None, None).await.unwrap();

        let res = index.get_faceting().await.unwrap();

        assert_eq!(faceting, res);
    }

    #[meilisearch_test]
    async fn test_get_faceting(index: Index) {
        let faceting = FacetingSettings {
            max_values_per_facet: 100,
        };

        let res = index.get_faceting().await.unwrap();

        assert_eq!(faceting, res);
    }

    #[meilisearch_test]
    async fn test_get_embeddings(index: Index) {
        let res = index.get_embedders().await.unwrap();

        assert_eq!(HashMap::new(), res);
    }

    #[meilisearch_test]
    async fn test_set_faceting(client: Client, index: Index) {
        let faceting = FacetingSettings {
            max_values_per_facet: 5,
        };
        let task_info = index.set_faceting(&faceting).await.unwrap();
        client.wait_for_task(task_info, None, None).await.unwrap();

        let res = index.get_faceting().await.unwrap();

        assert_eq!(faceting, res);
    }

    #[meilisearch_test]
    async fn test_reset_faceting(client: Client, index: Index) {
        let task_info = index.reset_faceting().await.unwrap();
        client.wait_for_task(task_info, None, None).await.unwrap();
        let faceting = FacetingSettings {
            max_values_per_facet: 100,
        };

        let res = index.get_faceting().await.unwrap();

        assert_eq!(faceting, res);
    }

    #[meilisearch_test]
    async fn test_reset_embedders(client: Client, index: Index) {
        let features = crate::features::ExperimentalFeatures::new(&client)
            .set_vector_store(true)
            .update()
            .await
            .expect("could not enable the vector store");
        assert_eq!(features.vector_store, true);
        let task_info = index.reset_embedders().await.unwrap();
        client.wait_for_task(task_info, None, None).await.unwrap();

        let res = index.get_embedders().await.unwrap();

        assert_eq!(HashMap::new(), res);
    }

    #[meilisearch_test]
    async fn test_get_dictionary(index: Index) {
        let dictionary: Vec<String> = vec![];

        let res = index.get_dictionary().await.unwrap();

        assert_eq!(dictionary, res);
    }

    #[meilisearch_test]
    async fn test_set_dictionary(client: Client, index: Index) {
        let dictionary: Vec<&str> = vec!["J. K.", "J. R. R."];
        let task_info = index.set_dictionary(&dictionary).await.unwrap();
        client.wait_for_task(task_info, None, None).await.unwrap();

        let res = index.get_dictionary().await.unwrap();

        assert_eq!(dictionary, res);
    }

    #[meilisearch_test]
    async fn test_set_empty_dictionary(client: Client, index: Index) {
        let dictionary: Vec<&str> = vec![];
        let task_info = index.set_dictionary(&dictionary).await.unwrap();
        client.wait_for_task(task_info, None, None).await.unwrap();

        let res = index.get_dictionary().await.unwrap();

        assert_eq!(dictionary, res);
    }

    #[meilisearch_test]
    async fn test_reset_dictionary(client: Client, index: Index) {
        let dictionary: Vec<&str> = vec![];
        let task_info = index.reset_dictionary().await.unwrap();
        client.wait_for_task(task_info, None, None).await.unwrap();

        let res = index.get_dictionary().await.unwrap();

        assert_eq!(dictionary, res);
    }

    #[meilisearch_test]
    async fn test_get_pagination(index: Index) {
        let pagination = PaginationSetting {
            max_total_hits: 1000,
        };

        let res = index.get_pagination().await.unwrap();

        assert_eq!(pagination, res);
    }

    #[meilisearch_test]
    async fn test_set_pagination(index: Index) {
        let pagination = PaginationSetting { max_total_hits: 11 };
        let task = index.set_pagination(pagination).await.unwrap();
        index.wait_for_task(task, None, None).await.unwrap();

        let res = index.get_pagination().await.unwrap();

        assert_eq!(pagination, res);
    }

    #[meilisearch_test]
    async fn test_reset_pagination(index: Index) {
        let pagination = PaginationSetting { max_total_hits: 10 };
        let default = PaginationSetting {
            max_total_hits: 1000,
        };

        let task = index.set_pagination(pagination).await.unwrap();
        index.wait_for_task(task, None, None).await.unwrap();

        let reset_task = index.reset_pagination().await.unwrap();
        index.wait_for_task(reset_task, None, None).await.unwrap();

        let res = index.get_pagination().await.unwrap();

        assert_eq!(default, res);
    }

    #[meilisearch_test]
    async fn test_get_typo_tolerance(index: Index) {
        let expected = TypoToleranceSettings {
            enabled: Some(true),
            disable_on_attributes: Some(vec![]),
            disable_on_words: Some(vec![]),
            min_word_size_for_typos: Some(MinWordSizeForTypos {
                one_typo: Some(5),
                two_typos: Some(9),
            }),
        };

        let res = index.get_typo_tolerance().await.unwrap();

        assert_eq!(expected, res);
    }

    #[meilisearch_test]
    async fn test_set_typo_tolerance(client: Client, index: Index) {
        let expected = TypoToleranceSettings {
            enabled: Some(true),
            disable_on_attributes: Some(vec!["title".to_string()]),
            disable_on_words: Some(vec![]),
            min_word_size_for_typos: Some(MinWordSizeForTypos {
                one_typo: Some(5),
                two_typos: Some(9),
            }),
        };

        let typo_tolerance = TypoToleranceSettings {
            disable_on_attributes: Some(vec!["title".to_string()]),
            ..Default::default()
        };

        let task_info = index.set_typo_tolerance(&typo_tolerance).await.unwrap();
        client.wait_for_task(task_info, None, None).await.unwrap();

        let res = index.get_typo_tolerance().await.unwrap();

        assert_eq!(expected, res);
    }

    #[meilisearch_test]
    async fn test_reset_typo_tolerance(index: Index) {
        let expected = TypoToleranceSettings {
            enabled: Some(true),
            disable_on_attributes: Some(vec![]),
            disable_on_words: Some(vec![]),
            min_word_size_for_typos: Some(MinWordSizeForTypos {
                one_typo: Some(5),
                two_typos: Some(9),
            }),
        };

        let typo_tolerance = TypoToleranceSettings {
            disable_on_attributes: Some(vec!["title".to_string()]),
            ..Default::default()
        };

        let task = index.set_typo_tolerance(&typo_tolerance).await.unwrap();
        index.wait_for_task(task, None, None).await.unwrap();

        let reset_task = index.reset_typo_tolerance().await.unwrap();
        index.wait_for_task(reset_task, None, None).await.unwrap();

        let default = index.get_typo_tolerance().await.unwrap();

        assert_eq!(expected, default);
    }

    #[meilisearch_test]
    async fn test_get_proximity_precision(index: Index) {
        let expected = "byWord".to_string();

        let res = index.get_proximity_precision().await.unwrap();

        assert_eq!(expected, res);
    }

    #[meilisearch_test]
    async fn test_set_proximity_precision(client: Client, index: Index) {
        let expected = "byAttribute".to_string();

        let task_info = index
            .set_proximity_precision("byAttribute".to_string())
            .await
            .unwrap();
        client.wait_for_task(task_info, None, None).await.unwrap();

        let res = index.get_proximity_precision().await.unwrap();

        assert_eq!(expected, res);
    }

    #[meilisearch_test]
    async fn test_set_embedding_settings(client: Client, index: Index) {
        let features = crate::features::ExperimentalFeatures::new(&client)
            .set_vector_store(true)
            .update()
            .await
            .expect("could not enable the vector store");
        assert_eq!(features.vector_store, true);

        let custom_embedder =
            Embedder::UserProvided(UserProvidedEmbedderSettings { dimensions: 2 });
        let embeddings = HashMap::from([("default".into(), custom_embedder)]);
        let settings = Settings::new().with_embedders(embeddings.clone());

        let task_info = index.set_settings(&settings).await.unwrap();
        client.wait_for_task(task_info, None, None).await.unwrap();

        let res = index.get_embedders().await.unwrap();

        assert_eq!(embeddings, res);
    }

    #[meilisearch_test]
    async fn test_reset_proximity_precision(index: Index) {
        let expected = "byWord".to_string();

        let task = index
            .set_proximity_precision("byAttribute".to_string())
            .await
            .unwrap();
        index.wait_for_task(task, None, None).await.unwrap();

        let reset_task = index.reset_proximity_precision().await.unwrap();
        index.wait_for_task(reset_task, None, None).await.unwrap();

        let default = index.get_proximity_precision().await.unwrap();

        assert_eq!(expected, default);
    }
}
