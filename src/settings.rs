use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{
    indexes::Index, 
    errors::Error, 
    request::{request, Method}, 
    progress::{Progress, ProgressJson}
};

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
#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// List of associated words treated similarly
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<HashMap<String, Vec<String>>>,
    /// List of words ignored by MeiliSearch when present in search queries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_words: Option<Vec<String>>,
    /// List of [ranking rules](https://docs.meilisearch.com/learn/core_concepts/relevancy.html#order-of-the-rules) sorted by order of importance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranking_rules: Option<Vec<String>>,
    /// Attributes to use as [facets](https://docs.meilisearch.com/reference/features/faceted_search.html)
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

pub trait IntoVecString: Sized {
    fn convert(self) -> Vec<String>;
}

impl IntoVecString for &[&str] {
    #[inline]
    fn convert(self) -> Vec<String> {
        let mut vec = Vec::new();
        for item in self {
            vec.push((*item).into())
        }
        vec
    }
}

impl IntoVecString for Vec<&str> {
    #[inline]
    fn convert(self) -> Vec<String> {
        let mut vec = Vec::new();
        for item in self {
            vec.push((*item).into())
        }
        vec
    }
}

impl IntoVecString for Vec<String> {
    #[inline]
    fn convert(self) -> Vec<String> {
        self
    }
}

impl IntoVecString for &[String] {
    #[inline]
    fn convert(self) -> Vec<String> {
        let mut vec = Vec::new();
        for item in self {
            vec.push(item.clone())
        }
        vec
    }
}

impl IntoVecString for &[&String] {
    #[inline]
    fn convert(self) -> Vec<String> {
        let mut vec = Vec::new();
        for item in self {
            vec.push((*item).clone())
        }
        vec
    }
}

impl<const N: usize> IntoVecString for &[String; N] {
    fn convert(self) -> Vec<String> {
        let mut vec = Vec::new();
        for item in self {
            vec.push((*item).clone())
        }
        vec
    }
}

impl<const N: usize> IntoVecString for &[&str; N] {
    fn convert(self) -> Vec<String> {
        let mut vec = Vec::new();
        for item in self {
            vec.push((*item).to_string())
        }
        vec
    }
}

impl<const N: usize> IntoVecString for [String; N] {
    fn convert(self) -> Vec<String> {
        let mut vec = Vec::new();
        for item in self.iter() {
            vec.push((*item).clone())
        }
        vec
    }
}

impl<const N: usize> IntoVecString for [&str; N] {
    fn convert(self) -> Vec<String> {
        let mut vec = Vec::new();
        for item in self.iter() {
            vec.push((*item).to_string())
        }
        vec
    }
}

#[allow(missing_docs)]
impl Settings {
    
    /// Create undefined settings
    pub fn new() -> Self {
        Self {
            synonyms: None,
            stop_words: None,
            ranking_rules: None,
            attributes_for_faceting: None,
            distinct_attribute: None,
            searchable_attributes: None,
            displayed_attributes: None,
        }
    }

    pub fn with_synonyms<S: AsRef<str>, U: IntoVecString>(self, synonyms: HashMap<S, U>) -> Settings {
        let mut converted_synonyms = HashMap::new();
        for (key, array) in synonyms {
            let key: String = key.as_ref().to_string();
            let array: Vec<String> = array.convert();
            converted_synonyms.insert(key, array);
        }

        Settings {
            synonyms: Some(converted_synonyms),
            ..self
        }
    }

    pub fn with_stop_words(self, stop_words: impl IntoVecString) -> Settings {
        Settings {
            stop_words: Some(stop_words.convert()),
            ..self
        }
    }

    pub fn with_ranking_rules<T: IntoVecString>(self, ranking_rules: T) -> Settings
    {
        Settings {
            ranking_rules: Some(ranking_rules.convert()),
            ..self
        }
    }

    pub fn with_attributes_for_faceting<T: IntoVecString>(self, attributes_for_faceting: T) -> Settings {
        Settings {
            attributes_for_faceting: Some(attributes_for_faceting.convert()),
            ..self
        }
    }

    pub fn with_distinct_attribute<S: AsRef<str>>(self, distinct_attribute: S) -> Settings {
        Settings {
            distinct_attribute: Some(distinct_attribute.as_ref().into()),
            ..self
        }
    }

