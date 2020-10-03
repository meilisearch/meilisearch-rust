//! There are several ways to set and get settings on an [Index](../indexes/struct.Index.html).\
//! \
//! First, there are [individual methods](../indexes/struct.Index.html#impl) you can use to set a single setting.\
//! \
//! There is also a [Settings](struct.Settings.html) struct used to get or set a batch of settings.\
//! The fields of this struct are generic so type inference won't work on unused fields.
//! If you don't want to set the types manually for unused fields, you can use [shortcuts types](#types) at the cost of flexibility.

use crate::{
    errors::Error,
    indexes::Index,
    progress::{Progress, ProgressJson},
    request::{request, Method},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;

/// Meilisearch server settings.\
/// \
/// [*See also the `meilisearch_sdk::settings` module*](index.html).\
/// \
/// This struct contains all Meilisearch settings available.\
/// Fields are generic and optional so Rust's type inference will be limited with this struct.
/// You may have to specify manually all the types.\
/// You might prefer to use [methods of the `Index` struct](../indexes/struct.Index.html#impl) to set settings one by one.\
/// 
/// # Examples
/// 
/// ```
/// # use std::collections::HashMap;
/// # use meilisearch_sdk::settings::Settings;
/// let mut synonyms: HashMap<&str, &[&str]> = HashMap::new();
/// synonyms.insert("green", &["emerald", "viridescent"]);
/// synonyms.insert("fast", &["speedy", "quick", "rapid", "swift", "turbo"]);
/// 
/// let settings = Settings::new()
///     .with_distinct_attribute("id")
///     .with_synonyms(synonyms)
///     .with_stop_words(&["a", "the", "and"])
///     .with_attributes_for_faceting(&["genres"])
///     .with_displayed_attributes(&["title", "description", "genres"])
///     .with_searchable_attributes(&["title", "description"])
///     .with_ranking_rules(&["typo", "words", "proximity", "attribute", "wordsPosition", "exactness"]);
/// ```
/// 
/// ```
/// # use std::collections::HashMap;
/// # use meilisearch_sdk::settings::Settings;
/// let mut synonyms: HashMap<&str, &[&str]> = HashMap::new();
/// synonyms.insert("green", &["emerald", "viridescent"]);
/// synonyms.insert("fast", &["speedy", "quick", "rapid", "swift", "turbo"]);
/// 
/// // type inference does not work for unused fields (see module documentation for shortcuts)
/// let settings = Settings::<_,_,_,_,&[&str],_,&[&str],&[&str]>::new()
///     .with_distinct_attribute("id")
///     .with_synonyms(synonyms)
///     .with_stop_words(&["a", "the", "and"])
///     .with_ranking_rules(&["typo", "words", "proximity", "attribute", "wordsPosition", "exactness"]);
/// ```
#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Settings<
    SynKey,
    SynList,
    SWordsList,
    RankList,
    FacetsList,
    DAttribute,
    SearchableList,
    DisplayedList,
> where
    SynKey: std::cmp::Eq + std::hash::Hash,
    SynList: IntoIterator,
    SWordsList: IntoIterator,
    RankList: IntoIterator,
    FacetsList: IntoIterator,
    SearchableList: IntoIterator,
    DisplayedList: IntoIterator,
{
    /// List of associated words treated similarly
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<HashMap<SynKey, SynList>>,
    /// List of words ignored by MeiliSearch when present in search queries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_words: Option<SWordsList>,
    /// List of [ranking rules](https://docs.meilisearch.com/guides/main_concepts/relevancy.html#order-of-the-rules) sorted by order of importance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranking_rules: Option<RankList>,
    /// Attributes to use as [facets](https://docs.meilisearch.com/guides/advanced_guides/faceted_search.html)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_for_faceting: Option<FacetsList>,
    /// Search returns documents with distinct (different) values of the given field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distinct_attribute: Option<DAttribute>,
    /// Fields in which to search for matching query words sorted by order of importance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchable_attributes: Option<SearchableList>,
    /// Fields displayed in the returned documents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub displayed_attributes: Option<DisplayedList>,
}

/// A shortcut type for [Settings](struct.Settings.html) containing owned data only.\
/// Fields are stored as `Vec<String>`, `String` or `HashMap<String, Vec<String>>`.\
/// It is useful for using the builder syntax without any type inference issue.\
/// \
/// # Example
/// \
/// ```
/// # use std::collections::HashMap;
/// # use meilisearch_sdk::settings::OwnedSettings;
/// let mut synonyms: HashMap<String, Vec<String>> = HashMap::new();
/// synonyms.insert("green".to_string(), vec!["emerald".to_string(), "viridescent".to_string()]);
/// synonyms.insert("fast".to_string(), vec!["speedy".to_string(), "quick".to_string(), "rapid".to_string(), "swift".to_string(), "turbo".to_string()]);
/// 
/// let settings = OwnedSettings::new()
///     .with_distinct_attribute("id".to_string())
///     .with_synonyms(synonyms)
///     .with_stop_words(vec!["a".to_string(), "the".to_string(), "and".to_string()])
///     .with_ranking_rules(vec!["typo".to_string(), "words".to_string(), "proximity".to_string(), "attribute".to_string(), "wordsPosition".to_string(), "exactness".to_string()]);
/// ```
pub type OwnedSettings = GenericSettings<String, Vec<String>>;
/// A shortcut type for [Settings](struct.Settings.html) containing borrowed data only.\
/// Fields are stored as `&[&str]`, `&str` or `HashMap<&str, &[&str]>`.\
/// It is useful for using the builder syntax without any type inference issue.\
/// \
/// # Example
/// \
/// ```
/// # use std::collections::HashMap;
/// # use meilisearch_sdk::settings::BorrowedSettings;
/// let mut synonyms: HashMap<&str, &[&str]> = HashMap::new();
/// synonyms.insert("green", &["emerald", "viridescent"]);
/// synonyms.insert("fast", &["speedy", "quick", "rapid", "swift", "turbo"]);
/// 
/// let settings = BorrowedSettings::new()
///     .with_distinct_attribute("id")
///     .with_synonyms(synonyms)
///     .with_stop_words(&["a", "the", "and"])
///     .with_ranking_rules(&["typo", "words", "proximity", "attribute", "wordsPosition", "exactness"]);
/// ```
pub type BorrowedSettings<'a> = GenericSettings<&'a str, &'a [&'a str]>;
/// A shortcut type for [Settings](struct.Settings.html) for which fields have the exact same type.\
/// Fields are stored as `T`, `U` or `HashMap<T, U>`.\
/// It is useful for using the builder syntax without any type inference issue.
pub type GenericSettings<T, U> = Settings<T, U, U, U, U, T, U, U>;

