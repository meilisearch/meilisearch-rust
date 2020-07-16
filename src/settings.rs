use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{indexes::Index, errors::Error, request::{request, Method}, progress::{Progress, ProgressJson}};

/// Struct reprensenting a set of settings.  
/// You can build this struct using the builder syntax.  
///
/// # Example
///
/// ```
/// # use meilisearch_sdk::settings::Settings;
/// let stop_words = vec![String::from("a"), String::from("the"), String::from("of")];
///
/// let settings = Settings::new()
///     .with_stop_words(stop_words.clone())
///     .with_accept_new_fields(false);
///
/// // OR
///
/// let mut settings = Settings::new();
/// settings.stop_words = Some(stop_words.clone());
/// settings.accept_new_fields = Some(false);
///
/// // OR
///
/// let settings = Settings {
///     stop_words: Some(stop_words.clone()),
///     accept_new_fields: Some(false),
///     ..Settings::new()
/// };
/// ```
#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// List of associated words treated similarly
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<HashMap<String, Vec<String>>>,
    /// List of words ignored by MeiliSearch when present in search queries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_words: Option<Vec<String>>,
    /// List of [ranking rules](https://docs.meilisearch.com/guides/main_concepts/relevancy.html#order-of-the-rules) sorted by order of importance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranking_rules: Option<Vec<String>>,
    /// Attributes to use as [facets](https://docs.meilisearch.com/guides/advanced_guides/faceted_search.html)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_for_faceting: Option<Vec<String>>,
    /// Search returns documents with distinct (different) values of the given field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distinct_attribute: Option<String>,
    /// Fields in which to search for matching query words sorted by order of importance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchable_attributes: Option<Vec<String>>,
    /// Fields displayed in the returned documents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub displayed_attributes: Option<Vec<String>>,
    /// Defines whether new fields should be searchable and displayed or not
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_new_fields: Option<bool>,
}

#[allow(missing_docs)]
impl Settings {
    /// Create undefined settings
    pub fn new() -> Settings {
        Settings {
            synonyms: None,
            stop_words: None,
            ranking_rules: None,
            attributes_for_faceting: None,
            distinct_attribute: None,
            searchable_attributes: None,
            displayed_attributes: None,
            accept_new_fields: None,
        }
    }
    pub fn with_synonyms(self, synonyms: HashMap<String, Vec<String>>) -> Settings {
        Settings {
            synonyms: Some(synonyms),
            ..self
        }
    }
    pub fn with_stop_words(self, stop_words: Vec<String>) -> Settings {
        Settings {
            stop_words: Some(stop_words),
            ..self
        }
    }
    pub fn with_ranking_rules(self, ranking_rules: Vec<String>) -> Settings {
        Settings {
            ranking_rules: Some(ranking_rules),
            ..self
        }
    }
    pub fn with_attributes_for_faceting(self, attributes_for_faceting: Vec<String>) -> Settings {
        Settings {
            attributes_for_faceting: Some(attributes_for_faceting),
            ..self
        }
    }
    pub fn with_distinct_attribute(self, distinct_attribute: String) -> Settings {
        Settings {
            distinct_attribute: Some(distinct_attribute),
            ..self
        }
    }
    pub fn with_searchable_attributes(self, searchable_attributes: Vec<String>) -> Settings {
        Settings {
            searchable_attributes: Some(searchable_attributes),
            ..self
        }
    }
    pub fn with_displayed_attributes(self, displayed_attributes: Vec<String>) -> Settings {
        Settings {
            displayed_attributes: Some(displayed_attributes),
            ..self
        }
    }
    pub fn with_accept_new_fields(self, accept_new_fields: bool) -> Settings {
        Settings {
            accept_new_fields: Some(accept_new_fields),
            ..self
        }
    }
}

impl<'a> Index<'a> {
    /// Get the [settings](../settings/struct.Settings.html) of the Index.
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "");
    /// let movie_index = client.get_or_create("movies").await.unwrap();
    /// let settings = movie_index.get_settings().await.unwrap();
    /// # }
    /// ```
    pub async fn get_settings(&self) -> Result<Settings, Error> {
        Ok(request::<(), Settings>(
            &format!("{}/indexes/{}/settings", self.client.host, self.uid),
            self.client.apikey,
            Method::Get,
            200,
        ).await?)
    }

    /// Get the [synonyms](https://docs.meilisearch.com/guides/advanced_guides/synonyms.html) of the Index.
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "");
    /// let movie_index = client.get_or_create("movies").await.unwrap();
    /// let synonyms = movie_index.get_synonyms().await.unwrap();
    /// # }
    /// ```
    pub async fn get_synonyms(&self) -> Result<HashMap<String, Vec<String>>, Error> {
        Ok(request::<(), HashMap<String, Vec<String>>>(
            &format!("{}/indexes/{}/settings/synonyms", self.client.host, self.uid),
            self.client.apikey,
            Method::Get,
            200,
        ).await?)
    }

    /// Update the settings of the index.  
    /// Updates in the settings are partial. This means that any parameters corresponding to a None value will be left unchanged.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let stop_words = vec![String::from("a"), String::from("the"), String::from("of")];
    /// let settings = Settings::new()
    ///     .with_stop_words(stop_words.clone())
    ///     .with_accept_new_fields(false);
    ///
    /// let progress = movie_index.set_settings(&settings).await.unwrap();
    /// # }
    /// ```
    pub async fn set_settings(&'a self, settings: &Settings) -> Result<Progress<'a>, Error> {
        Ok(request::<&Settings, ProgressJson>(
            &format!("{}/indexes/{}/settings", self.client.host, self.uid),
            self.client.apikey,
            Method::Post(settings),
            202,
        ).await?
        .into_progress(self))
    }

    /// Update the synonyms of the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let mut synonyms = std::collections::HashMap::new();
    /// synonyms.insert(String::from("wolverine"), vec![String::from("xmen"), String::from("logan")]);
    /// synonyms.insert(String::from("logan"), vec![String::from("xmen"), String::from("wolverine")]);
    /// synonyms.insert(String::from("wow"), vec![String::from("world of warcraft")]);
    ///
    /// let progress = movie_index.set_synonyms(&synonyms).await.unwrap();
    /// # }
    /// ```
    pub async fn set_synonyms(&'a self, synonyms: &HashMap<String, Vec<String>>) -> Result<Progress<'a>, Error> {
        Ok(request::<&HashMap<String, Vec<String>>, ProgressJson>(
            &format!("{}/indexes/{}/settings/synonyms", self.client.host, self.uid),
            self.client.apikey,
            Method::Post(synonyms),
            202,
        ).await?
        .into_progress(self))
    }

    /// Reset the settings of the index.  
    /// All settings will be reset to their [default value](https://docs.meilisearch.com/references/settings.html#reset-settings).
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let progress = movie_index.reset_settings().await.unwrap();
    /// # }
    /// ```
    pub async fn reset_settings(&'a self) -> Result<Progress<'a>, Error> {
        Ok(request::<(), ProgressJson>(
            &format!("{}/indexes/{}/settings", self.client.host, self.uid),
            self.client.apikey,
            Method::Delete,
            202,
        ).await?
        .into_progress(self))
    }

    /// Reset the synonyms of the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let progress = movie_index.reset_synonyms().await.unwrap();
    /// # }
    /// ```
    pub async fn reset_synonyms(&'a self) -> Result<Progress<'a>, Error> {
        Ok(request::<(), ProgressJson>(
            &format!("{}/indexes/{}/settings/synonyms", self.client.host, self.uid),
            self.client.apikey,
            Method::Delete,
            202,
        ).await?
        .into_progress(self))
    }
}