    pub fn with_searchable_attributes<T: IntoVecString>(self, searchable_attributes: T) -> Settings {
        Settings {
            searchable_attributes: Some(searchable_attributes.convert()),
            ..self
        }
    }

    pub fn with_displayed_attributes<T: IntoVecString>(self, displayed_attributes: T) -> Settings {
        Settings {
            displayed_attributes: Some(displayed_attributes.convert()),
            ..self
        }
    }

}

impl Index {
    /// Get [settings](../settings/struct.Settings.html) of the Index.
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let movie_index = client.get_or_create("movies").await.unwrap();
    /// let settings = movie_index.get_settings().await.unwrap();
    /// # });
    /// ```
    pub async fn get_settings(&self) -> Result<Settings, Error> {
        Ok(request::<(), Settings>(
            &format!("{}/indexes/{}/settings", &self.config.host, self.uid),
            &self.config.api_key,
            Method::Get,
            200,
        ).await?)
    }

    /// Get [synonyms](https://docs.meilisearch.com/reference/features/synonyms.html) of the Index.
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let movie_index = client.get_or_create("movies").await.unwrap();
    /// let synonyms = movie_index.get_synonyms().await.unwrap();
    /// # });
    /// ```
    pub async fn get_synonyms(&self) -> Result<HashMap<String, Vec<String>>, Error> {
        Ok(request::<(), HashMap<String, Vec<String>>>(
            &format!("{}/indexes/{}/settings/synonyms", self.config.host, self.uid),
            &self.config.api_key,
            Method::Get,
            200,
        ).await?)
    }

    /// Get [stop-words](https://docs.meilisearch.com/reference/features/stop_words.html) of the Index.
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let movie_index = client.get_or_create("movies").await.unwrap();
    /// let stop_words = movie_index.get_stop_words().await.unwrap();
    /// # });
    /// ```
    pub async fn get_stop_words(&self) -> Result<Vec<String>, Error> {
        Ok(request::<(), Vec<String>>(
            &format!("{}/indexes/{}/settings/stop-words", self.config.host, self.uid),
            &self.config.api_key,
            Method::Get,
            200,
        ).await?)
    }

    /// Get [ranking rules](https://docs.meilisearch.com/learn/core_concepts/relevancy.html#ranking-rules) of the Index.
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let movie_index = client.get_or_create("movies").await.unwrap();
    /// let ranking_rules = movie_index.get_ranking_rules().await.unwrap();
    /// # });
    /// ```
    pub async fn get_ranking_rules(&self) -> Result<Vec<String>, Error> {
        Ok(request::<(), Vec<String>>(
            &format!("{}/indexes/{}/settings/ranking-rules", self.config.host, self.uid),
            &self.config.api_key,
            Method::Get,
            200,
        ).await?)
    }

    /// Get [attributes for faceting](https://docs.meilisearch.com/reference/features/faceted_search.html) of the Index.
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let movie_index = client.get_or_create("movies").await.unwrap();
    /// let attributes_for_faceting = movie_index.get_attributes_for_faceting().await.unwrap();
    /// # });
    /// ```
    pub async fn get_attributes_for_faceting(&self) -> Result<Vec<String>, Error> {
        Ok(request::<(), Vec<String>>(
            &format!("{}/indexes/{}/settings/attributes-for-faceting", self.config.host, self.uid),
            &self.config.api_key,
            Method::Get,
            200,
        ).await?)
    }

    /// Get the [distinct attribute](https://docs.meilisearch.com/reference/features/settings.html#distinct-attribute) of the Index.
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let movie_index = client.get_or_create("movies").await.unwrap();
    /// let distinct_attribute = movie_index.get_distinct_attribute().await.unwrap();
    /// # });
    /// ```
    pub async fn get_distinct_attribute(&self) -> Result<Option<String>, Error> {
        Ok(request::<(), Option<String>>(
            &format!("{}/indexes/{}/settings/distinct-attribute", self.config.host, self.uid),
            &self.config.api_key,
            Method::Get,
            200,
        ).await?)
    }

    /// Get [searchable attributes](https://docs.meilisearch.com/reference/features/field_properties.html#searchable-fields) of the Index.
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let movie_index = client.get_or_create("movies").await.unwrap();
    /// let searchable_attributes = movie_index.get_searchable_attributes().await.unwrap();
    /// # });
    /// ```
    pub async fn get_searchable_attributes(&self) -> Result<Vec<String>, Error> {
        Ok(request::<(), Vec<String>>(
            &format!("{}/indexes/{}/settings/searchable-attributes", self.config.host, self.uid),
            &self.config.api_key,
            Method::Get,
            200,
        ).await?)
    }