#[allow(missing_docs)]
impl<
        SynKey: std::cmp::Eq + std::hash::Hash,
        SynList: IntoIterator,
        SWordsList: IntoIterator,
        RankList: IntoIterator,
        FacetsList: IntoIterator,
        DAttribute,
        SearchableList: IntoIterator,
        DisplayedList: IntoIterator,
    >
    Settings<
        SynKey,
        SynList,
        SWordsList,
        RankList,
        FacetsList,
        DAttribute,
        SearchableList,
        DisplayedList,
    >
{
    /// Create undefined settings
    pub fn new() -> Settings<
        SynKey,
        SynList,
        SWordsList,
        RankList,
        FacetsList,
        DAttribute,
        SearchableList,
        DisplayedList,
    > {
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
    pub fn with_synonyms(
        self,
        synonyms: HashMap<SynKey, SynList>,
    ) -> Settings<
        SynKey,
        SynList,
        SWordsList,
        RankList,
        FacetsList,
        DAttribute,
        SearchableList,
        DisplayedList,
    > {
        Settings {
            synonyms: Some(synonyms),
            ..self
        }
    }
    pub fn with_stop_words(
        self,
        stop_words: SWordsList,
    ) -> Settings<
        SynKey,
        SynList,
        SWordsList,
        RankList,
        FacetsList,
        DAttribute,
        SearchableList,
        DisplayedList,
    > {
        Settings {
            stop_words: Some(stop_words),
            ..self
        }
    }
    pub fn with_ranking_rules(
        self,
        ranking_rules: RankList,
    ) -> Settings<
        SynKey,
        SynList,
        SWordsList,
        RankList,
        FacetsList,
        DAttribute,
        SearchableList,
        DisplayedList,
    > {
        Settings {
            ranking_rules: Some(ranking_rules),
            ..self
        }
    }
    pub fn with_attributes_for_faceting(
        self,
        attributes_for_faceting: FacetsList,
    ) -> Settings<
        SynKey,
        SynList,
        SWordsList,
        RankList,
        FacetsList,
        DAttribute,
        SearchableList,
        DisplayedList,
    > {
        Settings {
            attributes_for_faceting: Some(attributes_for_faceting),
            ..self
        }
    }
    pub fn with_distinct_attribute(
        self,
        distinct_attribute: DAttribute,
    ) -> Settings<
        SynKey,
        SynList,
        SWordsList,
        RankList,
        FacetsList,
        DAttribute,
        SearchableList,
        DisplayedList,
    > {
        Settings {
            distinct_attribute: Some(distinct_attribute),
            ..self
        }
    }
    pub fn with_searchable_attributes(
        self,
        searchable_attributes: SearchableList,
    ) -> Settings<
        SynKey,
        SynList,
        SWordsList,
        RankList,
        FacetsList,
        DAttribute,
        SearchableList,
        DisplayedList,
    > {
        Settings {
            searchable_attributes: Some(searchable_attributes),
            ..self
        }
    }
    pub fn with_displayed_attributes(
        self,
        displayed_attributes: DisplayedList,
    ) -> Settings<
        SynKey,
        SynList,
        SWordsList,
        RankList,
        FacetsList,
        DAttribute,
        SearchableList,
        DisplayedList,
    > {
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
    pub async fn get_settings(
        &self,
    ) -> Result<OwnedSettings, Error> {
        Ok(request::<(), OwnedSettings>(
            &format!("{}/indexes/{}/settings", self.client.host, self.uid),
            self.client.apikey,
            Method::Get,
            200,
        )
        .await?)
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
            &format!(
                "{}/indexes/{}/settings/synonyms",
                self.client.host, self.uid
            ),
            self.client.apikey,
            Method::Get,
            200,
        )
        .await?)
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
            &format!(
                "{}/indexes/{}/settings/stop-words",
                self.client.host, self.uid
            ),
            self.client.apikey,
            Method::Get,
            200,
        )
        .await?)
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
            &format!(
                "{}/indexes/{}/settings/ranking-rules",
                self.client.host, self.uid
            ),
            self.client.apikey,
            Method::Get,
            200,
        )
        .await?)
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
            &format!(
                "{}/indexes/{}/settings/attributes-for-faceting",
                self.client.host, self.uid
            ),
            self.client.apikey,
            Method::Get,
            200,
        )
        .await?)
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
            &format!(
                "{}/indexes/{}/settings/distinct-attribute",
                self.client.host, self.uid
            ),
            self.client.apikey,
            Method::Get,
            200,
        )
        .await?)
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
            &format!(
                "{}/indexes/{}/settings/searchable-attributes",
                self.client.host, self.uid
            ),
            self.client.apikey,
            Method::Get,
            200,
        )
        .await?)
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
            &format!(
                "{}/indexes/{}/settings/displayed-attributes",
                self.client.host, self.uid
            ),
            self.client.apikey,
            Method::Get,
            200,
        )
        .await?)
    }

    /// Update [settings](../settings/struct.Settings.html) of the index.
    /// Updates in the settings are partial. This means that any parameters corresponding to a None value will be left unchanged.
    ///
    /// # Example
    ///
    /// ```
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::BorrowedSettings};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    /// 
    /// let settings = BorrowedSettings::new()
    ///     .with_stop_words(&["a", "the", "of"]);
    ///
    /// let progress = movie_index.set_settings(&settings).await.unwrap();
    /// # }
    /// ```
    pub async fn set_settings<
        SynKey: Debug + Serialize + std::cmp::Eq + std::hash::Hash,
        SynList: Debug + Serialize + IntoIterator,
        SWordsList: Debug + Serialize + IntoIterator,
        RankList: Debug + Serialize + IntoIterator,
        FacetsList: Debug + Serialize + IntoIterator,
        DAttribute: Debug + Serialize,
        SearchableList: Debug + Serialize + IntoIterator,
        DisplayedList: Debug + Serialize + IntoIterator,
    >(
        &'a self,
        settings: &Settings<
            SynKey,
            SynList,
            SWordsList,
            RankList,
            FacetsList,
            DAttribute,
            SearchableList,
            DisplayedList,
        >,
    ) -> Result<Progress<'a>, Error> {
        Ok(request::<
            &Settings<
                SynKey,
                SynList,
                SWordsList,
                RankList,
                FacetsList,
                DAttribute,
                SearchableList,
                DisplayedList,
            >,
            ProgressJson,
        >(
            &format!("{}/indexes/{}/settings", self.client.host, self.uid),
            self.client.apikey,
            Method::Post(settings),
            202,
        )
        .await?
        .into_progress(self))
    }

    /// Update [synonyms](https://docs.meilisearch.com/guides/advanced_guides/synonyms.html) of the index.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::collections::HashMap;
    /// # use meilisearch_sdk::{client::*, indexes::*, document::*, settings::Settings};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = Client::new("http://localhost:7700", "masterKey");
    /// let mut movie_index = client.get_or_create("movies").await.unwrap();
    ///
    /// let mut synonyms: HashMap<&str, &[&str]> = HashMap::new();
    /// synonyms.insert("wolverine", &["xmen", "logan"]);
    /// synonyms.insert("logan", &["xmen", "wolverine"]);
    /// synonyms.insert("wow", &["world of warcraft"]);
    ///
    /// let progress = movie_index.set_synonyms(&synonyms).await.unwrap();
    /// # }
    /// ```
    pub async fn set_synonyms<U: Debug + Serialize + std::cmp::Eq + std::hash::Hash + ToString, T: Debug + Serialize + IntoIterator<Item = impl ToString>>(
        &'a self,
        synonyms: &HashMap<U, T>,
    ) -> Result<Progress<'a>, Error> {
        Ok(request::<&HashMap<U, T>, ProgressJson>(
            &format!(
                "{}/indexes/{}/settings/synonyms",
                self.client.host, self.uid
            ),
            self.client.apikey,
            Method::Post(synonyms),
            202,
        )
        .await?
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
    /// let progress = movie_index.set_stop_words(&["the", "of", "to"]).await.unwrap();
    /// # }
    /// ```
    pub async fn set_stop_words<T: Debug + Serialize + IntoIterator<Item = impl ToString>>(&'a self, stop_words: T) -> Result<Progress<'a>, Error> {
        Ok(request::<T, ProgressJson>(
            &format!(
                "{}/indexes/{}/settings/stop-words",
                self.client.host, self.uid
            ),
            self.client.apikey,
            Method::Post(stop_words),
            202,
        )
        .await?
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
    pub async fn set_ranking_rules<T: Debug + Serialize + IntoIterator<Item = impl ToString>>(
        &'a self,
        ranking_rules: T,
    ) -> Result<Progress<'a>, Error> {
        Ok(request::<T, ProgressJson>(
            &format!(
                "{}/indexes/{}/settings/ranking-rules",
                self.client.host, self.uid
            ),
            self.client.apikey,
            Method::Post(ranking_rules),
            202,
        )
        .await?
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
    /// let progress = movie_index.set_attributes_for_faceting(&["genre", "director"]).await.unwrap();
    /// # }
    /// ```
    pub async fn set_attributes_for_faceting<T: Debug + Serialize + IntoIterator<Item = impl ToString>>(
        &'a self,
        ranking_rules: T,
    ) -> Result<Progress<'a>, Error> {
        Ok(request::<T, ProgressJson>(
            &format!(
                "{}/indexes/{}/settings/attributes-for-faceting",
                self.client.host, self.uid
            ),
            self.client.apikey,
            Method::Post(ranking_rules),
            202,
        )
        .await?
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
    pub async fn set_distinct_attribute<T: Debug + Serialize + ToString>(
        &'a self,
        distinct_attribute: T,
    ) -> Result<Progress<'a>, Error> {
        Ok(request::<T, ProgressJson>(
            &format!(
                "{}/indexes/{}/settings/distinct-attribute",
                self.client.host, self.uid
            ),
            self.client.apikey,
            Method::Post(distinct_attribute),
            202,
        )
        .await?
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
    pub async fn set_searchable_attributes<T: Debug + Serialize + IntoIterator<Item = impl ToString>>(
        &'a self,
        searchable_attributes: T,
    ) -> Result<Progress<'a>, Error> {
        Ok(request::<T, ProgressJson>(
            &format!(
                "{}/indexes/{}/settings/searchable-attributes",
                self.client.host, self.uid
            ),
            self.client.apikey,
            Method::Post(searchable_attributes),
            202,
        )
        .await?
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
    pub async fn set_displayed_attributes<T: Debug + Serialize + IntoIterator<Item = impl ToString>>(
        &'a self,
        displayed_attributes: T,
    ) -> Result<Progress<'a>, Error> {
        Ok(request::<T, ProgressJson>(
            &format!(
                "{}/indexes/{}/settings/displayed-attributes",
                self.client.host, self.uid
            ),
            self.client.apikey,
            Method::Post(displayed_attributes),
            202,
        )
        .await?
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
        )
        .await?
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
            &format!(
                "{}/indexes/{}/settings/synonyms",
                self.client.host, self.uid
            ),
            self.client.apikey,
            Method::Delete,
            202,
        )
        .await?
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
            &format!(
                "{}/indexes/{}/settings/stop-words",
                self.client.host, self.uid
            ),
            self.client.apikey,
            Method::Delete,
            202,
        )
        .await?
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
            &format!(
                "{}/indexes/{}/settings/ranking-rules",
                self.client.host, self.uid
            ),
            self.client.apikey,
            Method::Delete,
            202,
        )
        .await?
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
            &format!(
                "{}/indexes/{}/settings/attributes-for-faceting",
                self.client.host, self.uid
            ),
            self.client.apikey,
            Method::Delete,
            202,
        )
        .await?
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
            &format!(
                "{}/indexes/{}/settings/distinct-attribute",
                self.client.host, self.uid
            ),
            self.client.apikey,
            Method::Delete,
            202,
        )
        .await?
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
            &format!(
                "{}/indexes/{}/settings/searchable-attributes",
                self.client.host, self.uid
            ),
            self.client.apikey,
            Method::Delete,
            202,
        )
        .await?
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
            &format!(
                "{}/indexes/{}/settings/displayed-attributes",
                self.client.host, self.uid
            ),
            self.client.apikey,
            Method::Delete,
            202,
        )
        .await?
        .into_progress(self))
    }
}
