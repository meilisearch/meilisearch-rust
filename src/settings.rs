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
///     .with_stop_words(stop_words.clone());
///
/// // OR
///
/// let mut settings = Settings::new();
/// settings.stop_words = Some(stop_words.clone());
///
/// // OR
///
/// let settings = Settings {
///     stop_words: Some(stop_words.clone()),
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
}

impl<'a> Index<'a> {
    /// Get [settings](../settings/struct.Settings.html) of the Index.
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "masterKey");
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

    /// Get [synonyms](https://docs.meilisearch.com/guides/advanced_guides/synonyms.html) of the Index.
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "masterKey");
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

    /// Get [stop-words](https://docs.meilisearch.com/guides/advanced_guides/stop_words.html) of the Index.
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let movie_index = client.get_or_create("movies").await.unwrap();
    /// let stop_words = movie_index.get_stop_words().await.unwrap();
    /// # }
    /// ```
    pub async fn get_stop_words(&self) -> Result<Vec<String>, Error> {
        Ok(request::<(), Vec<String>>(
            &format!("{}/indexes/{}/settings/stop-words", self.client.host, self.uid),
            self.client.apikey,
            Method::Get,
            200,
        ).await?)
    }

    /// Get [ranking rules](https://docs.meilisearch.com/guides/main_concepts/relevancy.html#ranking-rules) of the Index.
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let movie_index = client.get_or_create("movies").await.unwrap();
    /// let ranking_rules = movie_index.get_ranking_rules().await.unwrap();
    /// # }
    /// ```
    pub async fn get_ranking_rules(&self) -> Result<Vec<String>, Error> {
        Ok(request::<(), Vec<String>>(
            &format!("{}/indexes/{}/settings/ranking-rules", self.client.host, self.uid),
            self.client.apikey,
            Method::Get,
            200,
        ).await?)
    }

    /// Get [attributes for faceting](https://docs.meilisearch.com/guides/advanced_guides/faceted_search.html) of the Index.
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let movie_index = client.get_or_create("movies").await.unwrap();
    /// let attributes_for_faceting = movie_index.get_attributes_for_faceting().await.unwrap();
    /// # }
    /// ```
    pub async fn get_attributes_for_faceting(&self) -> Result<Vec<String>, Error> {
        Ok(request::<(), Vec<String>>(
            &format!("{}/indexes/{}/settings/attributes-for-faceting", self.client.host, self.uid),
            self.client.apikey,
            Method::Get,
            200,
        ).await?)
    }

    /// Get the [distinct attribute](https://docs.meilisearch.com/guides/advanced_guides/settings.html#distinct-attribute) of the Index.
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let movie_index = client.get_or_create("movies").await.unwrap();
    /// let distinct_attribute = movie_index.get_distinct_attribute().await.unwrap();
    /// # }
    /// ```
    pub async fn get_distinct_attribute(&self) -> Result<Option<String>, Error> {
        Ok(request::<(), Option<String>>(
            &format!("{}/indexes/{}/settings/distinct-attribute", self.client.host, self.uid),
            self.client.apikey,
            Method::Get,
            200,
        ).await?)
    }

    /// Get [searchable attributes](https://docs.meilisearch.com/guides/advanced_guides/field_properties.html#searchable-fields) of the Index.
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let movie_index = client.get_or_create("movies").await.unwrap();
    /// let searchable_attributes = movie_index.get_searchable_attributes().await.unwrap();
    /// # }
    /// ```
    pub async fn get_searchable_attributes(&self) -> Result<Vec<String>, Error> {
        Ok(request::<(), Vec<String>>(
            &format!("{}/indexes/{}/settings/searchable-attributes", self.client.host, self.uid),
            self.client.apikey,
            Method::Get,
            200,
        ).await?)
    }

    /// Get [displayed attributes](https://docs.meilisearch.com/guides/advanced_guides/settings.html#displayed-attributes) of the Index.
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let movie_index = client.get_or_create("movies").await.unwrap();
    /// let displayed_attributes = movie_index.get_displayed_attributes().await.unwrap();
    /// # }
    /// ```
    pub async fn get_displayed_attributes(&self) -> Result<Vec<String>, Error> {
        Ok(request::<(), Vec<String>>(
            &format!("{}/indexes/{}/settings/displayed-attributes", self.client.host, self.uid),
            self.client.apikey,
            Method::Get,
            200,
        ).await?)
    }

    /// Update [settings](../settings/struct.Settings.html) of the index.
    /// Updates in the settings are partial. This means that any parameters corresponding to a None value will be left unchanged.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let stop_words = vec![String::from("a"), String::from("the"), String::from("of")];
    /// let settings = Settings::new()
    ///     .with_stop_words(stop_words.clone());
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

    /// Update [synonyms](https://docs.meilisearch.com/guides/advanced_guides/synonyms.html) of the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "masterKey");
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

    /// Update [stop-words](https://docs.meilisearch.com/guides/advanced_guides/stop_words.html) of the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let stop_words = &["the", "of", "to"];
    /// let progress = movie_index.set_stop_words(stop_words).await.unwrap();
    /// # }
    /// ```
    pub async fn set_stop_words(&'a self, stop_words: &[&str]) -> Result<Progress<'a>, Error> {
        Ok(request::<&[&str], ProgressJson>(
            &format!("{}/indexes/{}/settings/stop-words", self.client.host, self.uid),
            self.client.apikey,
            Method::Post(stop_words),
            202,
        ).await?
        .into_progress(self))
    }

    /// Update [ranking rules](https://docs.meilisearch.com/guides/main_concepts/relevancy.html#ranking-rules) of the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let ranking_rules = &[
    ///     "typo",
    ///     "words",
    ///     "proximity",
    ///     "attribute",
    ///     "wordsPosition",
    ///     "exactness",
    ///     "asc(release_date)",
    ///     "desc(rank)",
    /// ];
    /// let progress = movie_index.set_ranking_rules(ranking_rules).await.unwrap();
    /// # }
    /// ```
    pub async fn set_ranking_rules(&'a self, ranking_rules: &[&str]) -> Result<Progress<'a>, Error> {
        Ok(request::<&[&str], ProgressJson>(
            &format!("{}/indexes/{}/settings/ranking-rules", self.client.host, self.uid),
            self.client.apikey,
            Method::Post(ranking_rules),
            202,
        ).await?
        .into_progress(self))
    }

    /// Update [attributes for faceting](https://docs.meilisearch.com/guides/advanced_guides/faceted_search.html) of the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let attributes_for_faceting = &["genre", "director"];
    /// let progress = movie_index.set_attributes_for_faceting(attributes_for_faceting).await.unwrap();
    /// # }
    /// ```
    pub async fn set_attributes_for_faceting(&'a self, attributes_for_faceting: &[&str]) -> Result<Progress<'a>, Error> {
        Ok(request::<&[&str], ProgressJson>(
            &format!("{}/indexes/{}/settings/attributes-for-faceting", self.client.host, self.uid),
            self.client.apikey,
            Method::Post(attributes_for_faceting),
            202,
        ).await?
        .into_progress(self))
    }

    /// Update the [distinct attribute](https://docs.meilisearch.com/guides/advanced_guides/settings.html#distinct-attribute) of the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let progress = movie_index.set_distinct_attribute("movie_id").await.unwrap();
    /// # }
    /// ```
    pub async fn set_distinct_attribute(&'a self, distinct_attribute: &str) -> Result<Progress<'a>, Error> {
        Ok(request::<&str, ProgressJson>(
            &format!("{}/indexes/{}/settings/distinct-attribute", self.client.host, self.uid),
            self.client.apikey,
            Method::Post(distinct_attribute),
            202,
        ).await?
        .into_progress(self))
    }

    /// Update [searchable attributes](https://docs.meilisearch.com/guides/advanced_guides/field_properties.html#searchable-fields) of the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let progress = movie_index.set_searchable_attributes(&["title", "description", "uid"]).await.unwrap();
    /// # }
    /// ```
    pub async fn set_searchable_attributes(&'a self, searchable_attributes: &[&str]) -> Result<Progress<'a>, Error> {
        Ok(request::<&[&str], ProgressJson>(
            &format!("{}/indexes/{}/settings/searchable-attributes", self.client.host, self.uid),
            self.client.apikey,
            Method::Post(searchable_attributes),
            202,
        ).await?
        .into_progress(self))
    }

    /// Update [displayed attributes](https://docs.meilisearch.com/guides/advanced_guides/settings.html#displayed-attributes) of the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let progress = movie_index.set_displayed_attributes(&["title", "description", "release_date", "rank", "poster"]).await.unwrap();
    /// # }
    /// ```
    pub async fn set_displayed_attributes(&'a self, displayed_attributes: &[&str]) -> Result<Progress<'a>, Error> {
        Ok(request::<&[&str], ProgressJson>(
            &format!("{}/indexes/{}/settings/displayed-attributes", self.client.host, self.uid),
            self.client.apikey,
            Method::Post(displayed_attributes),
            202,
        ).await?
        .into_progress(self))
    }

    /// Reset [settings](../settings/struct.Settings.html) of the index.
    /// All settings will be reset to their [default value](https://docs.meilisearch.com/references/settings.html#reset-settings).
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "masterKey");
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

    /// Reset [synonyms](https://docs.meilisearch.com/guides/advanced_guides/synonyms.html) of the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "masterKey");
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

    /// Reset [stop-words](https://docs.meilisearch.com/guides/advanced_guides/stop_words.html) of the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let progress = movie_index.reset_stop_words().await.unwrap();
    /// # }
    /// ```
    pub async fn reset_stop_words(&'a self) -> Result<Progress<'a>, Error> {
        Ok(request::<(), ProgressJson>(
            &format!("{}/indexes/{}/settings/stop-words", self.client.host, self.uid),
            self.client.apikey,
            Method::Delete,
            202,
        ).await?
        .into_progress(self))
    }

    /// Reset [ranking rules](https://docs.meilisearch.com/guides/main_concepts/relevancy.html#ranking-rules) of the index to default value.
    /// Default value: ["typo", "words", "proximity", "attribute", "wordsPosition", "exactness"].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let progress = movie_index.reset_ranking_rules().await.unwrap();
    /// # }
    /// ```
    pub async fn reset_ranking_rules(&'a self) -> Result<Progress<'a>, Error> {
        Ok(request::<(), ProgressJson>(
            &format!("{}/indexes/{}/settings/ranking-rules", self.client.host, self.uid),
            self.client.apikey,
            Method::Delete,
            202,
        ).await?
        .into_progress(self))
    }

    /// Reset [attributes for faceting](https://docs.meilisearch.com/guides/advanced_guides/faceted_search.html) of the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let progress = movie_index.reset_attributes_for_faceting().await.unwrap();
    /// # }
    /// ```
    pub async fn reset_attributes_for_faceting(&'a self) -> Result<Progress<'a>, Error> {
        Ok(request::<(), ProgressJson>(
            &format!("{}/indexes/{}/settings/attributes-for-faceting", self.client.host, self.uid),
            self.client.apikey,
            Method::Delete,
            202,
        ).await?
        .into_progress(self))
    }

    /// Reset the [distinct attribute](https://docs.meilisearch.com/guides/advanced_guides/settings.html#distinct-attribute) of the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let progress = movie_index.reset_distinct_attribute().await.unwrap();
    /// # }
    /// ```
    pub async fn reset_distinct_attribute(&'a self) -> Result<Progress<'a>, Error> {
        Ok(request::<(), ProgressJson>(
            &format!("{}/indexes/{}/settings/distinct-attribute", self.client.host, self.uid),
            self.client.apikey,
            Method::Delete,
            202,
        ).await?
        .into_progress(self))
    }

    /// Reset [searchable attributes](https://docs.meilisearch.com/guides/advanced_guides/field_properties.html#searchable-fields) of the index (enable all attributes).
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let progress = movie_index.reset_searchable_attributes().await.unwrap();
    /// # }
    /// ```
    pub async fn reset_searchable_attributes(&'a self) -> Result<Progress<'a>, Error> {
        Ok(request::<(), ProgressJson>(
            &format!("{}/indexes/{}/settings/searchable-attributes", self.client.host, self.uid),
            self.client.apikey,
            Method::Delete,
            202,
        ).await?
        .into_progress(self))
    }

    /// Reset [displayed attributes](https://docs.meilisearch.com/guides/advanced_guides/settings.html#displayed-attributes) of the index (enable all attributes).
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let progress = movie_index.reset_displayed_attributes().await.unwrap();
    /// # }
    /// ```
    pub async fn reset_displayed_attributes(&'a self) -> Result<Progress<'a>, Error> {
        Ok(request::<(), ProgressJson>(
            &format!("{}/indexes/{}/settings/displayed-attributes", self.client.host, self.uid),
            self.client.apikey,
            Method::Delete,
            202,
        ).await?
        .into_progress(self))
    }
}