    /// Get [displayed attributes](https://docs.meilisearch.com/reference/features/settings.html#displayed-attributes) of the Index.
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let movie_index = client.get_or_create("movies").await.unwrap();
    /// let displayed_attributes = movie_index.get_displayed_attributes().await.unwrap();
    /// # });
    /// ```
    pub async fn get_displayed_attributes(&self) -> Result<Vec<String>, Error> {
        Ok(request::<(), Vec<String>>(
            &format!("{}/indexes/{}/settings/displayed-attributes", self.config.host, self.uid),
            &self.config.api_key,
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
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let stop_words = vec![String::from("a"), String::from("the"), String::from("of")];
    /// let settings = Settings::new()
    ///     .with_stop_words(stop_words.clone());
    ///
    /// let progress = movie_index.set_settings(&settings).await.unwrap();
    /// # std::thread::sleep(std::time::Duration::from_secs(2));
    /// # progress.get_status().await.unwrap();
    /// # });
    /// ```
    pub async fn set_settings(&self, settings: &Settings) -> Result<Progress, Error> {
        Ok(request::<&Settings, ProgressJson>(
            &format!("{}/indexes/{}/settings", self.config.host, self.uid),
            &self.config.api_key,
            Method::Post(settings),
            202,
        ).await?
        .into_progress(self))
    }

    /// Update [synonyms](https://docs.meilisearch.com/reference/features/synonyms.html) of the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let mut synonyms = std::collections::HashMap::new();
    /// synonyms.insert(String::from("wolverine"), vec![String::from("xmen"), String::from("logan")]);
    /// synonyms.insert(String::from("logan"), vec![String::from("xmen"), String::from("wolverine")]);
    /// synonyms.insert(String::from("wow"), vec![String::from("world of warcraft")]);
    ///
    /// let progress = movie_index.set_synonyms(&synonyms).await.unwrap();
    /// # std::thread::sleep(std::time::Duration::from_secs(2));
    /// # progress.get_status().await.unwrap();
    /// # });
    /// ```
    pub async fn set_synonyms(&self, synonyms: &HashMap<String, Vec<String>>) -> Result<Progress, Error> {
        Ok(request::<&HashMap<String, Vec<String>>, ProgressJson>(
            &format!("{}/indexes/{}/settings/synonyms", self.config.host, self.uid),
            &self.config.api_key,
            Method::Post(synonyms),
            202,
        ).await?
        .into_progress(self))
    }

    /// Update [stop-words](https://docs.meilisearch.com/reference/features/stop_words.html) of the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let stop_words = ["the", "of", "to"];
    /// let progress = movie_index.set_stop_words(&stop_words).await.unwrap();
    /// # std::thread::sleep(std::time::Duration::from_secs(2));
    /// # progress.get_status().await.unwrap();
    /// # });
    /// ```
    pub async fn set_stop_words(&self, stop_words: impl IntoVecString) -> Result<Progress, Error> {
        Ok(request::<Vec<String>, ProgressJson>(
            &format!("{}/indexes/{}/settings/stop-words", self.config.host, self.uid),
            &self.config.api_key,
            Method::Post(stop_words.convert()),
            202,
        ).await?
        .into_progress(self))
    }

    /// Update [ranking rules](https://docs.meilisearch.com/learn/core_concepts/relevancy.html#ranking-rules) of the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let ranking_rules = [
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
    /// # std::thread::sleep(std::time::Duration::from_secs(2));
    /// # progress.get_status().await.unwrap();
    /// # });
    /// ```
    pub async fn set_ranking_rules(&self, ranking_rules: impl IntoVecString) -> Result<Progress, Error> {
        Ok(request::<Vec<String>, ProgressJson>(
            &format!("{}/indexes/{}/settings/ranking-rules", self.config.host, self.uid),
            &self.config.api_key,
            Method::Post(ranking_rules.convert()),
            202,
        ).await?
        .into_progress(self))
    }

    /// Update [attributes for faceting](https://docs.meilisearch.com/reference/features/faceted_search.html) of the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let attributes_for_faceting = ["genre", "director"];
    /// let progress = movie_index.set_attributes_for_faceting(&attributes_for_faceting).await.unwrap();
    /// # std::thread::sleep(std::time::Duration::from_secs(2));
    /// # progress.get_status().await.unwrap();
    /// # });
    /// ```
    pub async fn set_attributes_for_faceting(&self, attributes_for_faceting: impl IntoVecString) -> Result<Progress, Error> {
        Ok(request::<Vec<String>, ProgressJson>(
            &format!("{}/indexes/{}/settings/attributes-for-faceting", self.config.host, self.uid),
            &self.config.api_key,
            Method::Post(attributes_for_faceting.convert()),
            202,
        ).await?
        .into_progress(self))
    }

    /// Update the [distinct attribute](https://docs.meilisearch.com/reference/features/settings.html#distinct-attribute) of the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let progress = movie_index.set_distinct_attribute("movie_id").await.unwrap();
    /// # std::thread::sleep(std::time::Duration::from_secs(2));
    /// # progress.get_status().await.unwrap();
    /// # });
    /// ```
    pub async fn set_distinct_attribute(&self, distinct_attribute: impl AsRef<str>) -> Result<Progress, Error> {
        Ok(request::<String, ProgressJson>(
            &format!("{}/indexes/{}/settings/distinct-attribute", self.config.host, self.uid),
            &self.config.api_key,
            Method::Post(distinct_attribute.as_ref().into()),
            202,
        ).await?
        .into_progress(self))
    }

    /// Update [searchable attributes](https://docs.meilisearch.com/reference/features/field_properties.html#searchable-fields) of the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let progress = movie_index.set_searchable_attributes(["title", "description", "uid"]).await.unwrap();
    /// # std::thread::sleep(std::time::Duration::from_secs(2));
    /// # progress.get_status().await.unwrap();
    /// # });
    /// ```
    pub async fn set_searchable_attributes(&self, searchable_attributes: impl IntoVecString) -> Result<Progress, Error> {
        Ok(request::<Vec<String>, ProgressJson>(
            &format!("{}/indexes/{}/settings/searchable-attributes", self.config.host, self.uid),
            &self.config.api_key,
            Method::Post(searchable_attributes.convert()),
            202,
        ).await?
        .into_progress(self))
    }

    /// Update [displayed attributes](https://docs.meilisearch.com/reference/features/settings.html#displayed-attributes) of the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let progress = movie_index.set_displayed_attributes(["title", "description", "release_date", "rank", "poster"]).await.unwrap();
    /// # std::thread::sleep(std::time::Duration::from_secs(2));
    /// # progress.get_status().await.unwrap();
    /// # });
    /// ```
    pub async fn set_displayed_attributes(&self, displayed_attributes: impl IntoVecString) -> Result<Progress, Error> {
        Ok(request::<Vec<String>, ProgressJson>(
            &format!("{}/indexes/{}/settings/displayed-attributes", self.config.host, self.uid),
            &self.config.api_key,
            Method::Post(displayed_attributes.convert()),
            202,
        ).await?
        .into_progress(self))
    }

    /// Reset [settings](../settings/struct.Settings.html) of the index.
    /// All settings will be reset to their [default value](https://docs.meilisearch.com/reference/api/settings.html#reset-settings).
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let progress = movie_index.reset_settings().await.unwrap();
    /// # std::thread::sleep(std::time::Duration::from_secs(2));
    /// # progress.get_status().await.unwrap();
    /// # });
    /// ```
    pub async fn reset_settings(&self) -> Result<Progress, Error> {
        Ok(request::<(), ProgressJson>(
            &format!("{}/indexes/{}/settings", self.config.host, self.uid),
            &self.config.api_key,
            Method::Delete,
            202,
        ).await?
        .into_progress(self))
    }

    /// Reset [synonyms](https://docs.meilisearch.com/reference/features/synonyms.html) of the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let progress = movie_index.reset_synonyms().await.unwrap();
    /// # std::thread::sleep(std::time::Duration::from_secs(2));
    /// # progress.get_status().await.unwrap();
    /// # });
    /// ```
    pub async fn reset_synonyms(&self) -> Result<Progress, Error> {
        Ok(request::<(), ProgressJson>(
            &format!("{}/indexes/{}/settings/synonyms", self.config.host, self.uid),
            &self.config.api_key,
            Method::Delete,
            202,
        ).await?
        .into_progress(self))
    }

    /// Reset [stop-words](https://docs.meilisearch.com/reference/features/stop_words.html) of the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let progress = movie_index.reset_stop_words().await.unwrap();
    /// # std::thread::sleep(std::time::Duration::from_secs(2));
    /// # progress.get_status().await.unwrap();
    /// # });
    /// ```
    pub async fn reset_stop_words(&self) -> Result<Progress, Error> {
        Ok(request::<(), ProgressJson>(
            &format!("{}/indexes/{}/settings/stop-words", self.config.host, self.uid),
            &self.config.api_key,
            Method::Delete,
            202,
        ).await?
        .into_progress(self))
    }

    /// Reset [ranking rules](https://docs.meilisearch.com/learn/core_concepts/relevancy.html#ranking-rules) of the index to default value.
    /// Default value: ["typo", "words", "proximity", "attribute", "wordsPosition", "exactness"].
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let progress = movie_index.reset_ranking_rules().await.unwrap();
    /// # std::thread::sleep(std::time::Duration::from_secs(2));
    /// # progress.get_status().await.unwrap();
    /// # });
    /// ```
    pub async fn reset_ranking_rules(&self) -> Result<Progress, Error> {
        Ok(request::<(), ProgressJson>(
            &format!("{}/indexes/{}/settings/ranking-rules", self.config.host, self.uid),
            &self.config.api_key,
            Method::Delete,
            202,
        ).await?
        .into_progress(self))
    }

    /// Reset [attributes for faceting](https://docs.meilisearch.com/reference/features/faceted_search.html) of the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let progress = movie_index.reset_attributes_for_faceting().await.unwrap();
    /// # std::thread::sleep(std::time::Duration::from_secs(2));
    /// # progress.get_status().await.unwrap();
    /// # });
    /// ```
    pub async fn reset_attributes_for_faceting(&self) -> Result<Progress, Error> {
        Ok(request::<(), ProgressJson>(
            &format!("{}/indexes/{}/settings/attributes-for-faceting", self.config.host, self.uid),
            &self.config.api_key,
            Method::Delete,
            202,
        ).await?
        .into_progress(self))
    }

    /// Reset the [distinct attribute](https://docs.meilisearch.com/reference/features/settings.html#distinct-attribute) of the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let progress = movie_index.reset_distinct_attribute().await.unwrap();
    /// # std::thread::sleep(std::time::Duration::from_secs(2));
    /// # progress.get_status().await.unwrap();
    /// # });
    /// ```
    pub async fn reset_distinct_attribute(&self) -> Result<Progress, Error> {
        Ok(request::<(), ProgressJson>(
            &format!("{}/indexes/{}/settings/distinct-attribute", self.config.host, self.uid),
            &self.config.api_key,
            Method::Delete,
            202,
        ).await?
        .into_progress(self))
    }

    /// Reset [searchable attributes](https://docs.meilisearch.com/reference/features/field_properties.html#searchable-fields) of the index (enable all attributes).
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let progress = movie_index.reset_searchable_attributes().await.unwrap();
    /// # std::thread::sleep(std::time::Duration::from_secs(2));
    /// # progress.get_status().await.unwrap();
    /// # });
    /// ```
    pub async fn reset_searchable_attributes(&self) -> Result<Progress, Error> {
        Ok(request::<(), ProgressJson>(
            &format!("{}/indexes/{}/settings/searchable-attributes", self.config.host, self.uid),
            &self.config.api_key,
            Method::Delete,
            202,
        ).await?
        .into_progress(self))
    }

    /// Reset [displayed attributes](https://docs.meilisearch.com/reference/features/settings.html#displayed-attributes) of the index (enable all attributes).
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # futures::executor::block_on(async move {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let progress = movie_index.reset_displayed_attributes().await.unwrap();
    /// # std::thread::sleep(std::time::Duration::from_secs(2));
    /// # progress.get_status().await.unwrap();
    /// # });
    /// ```
    pub async fn reset_displayed_attributes(&self) -> Result<Progress, Error> {
        Ok(request::<(), ProgressJson>(
            &format!("{}/indexes/{}/settings/displayed-attributes", self.config.host, self.uid),
            &self.config.api_key,
            Method::Delete,
            202,
        ).await?
        .into_progress(self))
    }
}
